use crate::domain::entities::language::Language;
use async_trait::async_trait;

#[async_trait]
#[allow(dead_code)]
pub trait LanguageRepository: Send + Sync {
    async fn list_all(&self) -> Result<Vec<Language>, String>;
    async fn get_default(&self) -> Result<Option<Language>, String>;
    async fn find_by_code(&self, code: &str) -> Result<Option<Language>, String>;
}

#[async_trait]
impl<T: LanguageRepository + ?Sized + Send + Sync> LanguageRepository for std::sync::Arc<T> {
    async fn list_all(&self) -> Result<Vec<Language>, String> {
        (**self).list_all().await
    }

    async fn get_default(&self) -> Result<Option<Language>, String> {
        (**self).get_default().await
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Language>, String> {
        (**self).find_by_code(code).await
    }
}

