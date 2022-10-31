trait Element {
    const TAG_NAME: &'static str;

    fn as_string(&self) -> String;
}

#[derive(Clone)]
pub enum HTMLElement {
    Body(Body),
    HTML(HTML),
    Div(Div)
}

#[derive(Clone)]
pub struct HTML {
    pub children: Vec<HTMLElement>,
}

impl Element for HTML {
    const TAG_NAME: &'static str = "html";

    fn as_string(&self) -> String {
        format!()
    }
}

#[derive(Clone)]
pub struct Body {
    pub children: Vec<HTMLElement>,
}

impl Element for Body {
    const TAG_NAME: &'static str = "body";
}

#[derive(Clone)]
pub struct Div {
    pub children: Vec<HTMLElement>,
}

impl Element for Div {
    const TAG_NAME: &'static str = "div";
}
