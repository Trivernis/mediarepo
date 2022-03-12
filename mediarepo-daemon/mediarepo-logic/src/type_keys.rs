use std::sync::Arc;

use crate::dao::repo::Repo;
use mediarepo_core::typemap_rev::TypeMapKey;

pub struct RepoKey;

impl TypeMapKey for RepoKey {
    type Value = Arc<Repo>;
}
