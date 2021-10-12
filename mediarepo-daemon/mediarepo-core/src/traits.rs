use async_trait::async_trait;

#[async_trait]
pub trait AsyncTryFrom<T> {
    type Error;
    fn async_try_from(other: T) -> Result<Self, Self::Error>;
}
