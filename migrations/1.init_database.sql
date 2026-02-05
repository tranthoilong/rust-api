
-- Enable pgcrypto for UUID generation if needed (though we use a custom function for v7)
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Enable pg_trgm for fuzzy / typo-tolerant search (ILIKE %...%, similarity, trigram indexes)
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Function to generate UUID v7
-- Based on common implementations for Postgres < 17
CREATE OR REPLACE FUNCTION uuid_generate_v7()
RETURNS uuid
AS $$
DECLARE
  unix_ts_ms bytea;
  uuid_bytes bytea;
BEGIN
  unix_ts_ms = substring(int8send(floor(extract(epoch from clock_timestamp()) * 1000)::bigint) from 3);

  -- use random v4 uuid as starting point (which has the same variant we need)
  uuid_bytes = uuid_send(gen_random_uuid());

  -- overlay timestamp
  uuid_bytes = overlay(uuid_bytes placing unix_ts_ms from 1 for 6);

  -- set version 7
  uuid_bytes = set_byte(uuid_bytes, 6, (get_byte(uuid_bytes, 6) & x'0f'::int) | x'70'::int);

  RETURN encode(uuid_bytes, 'hex')::uuid;
END
$$
LANGUAGE plpgsql
VOLATILE;

DO $$ BEGIN
    CREATE TYPE user_status AS ENUM ('active', 'inactive', 'deleted');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    status user_status DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);
-- Đảm bảo tên role duy nhất để hỗ trợ ON CONFLICT
ALTER TABLE roles ADD CONSTRAINT roles_name_key UNIQUE (name);

CREATE TABLE IF NOT EXISTS user_roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL,
    role_id UUID NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

CREATE TABLE IF NOT EXISTS permissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);
-- Đảm bảo tên permission duy nhất để hỗ trợ ON CONFLICT
ALTER TABLE permissions ADD CONSTRAINT permissions_name_key UNIQUE (name);

CREATE TABLE IF NOT EXISTS role_permissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    role_id UUID NOT NULL,
    permission_id UUID NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (role_id) REFERENCES roles(id),
    FOREIGN KEY (permission_id) REFERENCES permissions(id)
);

CREATE TABLE IF NOT EXISTS media (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL,
    media_type TEXT NOT NULL,
    file_path TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Bảng lưu thông tin hồ sơ người dùng, tách khỏi bảng users
CREATE TABLE IF NOT EXISTS user_profiles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL UNIQUE,
    avatar_url TEXT,
    phone TEXT,
    address TEXT,
    bio TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Bảng audit log cho các hành động nghiệp vụ
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID,
    action TEXT NOT NULL,
    entity_type TEXT,
    entity_id UUID,
    metadata JSONB, -- thông tin thêm (context, params,...)
    old_data JSONB, -- dữ liệu trước khi thay đổi
    new_data JSONB, -- dữ liệu sau khi thay đổi
    ip_address TEXT,
    user_agent TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Bảng cấu hình hệ thống dạng key-value
CREATE TABLE IF NOT EXISTS settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    key TEXT NOT NULL,
    value JSONB,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);

ALTER TABLE settings ADD CONSTRAINT settings_key_unique UNIQUE (key);

-- Bảng banner tổng
CREATE TABLE IF NOT EXISTS banners (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    title TEXT NOT NULL,
    slug TEXT,
    "key" TEXT, -- dùng để định danh banner khi render
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    starts_at TIMESTAMP,
    ends_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);

ALTER TABLE banners ADD CONSTRAINT banners_slug_unique UNIQUE (slug);
ALTER TABLE banners ADD CONSTRAINT banners_key_unique UNIQUE ("key");

-- Bảng item con của banner (slide, item quảng cáo,...)
CREATE TABLE IF NOT EXISTS banner_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    banner_id UUID NOT NULL,
    title TEXT,
    subtitle TEXT,
    image_url TEXT,
    link_url TEXT,
    position INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (banner_id) REFERENCES banners(id)
);

-- Bảng categories dùng chung cho nhiều loại nội dung (post, product, media, ...)
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    parent_id UUID,
    name TEXT NOT NULL,
    slug TEXT,
    type TEXT NOT NULL, -- ví dụ: 'post', 'product', 'media', ...
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (parent_id) REFERENCES categories(id)
);

ALTER TABLE categories ADD CONSTRAINT categories_slug_type_unique UNIQUE (slug, type);

-- Bảng tags dùng chung (cũng có type để phân biệt scope)
CREATE TABLE IF NOT EXISTS tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name TEXT NOT NULL,
    slug TEXT,
    type TEXT NOT NULL, -- ví dụ: 'post', 'product', 'media', ...
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);

ALTER TABLE tags ADD CONSTRAINT tags_slug_type_unique UNIQUE (slug, type);

-- Kiểu enum cho trạng thái bài viết
DO $$ BEGIN
    CREATE TYPE post_status AS ENUM ('draft', 'published', 'archived');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Bảng posts (bài viết / nội dung)
CREATE TABLE IF NOT EXISTS posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    author_id UUID NOT NULL,
    category_id UUID,
    title TEXT NOT NULL,
    slug TEXT,
    excerpt TEXT,
    content TEXT,
    status post_status DEFAULT 'draft', -- ví dụ: draft/published/archived
    published_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

ALTER TABLE posts ADD CONSTRAINT posts_slug_unique UNIQUE (slug);

-- Bảng quan hệ nhiều-nhiều giữa posts và tags
CREATE TABLE IF NOT EXISTS post_tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    post_id UUID NOT NULL,
    tag_id UUID NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

ALTER TABLE post_tags ADD CONSTRAINT post_tags_post_tag_unique UNIQUE (post_id, tag_id);

-- ============================================================
-- Đa ngôn ngữ (i18n)
-- ============================================================

-- Danh sách ngôn ngữ hệ thống hỗ trợ
CREATE TABLE IF NOT EXISTS languages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    code TEXT NOT NULL, -- ví dụ: 'vi', 'en', 'ja'
    name TEXT NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
);

ALTER TABLE languages ADD CONSTRAINT languages_code_unique UNIQUE (code);

-- Bản dịch cho categories
CREATE TABLE IF NOT EXISTS category_translations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    category_id UUID NOT NULL,
    language_code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (category_id) REFERENCES categories(id),
    FOREIGN KEY (language_code) REFERENCES languages(code)
);

ALTER TABLE category_translations
    ADD CONSTRAINT category_translations_unique UNIQUE (category_id, language_code);

-- Bản dịch cho tags
CREATE TABLE IF NOT EXISTS tag_translations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    tag_id UUID NOT NULL,
    language_code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (tag_id) REFERENCES tags(id),
    FOREIGN KEY (language_code) REFERENCES languages(code)
);

ALTER TABLE tag_translations
    ADD CONSTRAINT tag_translations_unique UNIQUE (tag_id, language_code);

-- Bản dịch cho posts
CREATE TABLE IF NOT EXISTS post_translations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    post_id UUID NOT NULL,
    language_code TEXT NOT NULL,
    title TEXT NOT NULL,
    slug TEXT,
    excerpt TEXT,
    content TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (language_code) REFERENCES languages(code)
);

ALTER TABLE post_translations
    ADD CONSTRAINT post_translations_post_lang_unique UNIQUE (post_id, language_code);

-- Bản dịch cho banners
CREATE TABLE IF NOT EXISTS banner_translations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    banner_id UUID NOT NULL,
    language_code TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL,
    FOREIGN KEY (banner_id) REFERENCES banners(id),
    FOREIGN KEY (language_code) REFERENCES languages(code)
);

ALTER TABLE banner_translations
    ADD CONSTRAINT banner_translations_unique UNIQUE (banner_id, language_code);

-- Xóa các user trùng email, chỉ giữ lại user có id nhỏ nhất/lớn nhất (UUID không so sánh được > < theo thứ tự thời gian như SERIAL, nhưng v7 có thể sort được)
-- With UUID v7, alphabetical sort ~ time sort if strictly monotonic, but generally we can trust created_at if reliable.
-- Or just rely on generated UUIDs being sortable.
DELETE FROM users a USING users b WHERE a.email = b.email AND a.created_at > b.created_at;

-- Thêm constraint UNIQUE cho cột email
ALTER TABLE users ADD CONSTRAINT users_email_key UNIQUE (email);
