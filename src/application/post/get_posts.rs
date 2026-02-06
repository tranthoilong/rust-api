use std::sync::Arc;

use crate::domain::entities::post::Post;
use crate::domain::repositories::post_repository::{PostRepository, PostSearchFilter};

pub struct GetPostsUseCase {
    repo: Arc<dyn PostRepository>,
}

impl GetPostsUseCase {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<Post>, String> {
        // Tạm thời ignore filter chi tiết, PostRepository::search sẽ trả về bài viết đã publish.
        let filter = PostSearchFilter {
            search: None,
            category_id: None,
            status: None,
        };
        self.repo.search(&filter).await
    }
}

