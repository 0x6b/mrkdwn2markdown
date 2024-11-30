use std::{fmt::Display, sync::LazyLock};

use regex::Regex;
use serde::Deserialize;

// https://api.slack.com/reference/block-kit/composition-objects#text
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct TextObject {
    pub r#type: String,
    pub text: String,
    pub emoji: Option<bool>,
    pub verbatim: Option<bool>,
}

static RE_BOLD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*([^*]+)\*").unwrap());
static RE_STRIKE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"~([^~]+)~").unwrap());
static RE_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<([^|]+)\|([^>]+)?>").unwrap());

impl Display for TextObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.r#type == "plain_text" {
            write!(f, "{}", self.text)
        } else {
            let text = self
                .text
                .lines()
                .map(|line| line.trim())
                .collect::<Vec<_>>()
                .join("\n")
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">");
            let text = RE_BOLD.replace_all(&text, "**$1**");
            let text = RE_STRIKE.replace_all(&text, "~~$1~~");
            let mut new_text = String::with_capacity(text.len());
            let mut last = 0;

            for cap in RE_LINK.captures_iter(&text) {
                if let (Some(url), Some(title)) = (cap.get(1), cap.get(2)) {
                    new_text.push_str(&text[last..url.start().saturating_sub(1)]); // remove the `<`
                    new_text.push('[');
                    new_text.push_str(title.as_str());
                    new_text.push_str(r#"]("#);
                    new_text.push_str(url.as_str());
                    new_text.push(')');
                    last = title.end().saturating_add(1); // remove the `>`
                }
            }
            new_text.push_str(&text[last..]);

            write!(f, "{new_text}")
        }
    }
}
