use quux::prelude::*;

use super::{error, nav_bar, Head};

#[derive(Serialize, Deserialize, Clone)]
pub struct SetCard {
    pub name: String,
    pub url: String,
}

impl Component for SetCard {
    fn render(self) -> impl Item
    where
        Self: Sized,
    {
        article().class("card shadow w-auto bg-base-200").child(
            div()
                .class("card-body")
                .child(h2().class("card-title break-words").text(self.name))
                .child(
                    div().class("card-actions").child(
                        a().attribute("href", self.url)
                            .class("btn btn-primary")
                            .text("View Cards"),
                    ),
                ),
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Discover(Vec<SetCard>);

impl Discover {
    /// # Errors
    /// If the database query fails
    #[cfg_server]
    pub async fn new(pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<Self, error::Database> {
        let query: sqlx::query::Map<_, _, _> = sqlx::query!("SELECT * FROM sets");
        let sets = query.fetch_all(pool).await?;

        Ok(Self(
            sets.into_iter()
                .map(|entry| SetCard {
                    name: entry.name,
                    url: format!("/set/{}", entry.id),
                })
                .collect(),
        ))
    }
}

impl Component for Discover {
    fn render(self) -> impl Item
    where
        Self: Sized,
    {
        html()
            .attribute("lang", "en")
            .component(Head::new("Discover - QUUXLET"))
            .child(
                body()
                    .class("base-layout")
                    .child(nav_bar())
                    .child(
                        main()
                            .class("grid content-start p-4")
                            .child(h1().class("break-words").text("Discover"))
                            .child(
                                section()
                                    .class("p-4 grid gap-4 [grid-template-columns:repeat(auto-fit,minmax(50ch,1fr))] w-full")
                                    .child(
                                        self.0
                                        .into_iter()
                                        .map(|set| set.render())
                                        .collect::<Many<_>>(),
                                    ),
                            )
                    ),
            )
    }
}
