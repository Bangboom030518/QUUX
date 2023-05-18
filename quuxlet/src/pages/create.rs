use super::Head;
use crate::components::flashcards::Term;
use quux::{prelude::*, tree::Element};
use std::convert::Infallible;

fn text_input(value: &str, placeholder: &str) -> impl Item {
    input()
        .class("input input-bordered input-primary w-full")
        .attribute("name", format!("{}[]", placeholder.to_lowercase()))
        .attribute("type", "text")
        .attribute("placeholder", placeholder)
        .attribute("value", value)
}

// TODO: add trippy animations
fn term_editor<'a>(
    index: Store<usize>,
    Term { term, definition }: &Term,
    terms: store::List<Term>,
) -> Element<'a, impl Item> {
    fieldset()
        .class("card card-bordered shadow")
        .child(legend().class("badge").text("Card"))
        .child(
            div()
                .class("card-body")
                .child(
                    menu()
                        .class("card-actions justify-between")
                        .child(
                            menu()
                                .class("flex gap-4")
                                .child(
                                    button()
                                        .class("tooltip btn btn-square text-white")
                                        .data_attribute("tip", "Move Left")
                                        .attribute("title", "Move Left")
                                        .attribute("type", "button")
                                        .on("click", event!(|| todo!()))
                                        .raw_html(include_str!("../../assets/left-arrow.svg")),
                                )
                                .child(
                                    button()
                                        .class("tooltip btn btn-square text-white")
                                        .data_attribute("tip", "Move Right")
                                        .attribute("title", "Move Right")
                                        .attribute("type", "button")
                                        .on("click", event!(|| todo!()))
                                        .raw_html(include_str!("../../assets/right-arrow.svg")),
                                ),
                        )
                        .child(
                            button()
                                .class("tooltip btn btn-square text-white")
                                .data_attribute("tip", "Delete")
                                .attribute("title", "Delete")
                                .attribute("type", "button")
                                .on(
                                    "click",
                                    event!({
                                        // let index = index.clone();
                                        move || {
                                            terms.remove(*index.get());
                                        }
                                    }),
                                )
                                .raw_html(include_str!("../../assets/bin.svg")),
                        ),
                )
                .child(text_input(term, "Term"))
                .child(text_input(definition, "Definition")),
        )
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Create;

impl Component for Create {
    fn render(self, _: Context<Self>) -> impl Item {
        let terms = store::List::<Term>::new(vec![Term::new("", ""), Term::new("", "")]);
        html()
            .attribute("lang", "en")
            .component(Head::new("Flashcards - QUUX"))
            .child(
                body().class("p-4 grid content-start").child(h1().text("Create Set")).child(
                    form()
                        .attribute("action", "create")
                        .attribute("method", "POST")
                        .class("grid gap-4 w-full")
                        .child(
                            input()
                                .attribute("type", "text")
                                .attribute("placeholder", "Set Name")
                                .class("input input-bordered input-primary w-full"),
                        )
                        .child(
                            fieldset()
                                .class("grid gap-4 [grid-template-columns:repeat(auto-fit,minmax(50ch,1fr))]")
                                .reactive_many(terms.clone(), {
                                    let terms = terms.clone();
                                    move |index, term| term_editor(index, term, terms.clone())
                                }),
                        )
                        .child(
                            button()
                                .attribute("type", "button")
                                .class("btn btn-primary btn-outline w-full")
                                .text("New Card")
                                .on("click", event!(move || terms.push(Term::default()))),
                        )
                        .child(button().class("btn btn-primary w-full").text("Create")),
                ),
            )
            .component(InitialisationScript::init(include_str!(
                "../../dist/init.js"
            )))
    }
}

impl Create {
    #[server]
    #[must_use]
    pub fn routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Copy
    {
        use http::Uri;
        use warp::Filter;

        warp::path!("create")
            .and(warp::get())
            .map(|| Self)
            .or(warp::post().and(warp::body::form::<PostData>()).and_then({
                |data| async move {
                    println!("{data:?}");
                    Ok::<_, Infallible>(warp::redirect(Uri::from_static("/create")))
                }
            }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PostData {
    terms: Vec<Term>,
}

impl PostData {
    fn from_formdata(value: Vec<(String, String)>) -> Self {
        let mut terms = Vec::new();
        let mut iter = value.into_iter();
        loop {
            let Some((_, term)) = iter.find(|(key, _)| key == "term[]") else {
                break
            };
            let Some((_, definition)) = iter.find(|(key, _)| key == "definition[]") else {
                break
            };
            terms.push(Term::new(&term, &definition));
        }
        Self { terms }
    }
}

impl<'de> Deserialize<'de> for PostData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data: Vec<(String, String)> = Deserialize::deserialize(deserializer)?;
        Ok(Self::from_formdata(data))
    }
}
