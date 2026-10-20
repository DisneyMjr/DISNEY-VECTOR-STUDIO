//! Canva Bridge: safe hand-off to Canva while an OAuth backend is not configured.
//! Never embed a Canva client secret or access token in a desktop binary.
use serde_json::{json, Value};
use super::DialogSpec;
use crate::{VectorcraftApp};
use crate::state::Dialog;

pub const KIND: &str = "canvaBridge";
pub(super) const SPEC: DialogSpec = DialogSpec {
    heading: |_| "Canva Bridge · VectorSmart (Beta)".into(),
    body, confirm,
    ok: Some("Continuar"),
    min_width: 470.0,
    max_width: Some(610.0),
    ..DialogSpec::FORM
};

pub(crate) fn command(app: &mut VectorcraftApp, _p: &Value) -> Result<Value, String> {
    app.ui.dialog = Some(Dialog::new(KIND, json!({"action":"export"})));
    Ok(Value::Null)
}

fn body(_app: &mut VectorcraftApp, ui: &mut egui::Ui, d: &mut Dialog) -> bool {
    ui.heading("Leve suas artes para o Canva");
    ui.label("Nesta etapa, exporte um PDF e importe o arquivo no Canva.");
    ui.label("PDF é um dos formatos aceitos pela importação oficial do Canva. Textos e objetos podem variar após a conversão.");
    ui.separator();
    let mut action = d.str("action");
    ui.radio_value(&mut action, "export".into(), "1 · Exportar a arte atual como PDF");
    ui.radio_value(&mut action, "open".into(), "2 · Abrir Canva para importar o PDF salvo");
    d.fields.insert("action".into(), json!(action));
    ui.separator();
    ui.small("Atenção: isto ainda não vincula sua conta Canva ao VectorSmart. Sincronização automática exige um serviço web com OAuth 2.0 e credenciais armazenadas no servidor.");
    false
}

fn confirm(app: &mut VectorcraftApp, d: &Dialog) -> Result<Value, String> {
    match d.str("action").as_str() {
        "export" => app.run("file.export.pdf", json!({})),
        "open" => {
            app.open_url("https://www.canva.com/");
            app.status("Canva aberto. Importe o PDF salvo no VectorSmart.");
            Ok(json!({"opened":"https://www.canva.com/"}))
        }
        _ => Err("Ação Canva inválida".into()),
    }
}
