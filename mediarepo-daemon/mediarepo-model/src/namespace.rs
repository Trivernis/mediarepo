use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::namespace;
use sea_orm::prelude::*;
use sea_orm::{
    Condition, ConnectionTrait, DatabaseBackend, DatabaseConnection, InsertResult, Set, Statement,
};
use std::fmt::Debug;

#[derive(Clone)]
pub struct Namespace {
    db: DatabaseConnection,
    model: namespace::Model,
}

impl Namespace {
    #[tracing::instrument(level = "trace")]
    pub(crate) fn new(db: DatabaseConnection, model: namespace::Model) -> Self {
        Self { db, model }
    }

    /// Retrieves the namespace by id
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let namespace = namespace::Entity::find_by_id(id)
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(namespace)
    }

    /// Retrieves a namespace by its name
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_name<S: AsRef<str> + Debug>(
        db: DatabaseConnection,
        name: S,
    ) -> RepoResult<Option<Self>> {
        let namespace = namespace::Entity::find()
            .filter(namespace::Column::Name.eq(name.as_ref()))
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(namespace)
    }

    /// Returns all namespaces by name
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn all_by_name(db: DatabaseConnection, names: Vec<String>) -> RepoResult<Vec<Self>> {
        if names.is_empty() {
            return Ok(Vec::with_capacity(0));
        }
        let mut condition = Condition::any();
        for name in names {
            condition = condition.add(namespace::Column::Name.eq(name));
        }

        let namespaces = namespace::Entity::find()
            .filter(condition)
            .all(&db)
            .await?
            .into_iter()
            .map(|model| Self::new(db.clone(), model))
            .collect();

        Ok(namespaces)
    }

    /// Adds all namespaces to the database
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn add_all(db: DatabaseConnection, names: Vec<String>) -> RepoResult<Vec<Self>> {
        if names.is_empty() {
            return Ok(vec![]);
        }
        let models: Vec<namespace::ActiveModel> = names
            .into_iter()
            .map(|name| namespace::ActiveModel {
                name: Set(name),
                ..Default::default()
            })
            .collect();
        let txn = db.begin().await?;
        let last_id = txn
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                r#"SELECT MAX(id) AS "max_id" FROM namespaces;"#.to_owned(),
            ))
            .await?
            .and_then(|result| result.try_get("", "max_id").ok())
            .unwrap_or(-1);
        let result: InsertResult<namespace::ActiveModel> =
            namespace::Entity::insert_many(models).exec(&txn).await?;

        let namespaces = namespace::Entity::find()
            .filter(namespace::Column::Id.between(last_id, result.last_insert_id + 1))
            .all(&txn)
            .await?
            .into_iter()
            .map(|model| Self::new(db.clone(), model))
            .collect();
        txn.commit().await?;

        Ok(namespaces)
    }

    /// Adds a namespace to the database
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn add<S: ToString + Debug>(db: DatabaseConnection, name: S) -> RepoResult<Self> {
        let active_model = namespace::ActiveModel {
            name: Set(name.to_string()),
            ..Default::default()
        };
        let active_model = active_model.insert(&db).await?;
        let namespace = Self::by_id(db, active_model.id.unwrap()).await?.unwrap();

        Ok(namespace)
    }

    /// The ID of the namespace
    pub fn id(&self) -> i64 {
        self.model.id
    }

    /// The name of the namespace
    pub fn name(&self) -> &String {
        &self.model.name
    }
}
