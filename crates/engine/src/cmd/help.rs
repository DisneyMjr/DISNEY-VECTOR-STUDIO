//! Help → social link for the app. The command returns the URL; the frontend opens it.

use serde_json::{Value, json};

use super::*;

pub const INSTAGRAM_URL: &str = "https://instagram.com/disneymkt";

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.discord", "Me siga no Insta", ["Help"], None, "{} → {url} Instagram do Disney", always, |_, _| url(INSTAGRAM_URL.into())),
        cmd!(query "help.links", "Links", [], None, "{} → {instagram}", always, |_, _| {
            Ok(json!({ "instagram": INSTAGRAM_URL }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.discord", &json!({})).unwrap()["url"], "https://instagram.com/disneymkt");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["instagram"], "https://instagram.com/disneymkt");
    }
}
