use super::text_input;
use crate::data::Term;
use quux::{prelude::*, tree::Element};

macro_rules! action_button {
    ($tooltip:expr, $callback:expr, $icon:literal) => {
        button()
            .class("tooltip btn btn-square text-white")
            .data_attribute("tip", $tooltip)
            .attribute("title", $tooltip)
            .attribute("type", "button")
            .on(
                "click",
                event!({
                    // let terms = terms.clone();
                    // let index = index.clone();
                    $callback
                }),
            )
            .raw_html(include_str!(concat!("../../../assets/", $icon, ".svg")))
    };
}

// TODO: add trippy animations
pub fn term_card<'a>(
    index: Store<usize>,
    Term { term, definition }: &Term,
    terms: store::List<Term>,
) -> Element<'a, impl Item> {
    fieldset()
        .class("card card-bordered bg-base-200 shadow")
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
                                .child(action_button!(
                                    "Move Left",
                                    {
                                        let index = index.clone();
                                        let terms = terms.clone();
                                        move || {
                                            let index = *index.get();
                                            terms.swap(index, index.saturating_sub(1));
                                        }
                                    },
                                    "left-arrow"
                                ))
                                .child(action_button!(
                                    "Move Right",
                                    {
                                        let index = index.clone();
                                        let terms = terms.clone();
                                        move || {
                                            let index_value = *index.get();

                                            terms.swap(
                                                index_value,
                                                index_value
                                                    .saturating_add(1)
                                                    .min(terms.length() - 1),
                                            );
                                        }
                                    },
                                    "right-arrow"
                                )),
                        )
                        .child(action_button!(
                            "Delete",
                            {
                                let index = index.clone();
                                let terms = terms.clone();
                                move || {
                                    terms.remove(*index.get());
                                }
                            },
                            "bin"
                        )),
                )
                .child(text_input(term, "Term", true))
                .child(text_input(definition, "Definition", true)),
        )
}
