use std::fmt::Display;

use serde::Deserialize;

use crate::types::{ListStyle, Style};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextElement {
    #[serde(rename = "rich_text_section")]
    Section {
        elements: Vec<RichTextElement>,
    },
    #[serde(rename = "rich_text_quote")]
    Quote {
        elements: Vec<RichTextElement>,
    },
    #[serde(rename = "rich_text_list")]
    List {
        style: ListStyle,
        indent: i64,
        elements: Vec<RichTextElement>,
    },
    #[serde(rename = "rich_text_preformatted")]
    Preformatted {
        elements: Vec<RichTextElement>,
    },
    Emoji {
        name: String,
        style: Option<Style>,
    },
    Text {
        text: String,
        style: Option<Style>,
    },
    Mrkdwn {
        text: String,
        style: Option<Style>,
    },
    Link {
        url: String,
        text: Option<String>,
    },
    User {
        user_id: String,
    },
    Usergroup {
        usergroup_id: String,
    },
    Broadcast {
        range: String,
    },
    Channel {
        channel_id: String,
        style: Option<Style>,
    },
}

impl Display for RichTextElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RichTextElement::Section { elements } => {
                let mut result = String::new();
                for element in elements {
                    result.push_str(&element.to_string());
                }
                result
            }
            RichTextElement::Quote { elements } => {
                let mut result = String::new();
                for element in elements {
                    result.push_str(&format!("> {element}"));
                }
                result
            }
            RichTextElement::List { style, indent, elements } => {
                let mut result = String::new();
                for (n, element) in elements.iter().enumerate() {
                    match style {
                        ListStyle::Bullet => {
                            result.push_str(&"  ".repeat(*indent as usize));
                            result.push_str("- ");
                        }
                        ListStyle::Ordered => {
                            result.push_str(&"   ".repeat(*indent as usize));
                            result.push_str(&format!("{}. ", n + 1));
                        }
                    }
                    result.push_str(&element.to_string());
                    result.push('\n');
                }
                result
            }
            RichTextElement::Preformatted { elements } => {
                let mut result = String::new();
                for element in elements {
                    result.push_str("\n```\n");
                    result.push_str(&element.to_string());
                    result.push_str("\n```\n");
                }
                result
            }
            RichTextElement::Emoji { name, style: _style } => {
                let mut result = String::new();
                result.push(':');
                result.push_str(name);
                result.push(':');
                result
            }
            RichTextElement::Text { text, style } | RichTextElement::Mrkdwn { text, style } => {
                let mut result = String::new();
                match style {
                    Some(Style { code, bold, italic, strike }) => {
                        let (code, bold, italic, strike) = (
                            code.unwrap_or_default(),
                            bold.unwrap_or_default(),
                            italic.unwrap_or_default(),
                            strike.unwrap_or_default(),
                        );
                        result.push_str(
                            &Some(text.to_string())
                                .map(|t| if code { format!("`{t}`") } else { t })
                                .map(|t| if bold { format!("**{t}**") } else { t })
                                .map(|t| if italic { format!("_{t}_") } else { t })
                                .map(|t| if strike { format!("~~{t}~~") } else { t })
                                .unwrap(),
                        );
                    }
                    None => {
                        result.push_str(text);
                    }
                }
                result
            }
            RichTextElement::Link { url, text } => {
                let mut result = String::new();
                match text {
                    Some(t) => {
                        result.push('[');
                        result.push_str(t);
                        result.push_str("](");
                        result.push_str(url);
                        result.push(')');
                    }
                    None => {
                        result.push_str(url);
                    }
                }
                result
            }
            RichTextElement::User { user_id } => {
                let mut result = String::new();
                result.push_str("<@");
                result.push_str(user_id);
                result.push('>');
                result
            }
            RichTextElement::Usergroup { usergroup_id } => {
                let mut result = String::new();
                result.push_str("<!subteam^");
                result.push_str(usergroup_id);
                result.push('>');
                result
            }
            RichTextElement::Broadcast { range } => {
                let mut result = String::new();
                result.push_str("<!");
                result.push_str(range);
                result.push('>');
                result
            }
            RichTextElement::Channel { channel_id, style: _style } => {
                let mut result = String::new();
                result.push_str("<#");
                result.push_str(channel_id);
                result.push('>');
                result
            }
        };
        write!(f, "{str}")
    }
}
