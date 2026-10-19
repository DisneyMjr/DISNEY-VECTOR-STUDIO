//! Experimental JSX runner: explicit trust confirmation, limited Illustrator DOM and SVG import.
//! Requires Node.js on PATH. The compatibility bridge is not a JavaScript sandbox.
use serde_json::{Value, json};
use super::DialogSpec;
use crate::state::Dialog;
use crate::{FilePick, VectorcraftApp, io, widgets};

pub const KIND: &str = "jsxRun";
pub(super) const SPEC: DialogSpec = DialogSpec {
    heading: |_| "Executar JSX (experimental)".into(),
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
        app.ui.dialog = Some(Dialog::new(KIND, json!({"path":path,"data":"https://instagram.com/disneymkt","size":"2","trusted":false})));
        Ok(Value::Null)
    }
}

fn body(_app: &mut VectorcraftApp, ui: &mut egui::Ui, d: &mut Dialog) -> bool {
    ui.label("Compatibilidade parcial com ExtendScript. Testado com o QR.jsx.");
    ui.label(format!("Arquivo: {}", d.str("path")));
    ui.label("Texto ou endereço do QR Code:");
    if let Some(v)=widgets::text_field(ui,"jsx-data",Some(&d.str("data")),430.0,1) {d.fields.insert("data".into(),json!(v));}
    ui.label("Tamanho em polegadas (0 a 50):");
    if let Some(v)=widgets::text_field(ui,"jsx-size",Some(&d.str("size")),120.0,1) {d.fields.insert("size".into(),json!(v));}
    ui.separator();
    ui.label("ATENÇÃO: scripts podem executar código não confiável. O Node.js deve estar instalado. Execute apenas arquivos cuja origem você conhece.");
    let mut trusted=d.fields["trusted"].as_bool().unwrap_or(false);
    if ui.checkbox(&mut trusted,"Eu confio neste script e autorizo sua execução").changed(){d.fields.insert("trusted".into(),json!(trusted));}
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
        let data=d.str("data");
        let size=d.str("size");
        if data.trim().is_empty(){return Err("Informe o texto do QR Code".into());}
        let parsed: f32=size.parse().map_err(|_|"Tamanho inválido")?;
        if !(0.0..=50.0).contains(&parsed) || parsed == 0.0 {return Err("Tamanho deve ser maior que zero e até 50".into());}
        let stamp=SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos();
        let tmp=std::env::temp_dir().join(format!("vectorsmart-jsx-{stamp}"));
        fs::create_dir_all(&tmp).map_err(|e|e.to_string())?;
        let runner=tmp.join("jsx-runner.cjs");let svg=tmp.join("resultado.svg");
        fs::write(&runner,include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../tools/jsx-runner.cjs"))).map_err(|e|e.to_string())?;
        let output=Command::new("node").arg(&runner).arg(&path).args(["--data",&data,"--size",&size,"--out"]).arg(&svg)
            .output().map_err(|e|format!("Node.js não encontrado ou falhou: {e}"))?;
        if !output.status.success(){return Err(format!("JSX não suportado ou falhou: {}",String::from_utf8_lossy(&output.stderr)));}
        let bytes=fs::read(&svg).map_err(|e|e.to_string())?;
        let result=io::open_document(app,"Resultado JSX.svg",&bytes,None,&Value::Null)?;
        app.status("JSX executado: resultado SVG aberto para edição");
        Ok(result)
    }
}
