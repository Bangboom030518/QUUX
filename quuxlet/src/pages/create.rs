use super::{nav_bar, Head};
use crate::data::{Set, Term};
pub use post_data::PostData;
use quux::{prelude::*, tree::Element};

mod post_data;

fn text_input(value: &str, placeholder: &str, multiple: bool) -> impl Item {
    input()
        .class("input input-bordered input-primary w-full")
        .attribute(
            "name",
            format!(
                "{}{}",
                placeholder.to_lowercase(),
                if multiple { "[]" } else { "" }
            ),
        )
        .attribute("type", "text")
        .attribute("required", true)
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
                                .child(
                                    button()
                                        .class("tooltip btn btn-square text-white")
                                        .data_attribute("tip", "Move Left")
                                        .attribute("title", "Move Left")
                                        .attribute("type", "button")
                                        .on(
                                            "click",
                                            event!({
                                                let terms = terms.clone();
                                                let index = index.clone();
                                                move || {
                                                    let index = *index.get();
                                                    console_log!("{index}");
                                                    terms.swap(index, index.saturating_sub(1));
                                                }
                                            }),
                                        )
                                        .raw_html(include_str!("../../assets/left-arrow.svg")),
                                )
                                .child(
                                    button()
                                        .class("tooltip btn btn-square text-white")
                                        .data_attribute("tip", "Move Right")
                                        .attribute("title", "Move Right")
                                        .attribute("type", "button")
                                        .on(
                                            "click",
                                            event!({
                                                let terms = terms.clone();
                                                let index = index.clone();
                                                move || {
                                                    let index_value = *index.get();

                                                    terms.swap(
                                                        index_value,
                                                        index_value
                                                            .saturating_add(1)
                                                            .min(terms.length() - 1),
                                                    );
                                                    console_log!(
                                                        "{index_value} --> {}",
                                                        *index.get()
                                                    );
                                                }
                                            }),
                                        )
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
                .child(text_input(term, "Term", true))
                .child(text_input(definition, "Definition", true)),
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
                body().child(nav_bar()).child(main().class("grid content-start p-4").child(h1().text("Create Set")).child(
                    form()
                        .attribute("action", "create")
                        .attribute("method", "POST")
                        .class("grid gap-4 w-full")
                        .child(
                            input()
                                .attribute("type", "text")
                                .attribute("placeholder", "Set Name")
                                .attribute("name", "name")
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
                )),
            )
            .component(InitialisationScript::init(include_str!(
                "../../dist/init.js"
            )))
    }
}

impl Create {
    #[server]
    #[must_use]
    #[allow(clippy::needless_lifetimes, opaque_hidden_inferred_bound)]
    // TODO: remove `allow(..)`
    #[allow(clippy::missing_panics_doc)]
    pub fn routes<'a>(
        pool: &'a sqlx::Pool<sqlx::Sqlite>,
    ) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + 'a {
        use warp::Filter;

        warp::path!("create")
            .and(warp::get())
            .map(|| Self)
            .or(warp::path!("create")
                .and(warp::post())
                .and(warp::any().map(move || pool.clone()))
                .and(warp::body::form::<PostData>())
                .and_then({
                    |pool: sqlx::Pool<sqlx::Sqlite>, data: PostData| async move {
                        println!("{data:?}");
                        let set =
                            Set::create(&pool, &data.name, data.terms)
                                .await
                                .map_err(|error| {
                                    warp::reject::custom(super::error::Database::from(error))
                                })?;

                        // TODO: `.parse()` is infallible
                        Ok::<_, warp::Rejection>(warp::redirect(
                            format!("/set/{}", set.id).parse::<http::Uri>().unwrap(),
                        ))
                    }
                }))
    }
}
