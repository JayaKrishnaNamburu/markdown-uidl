#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
mod structs;

use pulldown_cmark::{Event, Options, Parser, Tag};
use serde_json;
use structs::{create_component_uidl, create_element_node, ComponentUIDL};

fn tags_parser(tag: Tag) -> Option<String> {
    match tag {
        Tag::Heading(heading_level) => {
            return Some("h".to_string() + &heading_level.to_string());
        }
        Tag::Paragraph => return Some("p".to_string()),
        Tag::Strong => {
            return Some("b".to_string());
        }
        _ => {
            return None;
        }
    }
}

fn content_parser(iterator: Parser) -> ComponentUIDL {
    let mut component_uidl = create_component_uidl();

    for event in iterator {
        let &mut element;

        match event {
            Event::Start(tag) => {
                let tag_name = tags_parser(tag);
                if tag_name.is_some() {
                    element = create_element_node("text", &tag_name.unwrap().to_string());
                    println!("{:?}", &element);
                }
            }
            Event::Text(_) => {
                // println!("{:?}", &element);
            }
            Event::SoftBreak => {}
            Event::End(_) => {
                // &component_uidl.node.content.children.push(element);
            }
            _ => {
                // TODO handle other events
            }
        }
    }

    return component_uidl;
}

pub fn parse(input: &str) -> &str {
    println!("Recieved input {}", &input);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&input, options);

    let component_uidl = content_parser(parser);
    let stringify_json = serde_json::to_string(&component_uidl).unwrap();
    println!("{}", stringify_json);

    return "BACK";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(
            parse(
                "
# header
## header
### header
#### header
##### header
###### header
"
            ),
            "OK"
        )
    }
}
