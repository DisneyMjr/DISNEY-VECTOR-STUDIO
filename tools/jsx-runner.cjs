#!/usr/bin/env node
// VectorSmart experimental JSX bridge. Restricted, non-browser JavaScript execution.
// Usage: node tools/jsx-runner.cjs QR.jsx --data 'https://example.com' --size 2 --out qr.svg
'use strict';
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');
const args = process.argv.slice(2);
const input = args.shift();
function arg(flag, fallback) { const i=args.indexOf(flag); return i >= 0 ? args[i+1] : fallback; }
if (!input || !input.toLowerCase().endsWith('.jsx')) { console.error('Uso: node tools/jsx-runner.cjs arquivo.jsx --data TEXTO --size 2 --out saida.svg'); process.exit(2); }
const qrData = arg('--data', '');
const size = Number(arg('--size', '2'));
const output = arg('--out', path.basename(input,'.jsx')+'.svg');
const foreground=arg('--foreground','#000000');
const background=arg('--background','#FFFFFF');
const margin=Number(arg('--margin','4'));
const validHex=/^#[0-9a-fA-F]{6}$/;
if (!validHex.test(foreground)||!validHex.test(background)||!Number.isInteger(margin)||margin<0||margin>16) {console.error('Cores ou margem inválidas');process.exit(2);} 
if (!qrData || !(size>0 && size<=50)) { console.error('Informe --data e --size (em polegadas, entre 0 e 50).'); process.exit(2); }
const source=fs.readFileSync(input,'utf8');
if (source.length > 4_000_000) throw Error('Script muito grande');
const documents=[];
function documentNew(space, width=144, height=144) {
  const doc={width,height,paths:[],pathItems: {add(){const obj={points:[],stroked:false,filled:true,fillColor:null,closed:false,setEntirePath(points){this.points=points}};doc.paths.push(obj);return obj;}}};
  documents.push(doc);return doc;
}
const dataGroup={qrData:{text:qrData}}; const sizeGroup={qrSize:{text:String(size)}};
const alerts=[];
function Window(spec){
  if (typeof spec!=='string' || !spec.includes('qrData') || !spec.includes('qrSize')) throw Error('ScriptUI não suportado neste script');
  return {dataGroup,sizeGroup,btnGroup:{okBtn:{onClick:null},cancelBtn:{}},close(){},show(){if (typeof this.btnGroup.okBtn.onClick==='function')this.btnGroup.okBtn.onClick();}};
}
function GrayColor(){this.gray=0;}
const sandbox=Object.create(null);
Object.assign(sandbox,{GrayColor,Window,DocumentColorSpace:{CMYK:'CMYK',RGB:'RGB'},app:{documents:{add:documentNew}},alert:(x)=>alerts.push(String(x))});
// No Node globals are deliberately exposed, but vm is NOT a security boundary. Run trusted scripts only.
const context=vm.createContext(sandbox,{codeGeneration:{strings:false,wasm:false}});
vm.runInContext(source,context,{timeout:10000,filename:path.basename(input)});
if(!documents.length) throw Error('Nenhum documento gerado. Este script pode usar recursos Illustrator ainda não implementados.');
const doc=documents.at(-1), escape=(s)=>String(s).replace(/&/g,'&amp;').replace(/"/g,'&quot;');
const paths=doc.paths.map(p=>{
  const pts=p.points||[]; if(!pts.length)return '';
  const d=pts.map((pt,i)=>`${i?'L':'M'} ${Number(pt[0]).toFixed(4)} ${(doc.height-Number(pt[1])).toFixed(4)}`).join(' ')+(p.closed?' Z':'');
  const g=p.fillColor && Number.isFinite(p.fillColor.gray)?p.fillColor.gray:0;
  const gray=Math.round(Math.max(0,Math.min(100,g))*255/100).toString(16).padStart(2,'0');
  return `<path d="${escape(d)}" fill="#${gray}${gray}${gray}" stroke="none"/>`;
});
const moduleWidth=doc.width/((sandbox.qr && sandbox.qr.modules && sandbox.qr.modules.length)||33);
const pad=margin*moduleWidth;
const totalW=doc.width+2*pad,totalH=doc.height+2*pad;
const colored=paths.join('\n').replace(/fill="#000000"/gi,`fill="${foreground}"`).replace(/fill="#ffffff"/gi,`fill="${background}"`);
const svg=`<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" width="${totalW}pt" height="${totalH}pt" viewBox="0 0 ${totalW} ${totalH}">\n<rect width="${totalW}" height="${totalH}" fill="${background}"/>\n<g transform="translate(${pad},${pad})">${colored}</g>\n</svg>\n`;
fs.writeFileSync(output,svg,'utf8');
console.log(`VectorSmart JSX (experimental): ${paths.length} vetores salvos em ${output}`);
