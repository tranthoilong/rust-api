-- Indexes bổ sung cho các bảng, tách riêng khỏi init_database.sql
-- Giúp tối ưu truy vấn trên các cột FK và các trường hay filter.

-- ============================================================
-- RBAC / Users
-- ============================================================

-- user_roles
CREATE INDEX IF NOT EXISTS idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_role_id ON user_roles(role_id);

-- role_permissions
CREATE INDEX IF NOT EXISTS idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_role_permissions_permission_id ON role_permissions(permission_id);

-- media
CREATE INDEX IF NOT EXISTS idx_media_user_id ON media(user_id);

-- user_profiles
CREATE INDEX IF NOT EXISTS idx_user_profiles_user_id ON user_profiles(user_id);

-- audit_logs
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_entity_type_entity_id ON audit_logs(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);

-- ============================================================
-- Banners / Content
-- ============================================================

-- banners
CREATE INDEX IF NOT EXISTS idx_banners_is_active ON banners(is_active);
CREATE INDEX IF NOT EXISTS idx_banners_starts_ends_at ON banners(starts_at, ends_at);

-- banner_items
CREATE INDEX IF NOT EXISTS idx_banner_items_banner_id ON banner_items(banner_id);
CREATE INDEX IF NOT EXISTS idx_banner_items_position ON banner_items(position);

-- categories
CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id);
CREATE INDEX IF NOT EXISTS idx_categories_type ON categories(type);

-- tags
CREATE INDEX IF NOT EXISTS idx_tags_type ON tags(type);

-- posts
CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id);
CREATE INDEX IF NOT EXISTS idx_posts_category_id ON posts(category_id);
CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS idx_posts_published_at ON posts(published_at);

-- post_tags
CREATE INDEX IF NOT EXISTS idx_post_tags_post_id ON post_tags(post_id);
CREATE INDEX IF NOT EXISTS idx_post_tags_tag_id ON post_tags(tag_id);

-- ============================================================
-- i18n (translations)
-- ============================================================

-- languages
CREATE INDEX IF NOT EXISTS idx_languages_is_default ON languages(is_default);

-- category_translations
CREATE INDEX IF NOT EXISTS idx_category_translations_category_lang
    ON category_translations(category_id, language_code);

-- tag_translations
CREATE INDEX IF NOT EXISTS idx_tag_translations_tag_lang
    ON tag_translations(tag_id, language_code);

-- post_translations
CREATE INDEX IF NOT EXISTS idx_post_translations_post_lang
    ON post_translations(post_id, language_code);

-- banner_translations
CREATE INDEX IF NOT EXISTS idx_banner_translations_banner_lang
    ON banner_translations(banner_id, language_code);

