use std::collections::HashMap;

use super::Head;
use crate::components::flashcards::Term;
use quux::{prelude::*, tree::prelude::*};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    term: Term,
}

impl Component for Card {
    fn render(self, context: Context<Self>) -> impl Item {
        type Component = Card;
        fieldset()
            .class("card card-bordered shadow")
            .child(legend().class("badge").text("Card"))
    }
}

impl component::Init for Card {
    type Props = Term;
    fn init(term: Self::Props) -> Self {
        Self { term }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Create;

impl Component for Create {
    fn render(self, context: Context<Self>) -> impl Item {
        type Component = Create;
        let terms = store::List::<Term>::new(vec![Term::new("", ""), Term::new("", "")]);
        html()
            .attribute("lang", "en")
            .component(Head::new("Flashcards - QUUX"))
            .child(
                body().child(h1().text("Create Set")).child(
                    form()
                        .class("grid gap-4")
                        .child(
                            input()
                                .attribute("type", "text")
                                .attribute("placeholder", "Set Name")
                                .class("input input-bordered input-primary w-full"),
                        )
                        .child(
                            // TODO: for loop!
                            fieldset().class("grid gap-4"),
                        )
                        .child(
                            button()
                                .attribute("type", "button")
                                .class("btn btn-primary btn-outline w-full")
                                .text("New Card")
                                .on("click", event!(|| { panic!("LOLZ!!!!!!!!") })),
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
