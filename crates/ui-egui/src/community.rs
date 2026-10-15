//! Social links for the app: the Instagram button (header, About, Home).

use egui::{Color32, CornerRadius, Sense, Ui, vec2};

use crate::theme;
use crate::{VectorcraftApp, icons};

/// Instagram button color used in the custom UI.
const DISCORD: Color32 = Color32::from_rgb(0x58, 0x65, 0xF2);

/// The Discord button's label, height and width.
fn discord_layout(ui: &Ui, large: bool) -> (std::sync::Arc<egui::Galley>, f32, f32) {
    let (h, font, label) = if large { (36.0, theme::semibold(14.0), "Me siga no Insta") } else { (24.0, theme::semibold(12.0), "Insta") };
    let galley = ui.painter().layout_no_wrap(label.to_string(), font, Color32::WHITE);
    let w = galley.size().x + h * 0.55 + h * 0.9;
    (galley, h, w)
}

/// The Discord button's width (the app bar drops it when space runs out).
pub fn discord_width(ui: &Ui, large: bool) -> f32 {
    discord_layout(ui, large).2
}

/// A Discord pill button: compact for the header, large for About / Home.
pub fn discord_button(app: &mut VectorcraftApp, ui: &mut Ui, large: bool) -> egui::Response {
    let (galley, h, w) = discord_layout(ui, large);
    let icon = h * 0.55;
    let (r, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
    let fill = if resp.hovered() { DISCORD.gamma_multiply(0.85) } else { DISCORD };
    ui.painter().rect_filled(r, CornerRadius::same((h / 2.0) as u8), fill);
    let ir = egui::Rect::from_center_size(r.left_center() + vec2(h * 0.35 + icon / 2.0, 0.0), vec2(icon, icon));
    icons::paint(ui, "message-circle", ir, Color32::WHITE);
    ui.painter().galley(egui::pos2(ir.right() + h * 0.2, r.center().y - galley.size().y / 2.0), galley, Color32::WHITE);
    let resp = resp.on_hover_text("instagram.com/disneymkt");
    if resp.clicked() {
        app.open_link("help.discord");
    }
    resp
}

/// Social area for the app. Only the Instagram button is shown.
pub fn links(app: &mut VectorcraftApp, ui: &mut Ui) {
    discord_button(app, ui, true);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_clicks_open_the_right_urls() {
        use std::sync::{Arc, Mutex};
        let opened = Arc::new(Mutex::new(vec![]));
        let o = opened.clone();
        let services = crate::Services { open_url: Some(Box::new(move |u: &str| o.lock().unwrap().push(u.to_string()))), ..Default::default() };
        let mut app = VectorcraftApp::new(vectorcraft_engine::Session::new(), services);
        for id in ["help.discord"] {
            crate::menus::invoke(&mut app, id, serde_json::json!({}));
        }
        assert_eq!(
            *opened.lock().unwrap(),
            ["https://instagram.com/disneymkt"]
        );
        // The control channel / MCP path returns the URL without opening a browser.
        let v = app.run("help.discord", serde_json::json!({})).unwrap();
        assert_eq!(v["url"], "https://instagram.com/disneymkt");
        assert_eq!(opened.lock().unwrap().len(), 1);
        let ctx = egui::Context::default();
        theme::install_fonts(&ctx);
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| links(&mut app, ui));
        out.textures_delta.clear();
    }
}
