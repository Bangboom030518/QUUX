// use html::html;

use element::Element;
use quux::{Body, Head, Html, Paragraph, Text};

fn build() -> String {
    vec![&Html {
        children: vec![
            &Head::default(),
            &Body::new_with_children(&[&Paragraph::new_with_children(&[&Text(
                "HELLO!".to_string(),
            )])]),
        ],
        ..Default::default()
    }]
    .into_iter()
    .map(Element::as_string)
    .collect()
}

struct MyComponent;

impl Component for MyComponent {
    
}


fn main() {
    println!("{}", build())
}
