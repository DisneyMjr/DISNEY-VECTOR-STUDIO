//! Smart AI Studio: local prompt-assisted vector composition.
//! This is an offline, deterministic assistant, not a cloud generative model.
use serde_json::{json, Value};
use super::DialogSpec;
use crate::{VectorcraftApp, io, widgets};
use crate::state::Dialog;

pub const KIND: &str = "smartAiStudio";
pub(super) const SPEC: DialogSpec = DialogSpec {
    heading: |_| "Smart AI · Estúdio Vetorial (Beta)".into(),
    body, confirm,
    ok: Some("Criar SVG editável"),
    min_width: 490.0,
    max_width: Some(670.0),
    ..DialogSpec::FORM
};

pub(crate) fn command(app: &mut VectorcraftApp, p: &Value) -> Result<Value,String> {
    app.ui.dialog = Some(Dialog::new(KIND,json!({
        "prompt":p.get("prompt").and_then(Value::as_str).unwrap_or("Criar um cartaz moderno para uma campanha de marketing"),
        "heading":"IDEIAS QUE TRANSFORMAM",
        "subtitle":"Design inteligente · DisneyMkt",
        "format":"Post 1080x1440",
        "style":"Geométrico",
        "palette":"Azul e lilás",
        "accent":"#8B5CF6"
    })));
    Ok(Value::Null)
}

fn body(_app: &mut VectorcraftApp,ui: &mut egui::Ui,d: &mut Dialog) -> bool {
    ui.label("Descreva a arte desejada em português:");
    if let Some(v)=widgets::text_field(ui,"ai-prompt",Some(&d.str("prompt")),580.0,4) {d.fields.insert("prompt".into(),json!(v));}
    ui.label("Título editável:");
    if let Some(v)=widgets::text_field(ui,"ai-heading",Some(&d.str("heading")),580.0,2) {d.fields.insert("heading".into(),json!(v));}
    ui.label("Subtítulo editável:");
    if let Some(v)=widgets::text_field(ui,"ai-subtitle",Some(&d.str("subtitle")),580.0,2) {d.fields.insert("subtitle".into(),json!(v));}
    for (key,label,choices) in [
        ("format","Tamanho",&["Post 1080x1440","Quadrado 1080x1080","Story 1080x1920","Logo 1024x1024"][..]),
        ("style","Composição",&["Geométrico","Minimalista","Gradiente","Selo vetorial"][..]),
        ("palette","Paleta",&["Azul e lilás","Verde e creme","Preto e dourado","Coral e azul"][..]),
    ] {
        ui.label(label);
        let mut selected=d.str(key);
        egui::ComboBox::from_id_salt(format!("smart-ai-{key}")).selected_text(&selected).show_ui(ui,|ui| {
            for choice in choices {ui.selectable_value(&mut selected,(*choice).to_string(),*choice);}
        });
        d.fields.insert(key.into(),json!(selected));
    }
    ui.label("Cor de destaque personalizada (#RRGGBB):");
    if let Some(v)=widgets::text_field(ui,"ai-accent",Some(&d.str("accent")),170.0,1){d.fields.insert("accent".into(),json!(v));}
    ui.separator();
    ui.small("Beta local: combina regras de composição e palavras da descrição. Não utiliza modelo generativo nem envia dados pela internet. O SVG pode ser editado normalmente.");
    false
}

fn confirm(app:&mut VectorcraftApp,d:&Dialog)->Result<Value,String>{
    let prompt=d.str("prompt");
    if prompt.trim().is_empty(){return Err("Descreva o que deseja criar".into());}
    if prompt.chars().count()>2000{return Err("Descrição longa demais (máximo: 2000 caracteres)".into());}
    let heading=d.str("heading");
    if heading.trim().is_empty(){return Err("Informe um título".into());}
    if heading.chars().count()>180{return Err("Título muito longo".into());}
    let accent=d.str("accent");
    if !valid_color(&accent){return Err("Cor inválida. Use #RRGGBB".into());}
    let (w,h) = match d.str("format").as_str(){
        "Post 1080x1440" => (1080,1440),
        "Quadrado 1080x1080" => (1080,1080),
        "Story 1080x1920" => (1080,1920),
        "Logo 1024x1024" => (1024,1024),
        _=>return Err("Formato não reconhecido".into()),
    };
    let svg=render_svg(w,h,&prompt,&heading,&d.str("subtitle"),&d.str("style"),&d.str("palette"),&accent);
    let result=io::open_document(app,"Smart AI - Composição.svg",svg.as_bytes(),None,&Value::Null)?;
    app.status("Smart AI: composição vetorial local gerada. Objetos editáveis.");
    Ok(result)
}

fn valid_color(s:&str)->bool{s.len()==7&&s.starts_with('#')&&s[1..].bytes().all(|b|b.is_ascii_hexdigit())}
fn esc(s:&str)->String{s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;").replace('\'',"&apos;")}
fn wrap(s:&str,limit:usize,max_lines:usize)->Vec<String>{
    let mut lines=Vec::new();let mut current=String::new();
    for word in s.split_whitespace(){
        if current.is_empty(){current=word.to_string();}
        else if current.chars().count()+word.chars().count()+1<=limit {current.push(' ');current.push_str(word);}
        else {lines.push(std::mem::take(&mut current));current=word.to_string();}
        if lines.len()>=max_lines{break;}
    }
    if lines.len()<max_lines&&!current.is_empty(){lines.push(current);}
    lines
}
fn render_svg(w:u32,h:u32,prompt:&str,title:&str,subtitle:&str,style:&str,palette:&str,accent:&str)->String{
    let (bg,fg,secondary)=match palette {
        "Verde e creme"=>("#F7F7EB","#154734","#86EFAC"),
        "Preto e dourado"=>("#111827","#FFF5D6","#EABF67"),
        "Coral e azul"=>("#0C2549","#FFFFFF","#FB7185"),
        _=>("#111C44","#FFFFFF","#A78BFA"),
    };
    let wf=w as f64;let hf=h as f64;
    let mut s=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">\n<title>Smart AI - {}</title><desc>{}</desc>\n",esc(title),esc(prompt));
    s.push_str(&format!("<rect id=\"fundo\" width=\"{w}\" height=\"{h}\" fill=\"{bg}\"/>\n"));
    match style {
        "Minimalista" => {
            s.push_str(&format!("<rect id=\"barra-destaque\" x=\"80\" y=\"100\" width=\"260\" height=\"16\" rx=\"8\" fill=\"{accent}\"/>\n"));
            s.push_str(&format!("<circle id=\"elemento-circular\" cx=\"{:.0}\" cy=\"{:.0}\" r=\"{:.0}\" fill=\"none\" stroke=\"{secondary}\" stroke-width=\"14\"/>\n",wf*.78,hf*.28,wf*.15));
        },
        "Gradiente" => {
            s.push_str(&format!("<defs><linearGradient id=\"smart-grad\" x1=\"0\" x2=\"1\" y1=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"{accent}\"/><stop offset=\"1\" stop-color=\"{secondary}\"/></linearGradient></defs>\n"));
            s.push_str(&format!("<circle id=\"circulo-gradiente\" cx=\"{:.0}\" cy=\"{:.0}\" r=\"{:.0}\" fill=\"url(#smart-grad)\"/>\n",wf*.8,hf*.27,wf*.27));
        },
        "Selo vetorial" => {
            s.push_str(&format!("<circle id=\"selo-externo\" cx=\"{:.0}\" cy=\"{:.0}\" r=\"{:.0}\" fill=\"{accent}\" stroke=\"{secondary}\" stroke-width=\"14\"/>\n",wf*.5,hf*.35,wf*.28));
            s.push_str(&format!("<circle id=\"selo-interno\" cx=\"{:.0}\" cy=\"{:.0}\" r=\"{:.0}\" fill=\"none\" stroke=\"{fg}\" stroke-width=\"7\"/>\n",wf*.5,hf*.35,wf*.235));
        },
        _ => {
            s.push_str(&format!("<circle id=\"circulo\" cx=\"{:.0}\" cy=\"{:.0}\" r=\"{:.0}\" fill=\"{accent}\"/>\n",wf*.83,hf*.27,wf*.28));
            s.push_str(&format!("<rect id=\"quadrado\" x=\"{:.0}\" y=\"{:.0}\" width=\"{:.0}\" height=\"{:.0}\" rx=\"36\" transform=\"rotate(-22 {:.0} {:.0})\" fill=\"{secondary}\" opacity=\"0.8\"/>\n",wf*.53,hf*.14,wf*.19,wf*.19,wf*.63,hf*.24));
        },
    }
    let title_y=if style=="Selo vetorial"{hf*.69}else{hf*.55};
    let title_size=if w==1024{69}else{79};
    for (i,line) in wrap(title,18,4).iter().enumerate(){s.push_str(&format!("<text id=\"titulo-{i}\" x=\"80\" y=\"{:.0}\" fill=\"{fg}\" font-family=\"Arial,sans-serif\" font-weight=\"700\" font-size=\"{title_size}\">{}</text>\n",title_y+(i as f64*91.),esc(line)));}
    let sub_y=(hf*.86).min(hf-92.);
    for (i,line) in wrap(subtitle,38,2).iter().enumerate(){s.push_str(&format!("<text id=\"subtitulo-{i}\" x=\"80\" y=\"{:.0}\" fill=\"{fg}\" font-family=\"Arial,sans-serif\" font-size=\"34\">{}</text>\n",sub_y+(i as f64*44.),esc(line)));}
    s.push_str("</svg>");s
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn escapes_user_text(){let svg=render_svg(1080,1440,"<script>","A & <B>","@teste","Geométrico","Azul e lilás","#123456");assert!(svg.contains("A &amp; &lt;B&gt;"));assert!(!svg.contains("<script>"));}
    #[test] fn valid_hex(){assert!(valid_color("#ABCDEF"));assert!(!valid_color("red"));}
}
