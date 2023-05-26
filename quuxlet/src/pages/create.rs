use super::{nav_bar, Head};
use crate::data::{Set, Term};
pub use post_data::PostData;
use quux::prelude::*;
use term_card::term_card;

mod post_data;
mod term_card;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Create;

impl Component for Create {
    fn render(self, _: Context<Self>) -> impl Item {
        let terms = store::List::<Term>::new(vec![Term::new("", ""), Term::new("", "")]);
        html()
            .attribute("lang", "en")
            .component(Head::new("Flashcards - QUUX"))
            .child(
                body().class("base-layout").child(nav_bar()).child(main().child(h1().text("Create Set")).child(
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
                                    move |index, term| term_card(index, term, terms.clone())
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
