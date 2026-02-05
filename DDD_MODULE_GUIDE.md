## Hướng dẫn viết module mới theo DDD + search/sort/pagination

Tài liệu này mô tả **chuẩn kiến trúc** đang dùng cho các module `user`, `role`, `permission`, `media`.  
Các module mới (entity mới) nên **bám đúng pattern** này để code đồng nhất và dễ bảo trì.

---

### 1. Kiến trúc tổng quan

- **Layering**
  - **domain**: entity thuần, repository trait, value object (vd: `*SearchFilter`).
  - **application**: use case (service), DTO input/output (vd: `ListParams`, `PaginatedResult<T>`).
  - **infrastructure**: implement repository (Postgres, Redis, …), dùng SQLx, query builder.
  - **interface/http**: axum handler, mapping HTTP ↔ DTO application.

- **Các file chuẩn hiện có**
  - `application/common/list_params.rs`: `ListParams`, `PaginatedResult<T>`.
  - `shared/utils/query.rs`: query builder chung (`build_query`, `build_query_with_seed`, cursor).
  - `domain/entities/*`: entity (derive `sqlx::FromRow`).
  - `domain/repositories/*_repository.rs`: repository trait + `*SearchFilter`.
  - `infrastructure/persistence/postgres/*_repo.rs`: Postgres implementation.
  - `interface/http/handlers/*_handler.rs`: HTTP handler.

---

### 2. DTO chung: ListParams + PaginatedResult

- Định nghĩa tại `application/common/list_params.rs`:
  - **ListParams**
    - `search: Option<String>`
    - `fields: Option<Vec<String>>`
    - `sort_by: Option<String>` – format: `"field:asc"` hoặc `"field:desc"`.
    - `cursor: Option<String>` – dùng cho cursor-based pagination.
    - `limit: Option<i64>`
  - **PaginatedResult<T>**
    - `items: Vec<T>`
    - `next_cursor: Option<String>`
    - `limit: i64`

- **Nguyên tắc**
  - `ListParams` là DTO thuộc **application layer**.  
  - Domain **không** import `ListParams` trực tiếp → domain dùng `*SearchFilter` riêng.
  - Infrastructure **có thể** dùng lại `ListParams` để gọi query builder chung.

---

### 3. Query builder chung (`shared/utils/query.rs`)

- Các kiểu chính:
  - `FieldInfo { name: &'static str, field_type: FieldType }`
  - `FieldType` = `Text | Timestamp | Uuid | I64`
  - `BindValue` = `Text(String) | Timestamp(DateTime<Utc>) | Uuid(Uuid) | I64(i64)`
  - `SortDirection` = `Asc | Desc`

- Hàm chính:
  - `build_query(base_sql, &ListParams, &allowed_fields, default_sort_field, default_sort_dir, search_fields)`
  - `build_query_with_seed(base_sql, &ListParams, &allowed_fields, default_sort_field, default_sort_dir, search_fields, seed_conditions, seed_bind_start_index)`

- **Ý tưởng**
  - `base_sql`: câu SELECT cơ bản (đã có `WHERE deleted_at IS NULL` nếu dùng soft delete).
  - Dựa trên `search`, `fields`, `sort_by`, `cursor`, `limit` → sinh ra:
    - câu SQL cuối (`built.sql`)
    - danh sách `built.binds: Vec<BindValue>`
    - `built.limit`, `built.sort_field`, …
  - `search_fields`: danh sách các cột text được phép search (vd: `["name", "email"]`).
  - `allowed_fields`: danh sách cột cho sort/filter, kèm kiểu để decode cursor đúng.
  - Cursor được encode dạng base64 với payload `"sort_value|id"`.

---

### 4. Pattern cho MỖI entity

#### 4.1. Domain: Entity + SearchFilter + Repository trait

1. **Entity**
   - Định nghĩa trong `domain/entities/<entity>.rs`.
   - Bắt buộc derive:
     - `#[derive(Debug, Clone, sqlx::FromRow)]`
   - Ví dụ rút gọn:

   ```rust
   #[derive(Debug, Clone, FromRow)]
   pub struct Permission {
       pub id: Uuid,
       pub name: String,
       pub created_at: Option<DateTime<Utc>>,
       pub updated_at: Option<DateTime<Utc>>,
       pub deleted_at: Option<DateTime<Utc>>,
   }
   ```

2. **SearchFilter**
   - Đặt trong `domain/repositories/<entity>_repository.rs`.
   - Chỉ chứa các field **có nghĩa domain** (không phải DTO HTTP thuần).
   - Ví dụ:

   ```rust
   #[derive(Debug, Clone)]
   pub struct UserSearchFilter {
       pub search: Option<String>,
   }

   #[derive(Debug, Clone)]
   pub struct RoleSearchFilter {
       pub search: Option<String>,
   }

   #[derive(Debug, Clone)]
   pub struct PermissionSearchFilter {
       pub search: Option<String>,
   }

   #[derive(Debug, Clone)]
   pub struct MediaSearchFilter {
       pub search: Option<String>,
       pub user_id: Option<Uuid>,
   }
   ```

3. **Repository trait**
   - Đặt trong cùng file với `SearchFilter`.
   - **Bắt buộc** có method `search` dùng `*SearchFilter` + cursor pagination:

   ```rust
   #[async_trait]
   pub trait PermissionRepository: Send + Sync {
       #[allow(dead_code)]
       async fn find_all(&self) -> Result<Vec<Permission>, String>;

       async fn search(
           &self,
           filter: &PermissionSearchFilter,
           sort_by: Option<String>,
           cursor: Option<String>,
           limit: i64,
       ) -> Result<PaginatedResult<Permission>, String>;

       // các method CRUD/khác ...
   }
   ```

4. **Arc impl**
   - Luôn có impl sẵn cho `Arc<T>`:

   ```rust
   #[async_trait]
   impl<T: PermissionRepository + ?Sized + Send + Sync> PermissionRepository for Arc<T> {
       async fn search(
           &self,
           filter: &PermissionSearchFilter,
           sort_by: Option<String>,
           cursor: Option<String>,
           limit: i64,
       ) -> Result<PaginatedResult<Permission>, String> {
           (**self).search(filter, sort_by, cursor, limit).await
       }
       // forward các method khác...
   }
   ```

#### 4.2. Infrastructure: Pg*Repository (Postgres)

- File: `infrastructure/persistence/postgres/<entity>_repo.rs`.
- Cấu trúc chuẩn:

```rust
pub struct PgPermissionRepository {
    pool: Pool<Postgres>,
}

impl PgPermissionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PermissionRepository for PgPermissionRepository {
    async fn search(
        &self,
        filter: &PermissionSearchFilter,
        sort_by: Option<String>,
        cursor: Option<String>,
        limit: i64,
    ) -> Result<PaginatedResult<Permission>, String> {
        let allowed_fields = [
            FieldInfo { name: "name",       field_type: FieldType::Text },
            FieldInfo { name: "created_at", field_type: FieldType::Timestamp },
        ];

        let base_sql = r#"
            SELECT id, name, created_at, updated_at, deleted_at
            FROM permissions
            WHERE deleted_at IS NULL
        "#;

        // Map từ filter domain → ListParams nội bộ
        let params = ListParams {
            search: filter.search.clone(),
            fields: None,
            sort_by,
            cursor,
            limit: Some(limit),
        };

        let built = build_query(
            base_sql,
            &params,
            &allowed_fields,
            "created_at",
            SortDirection::Desc,
            &["name"],
        )?;

        let mut query = sqlx::query_as::<_, Permission>(&built.sql);
        for b in built.binds {
            query = match b {
                BindValue::Text(v)      => query.bind(v),
                BindValue::Timestamp(v) => query.bind(v),
                BindValue::Uuid(v)      => query.bind(v),
                BindValue::I64(v)       => query.bind(v),
            };
        }

        let items = query.fetch_all(&self.pool).await.map_err(|e| e.to_string())?;

        let next_cursor = if items.len() as i64 == built.limit {
            if let Some(last) = items.last() {
                match built.sort_field {
                    "name"       => Some(encode_cursor_text(&last.name, last.id)),
                    "created_at" => last.created_at.map(|dt| encode_cursor_ts(dt, last.id)),
                    _            => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(PaginatedResult { items, next_cursor, limit: built.limit })
    }

    // các method CRUD khác ...
}
```

- Nếu cần **filter cố định** (vd theo `user_id` như `Media`), dùng `build_query_with_seed`:

```rust
let params = ListParams { /* từ MediaSearchFilter */ };

let built = if let Some(uid) = filter.user_id {
    build_query_with_seed(
        base_sql,
        &params,
        &allowed_fields,
        "created_at",
        SortDirection::Desc,
        &["media_type", "file_path"],
        &[("user_id = $1", BindValue::Uuid(uid))],
        1, // vị trí bắt đầu cho bind index
    )?
} else {
    build_query(
        base_sql,
        &params,
        &allowed_fields,
        "created_at",
        SortDirection::Desc,
        &["media_type", "file_path"],
    )?
};
```

#### 4.3. Application: Use case cho listing/search

- File ví dụ:
  - `application/user/get_users.rs`
  - `application/role/get_roles.rs`
  - `application/permission/get_permissions.rs`
  - `application/media/get_media.rs`

- Pattern chung:

```rust
use crate::application::common::list_params::{ListParams, PaginatedResult};
use crate::domain::entities::<entity>::<Entity>;
use crate::domain::repositories::<entity>_repository::{<Entity>Repository, <Entity>SearchFilter};

pub struct GetEntitiesUseCase {
    repo: Arc<dyn <Entity>Repository>,
}

impl GetEntitiesUseCase {
    pub fn new(repo: Arc<dyn <Entity>Repository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        params: &ListParams,
    ) -> Result<PaginatedResult<<Entity>>, String> {
        let filter = <Entity>SearchFilter {
            search: params.search.clone(),
            // thêm field filter khác nếu cần (vd user_id)
        };

        let limit = params.limit.unwrap_or(20).clamp(1, 100);

        self.repo
            .search(&filter, params.sort_by.clone(), params.cursor.clone(), limit)
            .await
    }
}
```

- **Nguyên tắc**
  - Application **nhận `ListParams`** từ interface.
  - Application **tự build** `*SearchFilter` domain.
  - Application **set limit hợp lệ** (`clamp(1, 100)`).

#### 4.4. Interface/HTTP: Handler

- File ví dụ:
  - `interface/http/handlers/user_handler.rs`
  - `role_handler.rs`
  - `permission_handler.rs`
  - `media_handler.rs`

- Pattern cho list endpoint:

```rust
use crate::application::common::list_params::ListParams;
use crate::interface::http::response::ApiResponse;

pub async fn get_entities(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let usecase = GetEntitiesUseCase::new(state.<entity>_repo.clone());

    match usecase.execute(&params).await {
        Ok(result) => {
            let data = result
                .items
                .into_iter()
                .map(|e| serde_json::json!(e))
                .collect::<Vec<_>>();

            let pagination = serde_json::json!({
                "next_cursor": result.next_cursor,
                "limit": result.limit,
                "sort_by": params.sort_by.clone(),
                "fields": params.fields.clone(),
                "search": params.search.clone()
            });

            ApiResponse::success_with_pagination(data, pagination, None).into_response()
        }
        Err(e) => ApiResponse::<()>::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR".to_string(),
            e,
            None,
            None,
        )
        .into_response(),
    }
}
```

- **Chú ý import**
  - Luôn import `ListParams` từ `application::common::list_params::ListParams`, **không** lấy từ `shared::utils::query`.

---

### 5. Quy ước tên & DB

- **Tên struct/filter/use case**
  - `XxxSearchFilter` (domain).
  - `GetXxxUseCase`, `CreateXxxUseCase`, `UpdateXxxUseCase`, `DeleteXxxUseCase` (application).

- **Bảng DB**
  - Cột chuẩn: `id`, `created_at`, `updated_at`, `deleted_at`.
  - Soft delete dùng `deleted_at IS NULL` trong `base_sql`.
  - Nếu muốn dùng `ON CONFLICT (name)` trong seed hoặc insert, **bắt buộc** có:
    - `UNIQUE (name)` / constraint kiểu `"*name_key"` (vd: `permissions_name_key`, `roles_name_key`).

---

### 6. Xử lý lỗi thường gặp (gợi ý)

- **Duplicate unique constraint** (ví dụ: `"permissions_name_key"`)
  - Nguyên nhân: tạo mới entity với `name` trùng (hoặc seed.sql trùng).
  - Hiện tại lỗi bubble lên thành `INTERNAL_SERVER_ERROR` kèm message SQLx.
  - Gợi ý cải thiện (tương lai):
    - Map mã lỗi Postgres `23505` → HTTP 409 + code business `DUPLICATE_RESOURCE`.
    - Có thể xử lý ở repository hoặc lớp mapper error chung.

- **Cursor rỗng / không hợp lệ**
  - Đã được `build_query_with_seed` / `decode_cursor` handle: nếu `cursor` rỗng → bỏ qua paginate theo cursor.
  - Client:
    - Trang đầu **không** gửi `cursor`.
    - Các trang sau gửi đúng `next_cursor` server trả về.

---

### 7. Checklist khi tạo module mới

1. **Domain**
   - Tạo entity trong `domain/entities/<entity>.rs` với `#[derive(FromRow)]`.
   - Tạo `*SearchFilter` và repository trait trong `domain/repositories/<entity>_repository.rs`.

2. **Infrastructure**
   - Tạo `Pg<Entity>Repository` trong `infrastructure/persistence/postgres/<entity>_repo.rs`.
   - Implement đầy đủ repository trait, đặc biệt `search(...)` dùng query builder chung.

3. **Application**
   - Tạo use case list/search `Get<Entity>sUseCase` trong `application/<entity>/`.
   - Nhận `&ListParams`, build `*SearchFilter`, gọi `repo.search(...)`.

4. **Interface/HTTP**
   - Tạo handler trong `interface/http/handlers/<entity>_handler.rs`.
   - Handler list nhận `Query<ListParams>`, gọi use case, trả `ApiResponse::success_with_pagination`.

Chỉ cần bám đúng tài liệu này, các module mới sẽ đồng bộ với kiến trúc hiện tại (DDD + shared query builder + cursor pagination).

