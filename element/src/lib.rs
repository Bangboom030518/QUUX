use std::collections::HashMap;

pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub trait Element {
    // https://internals.rust-lang.org/t/make-associated-consts-object-safe/16357/11
    // https://www.reddit.com/r/rust/comments/7cn3ae/associated_constants_should_survive_in_trait/
    // const TAG_NAME: &'static str;

    fn get_tag_name(&self) -> &'static str;

    fn get_attributes(&self) -> HashMap<String, String>;

    fn get_children(&self) -> &[&dyn Element];

    fn as_string(&self) -> String {
        let attributes: String = self
            .get_attributes()
            .into_iter()
            .map(|(key, value)| format!("{}=\"{}\" ", key, &escape(&value)))
            .collect();
        format!(
            "<{0} {1}>{2}</{0}>",
            self.get_tag_name(),
            attributes,
            self.get_children()
                .iter()
                .map(|element| element.as_string())
                .collect::<String>()
        )
    }
}
