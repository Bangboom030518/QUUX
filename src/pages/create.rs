use super::Head;
use crate::components::flashcards::Term;
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    term: Term,
}

impl Component for Card {
    fn render(self, context: quux::view::Context<Self>) -> quux::view::Output<Self> {
        type Component = Card;
        view! {
            context,
            fieldset(class="card card-bordered shadow") {
                legend(class="badge") {{ "Card" }}
                div(class="card-body grid gap-2 grid-cols-2") {
                    input("type"="text", class="input input-bordered input-primary w-full", placeholder="Term", value=self.term.term)
                    input("type"="text", class="input input-bordered input-primary w-full", placeholder="Definition", value=self.term.definition)
                }
            }
        }
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
    fn render(self, context: Context<Self>) -> Output<Self> {
        type Component = Create;
        let terms = store::List::<Term>::new(vec![Term::new("", ""), Term::new("", "")]);
        view! {
            context,
            html(lang="en") {
                @Head("Flashcards - QUUX".to_string())
                body {
                    h1 {{ "Create Set" }}
                    form(class="grid gap-4") {
                        input("type"="text", class="input input-bordered input-primary w-full", placeholder="Set Name")
                        fieldset(class="grid gap-4") {
                            for term in $terms {
                                @Card(term)
                            }
                        }
                        button("type"="button", class="btn btn-primary btn-outline w-full", on:click={
                            let terms = terms.clone();
                            move || terms.push(Term::new("", ""))
                        }) {{ "New Card" }}
                        button(class="btn btn-primary w-full") {{ "Create" }}
                    }
                    @InitialisationScript(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}
