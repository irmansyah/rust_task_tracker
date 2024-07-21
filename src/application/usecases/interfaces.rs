use crate::domain::error::ApiError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait AbstractUseCase<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}

#[async_trait(?Send)]
pub trait AbstractPayloadUseCase<S, T> {
    async fn execute(&self, data: S) -> Result<T, ApiError>;
}
