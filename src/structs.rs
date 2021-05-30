#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UIDLStaticNode {
    pub r#type: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UIDLElement {
    pub element_type: String,
    pub semantic_type: String,
    pub children: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Node {
    ElementNode(UIDLElementNode),
    StaticNode(UIDLStaticNode),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UIDLElementNode {
    pub r#type: String,
    pub content: UIDLElement,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentUIDL {
    pub name: String,
    pub node: UIDLElementNode,
}

pub fn create_component_uidl() -> ComponentUIDL {
    let children: Vec<Node> = Vec::new();
    let parent_element_node = UIDLElementNode {
        r#type: "element".to_string(),
        content: UIDLElement {
            element_type: "container".to_string(),
            semantic_type: "div".to_string(),
            children: children,
        },
    };

    let uidl = ComponentUIDL {
        name: "App".to_string(),
        node: parent_element_node,
    };

    return uidl;
}

pub fn create_element_node(element_type: &str, semantic_type: &str) -> Node {
    let children: Vec<Node> = Vec::new();
    let element = Node::ElementNode(UIDLElementNode {
        r#type: "element".to_string(),
        content: UIDLElement {
            element_type: element_type.to_string(),
            semantic_type: semantic_type.to_string(),
            children: children,
        },
    });
    return element;
}