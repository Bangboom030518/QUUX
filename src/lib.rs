use element::{escape, Element};
use std::collections::HashMap;

macro_rules! element {
    ($name:ident, $tag_name:expr) => {
        #[derive(Clone, Default)]
        pub struct $name<'a> {
            pub children: Vec<&'a dyn element::Element>,
            pub attributes: std::collections::HashMap<String, String>,
        }

        impl<'a> element::Element for $name<'a> {
            fn get_tag_name(&self) -> &'static str {
                $tag_name
            }

            fn get_attributes(&self) -> std::collections::HashMap<String, String> {
                self.attributes.clone()
            }

            fn get_children(&self) -> &[&dyn element::Element] {
                &self.children
            }
        }

        impl<'a> $name<'a> {
            pub fn new_with_children(children: &'a [&dyn element::Element]) -> Self {
                Self {
                    children: children.to_vec(),
                    ..Default::default()
                }
            }
        }
    };
}

element!(Html, "html");
element!(Body, "body");
element!(Head, "head");
element!(Paragraph, "p");

pub struct Text(pub String);

impl Element for Text {
    fn get_tag_name(&self) -> &'static str {
        "#text"
    }

    fn get_children(&self) -> &[&dyn Element] {
        &[]
    }

    fn get_attributes(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    fn as_string(&self) -> String {
        escape(&self.0)
    }
}
