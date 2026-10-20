//! Experimental JSX runner: explicit trust confirmation, limited Illustrator DOM and SVG import.
//! Requires Node.js on PATH. The compatibility bridge is not a JavaScript sandbox.
use serde_json::{Value, json};
use super::DialogSpec;
use crate::state::Dialog;
use crate::{FilePick, VectorcraftApp, io, widgets};

pub const KIND: &str = "jsxRun";
pub(super) const SPEC: DialogSpec = DialogSpec {
    heading: |d| if d.fields["qrPro"].as_bool().unwrap_or(false) { "Smart QR Pro".into() } else { "Executar JSX (experimental)".into() },
    body,
    confirm,
    ok: Some("Executar e abrir SVG"),
    min_width: 440.0,
    max_width: Some(560.0),
    ..DialogSpec::FORM
};

pub(crate) fn command(app: &mut VectorcraftApp, p: &Value) -> Result<Value, String> {
    #[cfg(target_arch = "wasm32")]
    { let _ = (app,p); return Err("Execução JSX disponível apenas na versão desktop".into()); }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = if let Some(path)=p.get("path").and_then(Value::as_str) { path.to_owned() }
          else { crate::picks::open(app, &FilePick { filters:vec![("Script JSX", &["jsx"])], ..Default::default() }).ok_or("cancelled")? };
        if !path.to_lowercase().ends_with(".jsx") { return Err("Selecione um arquivo .jsx".into()); }
        app.ui.dialog = Some(Dialog::new(KIND, json!({"path":path,"data":"https://instagram.com/disneymkt","size":"2","foreground":"#000000","background":"#FFFFFF","margin":"4","trusted":false,"qrPro":false})));
        Ok(Value::Null)
    }
}

/// Open the bundled, reviewed QR script rather than asking users to pick a file.
pub(crate) fn qr_command(app: &mut VectorcraftApp) -> Result<Value, String> {
    #[cfg(target_arch = "wasm32")]
    { let _ = app; Err("Smart QR Pro disponível apenas no desktop".into()) }
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.ui.dialog = Some(Dialog::new(KIND, json!({"path":"__smart_qr_pro__","data":"https://instagram.com/disneymkt","size":"2","foreground":"#000000","background":"#FFFFFF","margin":"4","trusted":true,"qrPro":true})));
        Ok(Value::Null)
    }
}

fn body(_app: &mut VectorcraftApp, ui: &mut egui::Ui, d: &mut Dialog) -> bool {
    let is_qr = d.fields["qrPro"].as_bool().unwrap_or(false);
    if is_qr { ui.label("Crie um QR Code vetorial totalmente editável."); } else {
        ui.label("Compatibilidade parcial com ExtendScript. Testado com o QR.jsx.");
        ui.label(format!("Arquivo: {}", d.str("path")));
    }
    ui.label("Texto ou endereço do QR Code:");
    if let Some(v)=widgets::text_field(ui,"jsx-data",Some(&d.str("data")),430.0,1) {d.fields.insert("data".into(),json!(v));}
    ui.label("Tamanho em polegadas (maior que 0, até 50):");
    if let Some(v)=widgets::text_field(ui,"jsx-size",Some(&d.str("size")),120.0,1) {d.fields.insert("size".into(),json!(v));}
    ui.label("Cor do QR Code (#RRGGBB):");
    if let Some(v)=widgets::text_field(ui,"jsx-foreground",Some(&d.str("foreground")),150.0,1) {d.fields.insert("foreground".into(),json!(v));}
    ui.label("Cor do fundo (#RRGGBB):");
    if let Some(v)=widgets::text_field(ui,"jsx-background",Some(&d.str("background")),150.0,1) {d.fields.insert("background".into(),json!(v));}
    ui.label("Margem de segurança (0 a 16 módulos):");
    if let Some(v)=widgets::text_field(ui,"jsx-margin",Some(&d.str("margin")),100.0,1) {d.fields.insert("margin".into(),json!(v));}
    if !is_qr {
    ui.separator();
    ui.label("ATENÇÃO: scripts podem executar código não confiável. O Node.js deve estar instalado. Execute apenas arquivos cuja origem você conhece.");
    let mut trusted=d.fields["trusted"].as_bool().unwrap_or(false);
    if ui.checkbox(&mut trusted,"Eu confio neste script e autorizo sua execução").changed(){d.fields.insert("trusted".into(),json!(trusted));}
    }
    false
}

fn confirm(app: &mut VectorcraftApp, d: &Dialog) -> Result<Value,String> {
    if !d.fields["trusted"].as_bool().unwrap_or(false) {return Err("Confirme que confia no script antes de executar".into());}
    #[cfg(target_arch = "wasm32")]
    {let _=(app,d); Err("JSX desktop apenas".into())}
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::{fs, process::Command, time::{SystemTime,UNIX_EPOCH}};
        let path=d.str("path");
        let is_qr=d.fields["qrPro"].as_bool().unwrap_or(false);
        let data=d.str("data");
        let size=d.str("size");
        if data.trim().is_empty(){return Err("Informe o texto do QR Code".into());}
        let parsed: f32=size.replace(',', ".").parse().map_err(|_|"Tamanho inválido")?;
        if !(0.0..=50.0).contains(&parsed) || parsed == 0.0 {return Err("Tamanho deve ser maior que zero e até 50".into());}
        let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos();
        let tmp=std::env::temp_dir().join(format!("vectorsmart-jsx-{stamp}"));
        fs::create_dir_all(&tmp).map_err(|e|e.to_string())?;
        let runner=tmp.join("jsx-runner.cjs");let svg=tmp.join("resultado.svg");
        let script_path=if is_qr {
            let embedded=tmp.join("smart-qr.jsx");
            fs::write(&embedded, include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../tools/smart-qr-base.jsx"))).map_err(|e|e.to_string())?;
            embedded
        } else { std::path::PathBuf::from(path) };
        let color_ok=|v:&str| v.len()==7 && v.starts_with('#') && v[1..].bytes().all(|b|b.is_ascii_hexdigit());
        let foreground=d.str("foreground"); let background=d.str("background");
        if !color_ok(&foreground) || !color_ok(&background) {return Err("Informe cores hexadecimais como #112233".into());}
        let margin: u8=d.str("margin").parse().map_err(|_|"Margem inválida")?;
        if margin>16 {return Err("A margem deve ficar entre 0 e 16 módulos".into());}
        fs::write(&runner,include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../tools/jsx-runner.cjs"))).map_err(|e|e.to_string())?;
        let mut command=Command::new("node");
        command.arg(&runner).arg(&script_path).args(["--data",&data,"--size",&parsed.to_string(),"--out"])
            .arg(&svg).args(["--foreground",&foreground,"--background",&background,"--margin",&margin.to_string()]);
        #[cfg(target_os="windows")]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000); // CREATE_NO_WINDOW: do not flash a Node console.
        }
        let output=command.output().map_err(|e|format!("Node.js não encontrado ou falhou: {e}"))?;
        if !output.status.success(){return Err(format!("JSX não suportado ou falhou: {}",String::from_utf8_lossy(&output.stderr)));}
        let bytes=fs::read(&svg).map_err(|e|e.to_string())?;
        let result=io::open_document(app,if is_qr {"Smart QR Pro.svg"} else {"Resultado JSX.svg"},&bytes,None,&Value::Null)?;
        app.status("JSX executado: resultado SVG aberto para edição");
        Ok(result)
    }
}
