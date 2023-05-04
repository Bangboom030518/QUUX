use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use super::Head;
use crate::components::flashcards::Term;
use quux::{prelude::*, tree::Element};

fn text_input(id: &str, value: &str, placeholder: &str) -> impl Item {
    input()
        .class("input input-bordered input-primary w-full")
        .id(id)
        .attribute("type", "text")
        .attribute("placeholder", placeholder)
        .attribute("value", value)
}
// legend {
//     display: block;
//     padding-inline-start: 2px;
//     padding-inline-end: 2px;
//     border-width: initial;
//     border-style: none;
//     border-color: initial;
//     border-image: initial;
// }

// TODO: add trippy animations
fn term_editor<'a>(Term { term, definition }: Term) -> Element<'a, impl Item> {
    static INDEX: AtomicUsize = AtomicUsize::new(0);
    let index = INDEX.fetch_add(1, Relaxed);
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
                                        .text(include_str!("../../assets/left-arrow.svg")),
                                )
                                .child(
                                    button()
                                        .class("tooltip btn btn-square text-white")
                                        .data_attribute("tip", "Move Right")
                                        .attribute("title", "Move Right")
                                        .attribute("type", "button")
                                        .on("click", event!(|| todo!()))
                                        .text(include_str!("../../assets/right-arrow.svg")),
                                ),
                        )
                        .child(
                            button()
                                .class("tooltip btn btn-square text-white")
                                .data_attribute("tip", "Delete")
                                .attribute("title", "Delete")
                                .attribute("type", "button")
                                .on("click", event!(|| todo!()))
                                .text(include_str!("../../assets/bin.svg")),
                        ),
                )
                .child(text_input(&format!("new-card-term-{index}"), &term, "Term"))
                .child(text_input(
                    &format!("new-card-definition-{index}"),
                    &definition,
                    "Definition",
                )),
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
                                .reactive_many(terms.clone(), term_editor),
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

        // view! {
        //     context,
        //     fieldset(class="grid gap-4") {
        //         for term in $terms {
        //             @Card(term)
        //         }
        //     }
        //     button("type"="button", class="btn btn-primary btn-outline w-full", on:click={
        //         let terms = terms.clone();
        //         move || terms.push(Term::new("", ""))
        //     }) {{ "New Card" }}
        // }
    }
}
