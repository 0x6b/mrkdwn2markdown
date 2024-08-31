#[cfg(test)]
#[test]
fn main() {
    let blocks = r#"{"blocks":[{"type":"rich_text","block_id":"W\/ZWi","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Hello, Markdown "},{"type":"text","text":"mrkdwn","style":{"bold":true}},{"type":"text","text":"! and "},{"type":"text","text":"markdown","style":{"italic":true}},{"type":"text","text":".\n"},{"type":"text","text":"mrkdwn","style":{"code":true}},{"type":"text","text":" is text formatting markup style in "},{"type":"link","url":"https:\/\/slack.com\/","text":"Slack"},{"type":"text","text":".\n\n"}]},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"First"}]},{"type":"rich_text_section","elements":[{"type":"text","text":"Second"}]}],"style":"bullet","indent":0,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Third"}]}],"style":"bullet","indent":1,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Fourth"}]}],"style":"bullet","indent":0,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Fifth"}]}],"style":"bullet","indent":2,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Sixth"}]}],"style":"bullet","indent":1,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Seventh"}]}],"style":"bullet","indent":0,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Ordered list 1"}]}],"style":"ordered","indent":0,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Ordered list 1-1"}]}],"style":"bullet","indent":0,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Ordered list 1-2"}]}],"style":"bullet","indent":1,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Ordered list 2"}]}],"style":"ordered","indent":0,"offset":1,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Ordered list 2-1"}]},{"type":"rich_text_section","elements":[{"type":"text","text":"Ordered list 2-2"}]}],"style":"ordered","indent":1,"border":0},{"type":"rich_text_list","elements":[{"type":"rich_text_section","elements":[{"type":"text","text":"Ordered list 3"}]}],"style":"ordered","indent":0,"offset":2,"border":0},{"type":"rich_text_quote","elements":[{"type":"text","text":"This is blockquote.","style":{"italic":true}}]},{"type":"rich_text_preformatted","elements":[{"type":"text","text":"console.log('Hello, mrkdwn!')"}],"border":0},{"type":"rich_text_section","elements":[{"type":"text","text":"Another paragraph."}]}]}]}"#;
    let message = serde_json::from_str::<mrkdwn2markdown::Blocks>(blocks)
        .unwrap()
        .iter()
        .map(|block| block.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    assert_eq!(
        message,
        r#"Hello, Markdown **mrkdwn**! and _markdown_.
`mrkdwn` is text formatting markup style in [Slack](https://slack.com/).

- First
- Second
  - Third
- Fourth
    - Fifth
  - Sixth
- Seventh
1. Ordered list 1
- Ordered list 1-1
  - Ordered list 1-2
1. Ordered list 2
   1. Ordered list 2-1
   2. Ordered list 2-2
1. Ordered list 3
> _This is blockquote._
```
console.log('Hello, mrkdwn!')
```
Another paragraph."#
    );
}