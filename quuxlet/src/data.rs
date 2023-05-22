use quux::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Set {
    pub terms: Vec<Term>,
    pub name: String,
    pub id: String,
}

impl Set {
    #[server]
    pub async fn fetch(pool: &sqlx::Pool<sqlx::Sqlite>, set_id: &str) -> Result<Self, sqlx::Error> {
        use sqlx::query::Map;

        let query: Map<_, _, _> = sqlx::query!("SELECT sets.name FROM sets WHERE id = ?", set_id);
        let name = query.fetch_one(pool).await?.name;

        let query: Map<_, _, _> = sqlx::query!(
            "SELECT terms.term, terms.definition FROM terms WHERE set_id = ?",
            set_id
        );

        let terms = query
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|row| Term::new(&row.term, &row.definition))
            .collect();

        Ok(Self {
            terms,
            name,
            id: set_id.to_string(),
        })
    }

    #[server]
    pub async fn create(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        name: &str,
        terms: Vec<Term>,
    ) -> Result<Self, sqlx::Error> {
        // TODO: check for duplicates
        let id = nanoid::nanoid!(10);

        sqlx::query!("INSERT INTO sets (name, id) VALUES (?, ?)", name, id)
            .execute(pool)
            .await?;

        // TODO: transaction?
        for term in terms.clone() {
            sqlx::query!(
                "INSERT INTO terms (set_id, term, definition) VALUES (?, ?, ?)",
                id,
                term.term,
                term.definition
            )
            .execute(pool)
            .await?;
        }

        Ok(Self {
            name: name.to_string(),
            id,
            terms,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct Term {
    pub term: String,
    pub definition: String,
}

impl Term {
    pub fn new(term: &str, definition: &str) -> Self {
        Self {
            term: term.to_string(),
            definition: definition.to_string(),
        }
    }
}
