use crate::domain::entities::post::{Post, PostStatus};
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PostSearchFilter {
    pub search: Option<String>,
    pub category_id: Option<Uuid>,
    pub status: Option<PostStatus>,
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    #[allow(dead_code)]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, String>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Post>, String>;
    async fn search(&self, filter: &PostSearchFilter) -> Result<Vec<Post>, String>;
    /// Tạo bài viết mới
    async fn create(&self, post: Post) -> Result<Post, String>;
    /// Cập nhật bài viết (theo id)
    async fn update(&self, post: Post) -> Result<Post, String>;
    /// Xoá mềm 1 bài viết
    async fn soft_delete(&self, id: Uuid) -> Result<(), String>;
    /// Xoá mềm nhiều bài viết theo danh sách id
    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String>;
}

#[async_trait]
impl<T: PostRepository + ?Sized + Send + Sync> PostRepository for std::sync::Arc<T> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, String> {
        (**self).find_by_id(id).await
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Post>, String> {
        (**self).find_by_slug(slug).await
    }

    async fn search(&self, filter: &PostSearchFilter) -> Result<Vec<Post>, String> {
        (**self).search(filter).await
    }

    async fn create(&self, post: Post) -> Result<Post, String> {
        (**self).create(post).await
    }

    async fn update(&self, post: Post) -> Result<Post, String> {
        (**self).update(post).await
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), String> {
        (**self).soft_delete(id).await
    }

    async fn soft_delete_many(&self, ids: &[Uuid]) -> Result<(), String> {
        (**self).soft_delete_many(ids).await
    }
}

