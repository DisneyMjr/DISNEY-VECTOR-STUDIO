//! Smart Social: editable SVG social posts generated from CSV rows.
//! CSV columns: titulo, subtitulo, chamada, cor_fundo, cor_texto.
use serde_json::{json, Value};
use super::DialogSpec;
use crate::{VectorcraftApp, io, widgets};
use crate::state::Dialog;

pub const KIND: &str = "smartSocial";
pub(super) const SPEC: DialogSpec = DialogSpec {
    heading: |_| "Smart Social · Artes em lote".into(),
    body, confirm,
    ok: Some("Gerar artes vetoriais"),
    min_width: 480.0,
    max_width: Some(650.0),
    ..DialogSpec::FORM
};

pub(crate) fn command(app: &mut VectorcraftApp, p: &Value) -> Result<Value,String> {
    let initial = "titulo,subtitulo,chamada,cor_fundo,cor_texto\nNovidade DisneyMkt,Confira as novidades,Saiba mais,#14213D,#FFFFFF\nSmart Social,Posts em lote,Conheça agora,#683ACF,#FFFFFF";
    app.ui.dialog = Some(Dialog::new(KIND,json!({
        "csv": p.get("csv").and_then(Value::as_str).unwrap_or(initial),
        "format": p.get("format").and_then(Value::as_str).unwrap_or("Post 1080x1440"),
        "title": "Smart Social", "limit": "50"
    })));
    Ok(Value::Null)
}

fn body(_app: &mut VectorcraftApp, ui: &mut egui::Ui, d: &mut Dialog) -> bool {
    ui.label("Cole dados CSV: uma arte para cada linha. Colunas: titulo, subtitulo, chamada, cor_fundo, cor_texto.");
    ui.label("CSV com vírgulas, aspas e quebra de linha entre os registros; exporte com codificação UTF-8.");
    if let Some(v) = widgets::text_field(ui,"social-csv",Some(&d.str("csv")),565.0,10) {
        d.fields.insert("csv".into(),json!(v));
    }
    ui.separator();
    ui.label("Formato:");
    let mut fmt=d.str("format");
    egui::ComboBox::from_id_salt("social-format").selected_text(&fmt).show_ui(ui,|ui| {
        for choice in ["Post 1080x1440","Quadrado 1080x1080","Story 1080x1920"] {
            ui.selectable_value(&mut fmt,choice.to_string(),choice);
        }
    });
    d.fields.insert("format".into(),json!(fmt));
    ui.label("Limite de artes por operação (1 a 200):");
    if let Some(v)=widgets::text_field(ui,"social-limit",Some(&d.str("limit")),90.0,1) {
        d.fields.insert("limit".into(),json!(v));
    }
    ui.label("Cada registro será aberto como um SVG separado e editável no VectorSmart.");
    false
}

fn confirm(app: &mut VectorcraftApp, d: &Dialog) -> Result<Value,String> {
    let max: usize = d.str("limit").trim().parse().map_err(|_|"Limite inválido")?;
    if !(1..=200).contains(&max) {return Err("O limite deve ficar entre 1 e 200".into());}
    let (width,height) = match d.str("format").as_str() {
        "Post 1080x1440" => (1080,1440),
        "Quadrado 1080x1080" => (1080,1080),
        "Story 1080x1920" => (1080,1920),
        _ => return Err("Formato desconhecido".into())
    };
    let rows = parse_csv(&d.str("csv"))?;
    if rows.len()<2 {return Err("Insira o cabeçalho e ao menos uma linha de dados".into());}
    let headers: Vec<String>=rows[0].iter().map(|s|s.trim().trim_start_matches('\u{feff}').to_lowercase()).collect();
    if !headers.iter().any(|h|h=="titulo") {return Err("O CSV precisa de uma coluna 'titulo'".into());}
    if rows.len()-1 > max {return Err(format!("CSV tem {} artes, acima do limite de {}",rows.len()-1,max));}
    // Build and validate ALL documents before opening any to avoid partial imports for bad input.
    let mut documents=Vec::new();
    for (index, row) in rows.iter().skip(1).enumerate() {
        if row.len()!=headers.len() {return Err(format!("Linha {}: esperado {} colunas, recebidas {}",index+2,headers.len(),row.len()));}
        let val=|name:&str|->&str {headers.iter().position(|h|h==name).map(|i|row[i].as_str()).unwrap_or("")};
        let title=val("titulo");
        if title.trim().is_empty() {return Err(format!("Linha {}: título vazio",index+2));}
        let bg=if val("cor_fundo").trim().is_empty(){"#14213D"}else{val("cor_fundo").trim()};
        let fg=if val("cor_texto").trim().is_empty(){"#FFFFFF"}else{val("cor_texto").trim()};
        for color in [bg,fg] {if !valid_color(color) {return Err(format!("Linha {}: cor hexadecimal inválida: {}",index+2,color));}}
        let svg=render_svg(width,height,title,val("subtitulo"),val("chamada"),bg,fg);
        documents.push((format!("Smart Social {:03}.svg",index+1),svg.into_bytes()));
    }
    let count=documents.len();
    for (name,bytes) in documents {
        io::open_document(app,&name,&bytes,None,&Value::Null)?;
    }
    app.status(format!("Smart Social: {} arte(s) SVG geradas",count));
    Ok(json!({"generated":count,"format":d.str("format")}))
}

fn valid_color(s:&str)->bool {s.len()==7 && s.starts_with('#') && s[1..].bytes().all(|b|b.is_ascii_hexdigit())}
fn esc(s:&str)->String {s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;").replace('\'',"&apos;")}
fn text_lines(value:&str, max_chars:usize, max_lines:usize) -> Vec<String> {
    let mut lines=Vec::new();let mut curr=String::new();
    for word in value.split_whitespace() {
        if word.chars().count()>max_chars {
            if !curr.is_empty() {lines.push(std::mem::take(&mut curr));}
            let mut chunk=String::new();
            for ch in word.chars() {chunk.push(ch);if chunk.chars().count()==max_chars {lines.push(std::mem::take(&mut chunk));}}
            curr=chunk;
        } else if curr.is_empty() {curr=word.to_string();}
        else if curr.chars().count()+word.chars().count()+1<=max_chars {curr.push(' ');curr.push_str(word);}
        else {lines.push(std::mem::take(&mut curr));curr=word.to_string();}
    }
    if !curr.is_empty(){lines.push(curr);}
    lines.truncate(max_lines);lines
}
fn svg_multiline(value:&str,x:u32,y:u32,size:u32,spacing:u32,max_chars:usize,max_lines:usize,color:&str,weight:&str)->String {
    let mut out=String::new();
    for (i,line) in text_lines(value,max_chars,max_lines).iter().enumerate() {
        out.push_str(&format!("<text x=\"{x}\" y=\"{}\" fill=\"{color}\" font-family=\"Arial, sans-serif\" font-weight=\"{weight}\" font-size=\"{size}\">{}</text>\n",y+i as u32*spacing,esc(line)));
    }out
}
fn render_svg(w:u32,h:u32,title:&str,subtitle:&str,cta:&str,bg:&str,fg:&str)->String {
    let hero_y=(h as f32*0.40) as u32;
    let subtitle_y=(h as f32*0.66) as u32;
    let cta_y=h-160;
    let mut out=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">\n<rect width=\"{w}\" height=\"{h}\" fill=\"{bg}\"/>\n<rect x=\"80\" y=\"100\" width=\"120\" height=\"12\" rx=\"6\" fill=\"{fg}\"/>\n");
    out.push_str(&svg_multiline(title,80,hero_y,86,101,19,4,fg,"bold"));
    out.push_str(&svg_multiline(subtitle,80,subtitle_y,43,56,35,3,fg,"normal"));
    if !cta.trim().is_empty(){
        out.push_str(&format!("<rect x=\"80\" y=\"{}\" width=\"760\" height=\"92\" rx=\"18\" fill=\"{fg}\"/>\n",cta_y-68));
        out.push_str(&svg_multiline(cta,112,cta_y,39,44,30,1,bg,"bold"));
    }
    out.push_str("</svg>");out
}

// Lightweight RFC4180-style parser: quoted commas, escaped double-quotes and CRLF.
// Disallow embedded newlines in quoted fields to keep row diagnostics predictable.
fn parse_csv(csv:&str)->Result<Vec<Vec<String>>,String>{
    let mut result=Vec::new();let mut row=Vec::new();let mut field=String::new();
    let mut quoted=false;let mut iter=csv.chars().peekable();
    while let Some(ch)=iter.next(){
        match ch {
            '"' if quoted && iter.peek()==Some(&'"')=>{field.push('"');iter.next();},
            '"' if quoted=>quoted=false,
            '"' if field.is_empty()=>quoted=true,
            '"'=>return Err("Aspas inválidas no CSV".into()),
            ',' if !quoted=>{row.push(std::mem::take(&mut field));},
            '\n' if !quoted=>{row.push(std::mem::take(&mut field));if row.iter().any(|x|!x.trim().is_empty()){result.push(std::mem::take(&mut row));}else{row.clear();}},
            '\r' if !quoted=>{},
            '\n' | '\r' if quoted=>return Err("Quebras de linha dentro de uma célula não são suportadas".into()),
            _=>field.push(ch),
        }
    }
    if quoted {return Err("Aspas sem fechamento no CSV".into());}
    row.push(field);if row.iter().any(|x|!x.trim().is_empty()){result.push(row);}
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn csv_quotes_and_unicode() {let rows=parse_csv("titulo,subtitulo\n\"Olá, mundo\",\"Texto \"\"bom\"\"\"\n").unwrap();assert_eq!(rows[1][0],"Olá, mundo");assert_eq!(rows[1][1],"Texto \"bom\"");}
    #[test] fn svg_escaping() {let svg=render_svg(1080,1440,"A & <B>","Teste","Leia", "#123456","#ffffff");assert!(svg.contains("A &amp; &lt;B&gt;"));}
}
