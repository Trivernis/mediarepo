use crate::repo::Repo;
use typemap_rev::TypeMapKey;

pub struct RepoKey;

impl TypeMapKey for RepoKey {
    type Value = Repo;
}
