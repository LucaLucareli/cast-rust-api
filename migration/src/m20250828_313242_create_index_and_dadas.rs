use crate::m20250828_140352_create_streaming_schema::*;
use sea_orm_migration::prelude::*;
use shared::enums::access_group_enum::AccessGroupEnum;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // =====================================
        // CRIAÇÃO DE ÍNDICES
        // =====================================

        // USERS
        manager
            .create_index(
                Index::create()
                    .name("idx_users_email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_users_role")
                    .table(Users::Table)
                    .col(Users::Role)
                    .to_owned(),
            )
            .await?;

        // VIDEOS
        manager
            .create_index(
                Index::create()
                    .name("idx_videos_title")
                    .table(Videos::Table)
                    .col(Videos::Title)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_videos_rating")
                    .table(Videos::Table)
                    .col(Videos::Rating)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_videos_featured")
                    .table(Videos::Table)
                    .col(Videos::IsFeatured)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_videos_available")
                    .table(Videos::Table)
                    .col(Videos::IsAvailable)
                    .to_owned(),
            )
            .await?;

        // RELACIONAMENTOS
        manager
            .create_index(
                Index::create()
                    .name("idx_video_categories_video")
                    .table(VideoCategories::Table)
                    .col(VideoCategories::VideoId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_video_categories_category")
                    .table(VideoCategories::Table)
                    .col(VideoCategories::CategoryId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_video_actors_video")
                    .table(VideoActors::Table)
                    .col(VideoActors::VideoId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_video_actors_actor")
                    .table(VideoActors::Table)
                    .col(VideoActors::ActorId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_video_directors_video")
                    .table(VideoDirectors::Table)
                    .col(VideoDirectors::VideoId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_video_directors_director")
                    .table(VideoDirectors::Table)
                    .col(VideoDirectors::DirectorId)
                    .to_owned(),
            )
            .await?;

        // FUNCIONALIDADES
        manager
            .create_index(
                Index::create()
                    .name("idx_watch_history_user")
                    .table(WatchHistory::Table)
                    .col(WatchHistory::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_watch_history_video")
                    .table(WatchHistory::Table)
                    .col(WatchHistory::VideoId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_favorites_user")
                    .table(Favorites::Table)
                    .col(Favorites::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_favorites_video")
                    .table(Favorites::Table)
                    .col(Favorites::VideoId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_ratings_user")
                    .table(Ratings::Table)
                    .col(Ratings::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_ratings_video")
                    .table(Ratings::Table)
                    .col(Ratings::VideoId)
                    .to_owned(),
            )
            .await?;

        // =====================================
        // DADOS INICIAIS
        // =====================================

        // Categorias
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO categories (id, name, description, created_at) VALUES
                ('cat-001', 'Ação', 'Filmes e séries de ação', NOW()),
                ('cat-002', 'Comédia', 'Filmes e séries de comédia', NOW()),
                ('cat-003', 'Drama', 'Filmes e séries dramáticos', NOW()),
                ('cat-004', 'Ficção Científica', 'Filmes e séries de ficção científica', NOW()),
                ('cat-005', 'Terror', 'Filmes e séries de terror', NOW()),
                ('cat-006', 'Romance', 'Filmes e séries românticos', NOW()),
                ('cat-007', 'Documentário', 'Documentários', NOW()),
                ('cat-008', 'Animação', 'Filmes e séries animados', NOW())
                "#,
            )
            .await?;

        // Grupos de acesso
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                INSERT INTO access_groups (id, name, description, created_at) VALUES
                ({viewer}, 'Usuários Básicos', 'Acesso básico ao catálogo', NOW()),
                ({premium}, 'Usuários Premium', 'Acesso completo ao catálogo', NOW()),
                ({admin}, 'Administradores', 'Acesso total ao sistema', NOW())
                "#,
                viewer = AccessGroupEnum::VIEWER as i32,
                premium = AccessGroupEnum::PREMIUM as i32,
                admin = AccessGroupEnum::ADMIN as i32,
            ))
            .await?;

        // Usuários
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO users (id, email, password_hash, name, role, created_at, updated_at) VALUES
                ('admin-001', 'admin@streaming.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/HS.iK8i', 'Admin', 'Admin', NOW(), NOW()),
                ('viewer-001', 'viewer@streaming.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/HS.iK8i', 'Viewer', 'Viewer', NOW(), NOW())
                "#,
            )
            .await?;

        // Associação de usuários a grupos

        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                INSERT INTO users_access_groups (id, user_id, access_group_id, assigned_at) VALUES
                ('uag-001', 'admin-001', {}, NOW()),
                ('uag-002', 'viewer-001', {}, NOW())
                "#,
                AccessGroupEnum::ADMIN as i32, // admin-001 pertence ao grupo ADMIN
                AccessGroupEnum::VIEWER as i32  // viewer-001 pertence ao grupo VIEWER
            ))
            .await?;

        Ok(())
    }
}
