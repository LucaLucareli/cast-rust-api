use crate::m20250828_140352_create_streaming_schema::*;
use sea_orm_migration::prelude::*;
use shared::enums::access_group_enum::AccessGroupEnum;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // CRIAÇÃO DE ÍNDICES

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

        // SERIES
        manager
            .create_index(
                Index::create()
                    .name("idx_series_title")
                    .table(Series::Table)
                    .col(Series::Title)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_series_featured")
                    .table(Series::Table)
                    .col(Series::IsFeatured)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_series_release_year")
                    .table(Series::Table)
                    .col(Series::ReleaseYear)
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

        // DADOS INICIAIS

        // Categorias
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO categories (name, description, created_at) VALUES
                ('Ação', 'Filmes e séries de ação', NOW()),
                ('Comédia', 'Filmes e séries de comédia', NOW()),
                ('Drama', 'Filmes e séries dramáticos', NOW()),
                ('Ficção Científica', 'Filmes e séries de ficção científica', NOW()),
                ('Terror', 'Filmes e séries de terror', NOW()),
                ('Romance', 'Filmes e séries românticos', NOW()),
                ('Documentário', 'Documentários', NOW()),
                ('Animação', 'Filmes e séries animados', NOW())
                "#,
            )
            .await?;

        // Grupos de acesso
        manager
            .get_connection()
            .execute_unprepared(&format!(
                r#"
                INSERT INTO access_groups (id, name, description, created_at) VALUES
                ({}, 'Usuários Básicos', 'Acesso básico ao catálogo', NOW()),
                ({}, 'Usuários Premium', 'Acesso completo ao catálogo', NOW()),
                ({}, 'Administradores', 'Acesso ao admin ao sistema', NOW())
                ({}, 'Super Administradores', 'Acesso total ao sistema', NOW())
                "#,
                AccessGroupEnum::VIEWER as i32,
                AccessGroupEnum::PREMIUM as i32,
                AccessGroupEnum::ADMIN as i32,
                AccessGroupEnum::SUPER_ADMIN as i32,
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
                INSERT INTO users_access_groups (user_id, access_group_id, assigned_at) VALUES
                ('admin-001', {}, NOW()),
                ('viewer-001', {}, NOW())
                "#,
                AccessGroupEnum::ADMIN as i32,
                AccessGroupEnum::VIEWER as i32,
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop all created indexes
        manager
            .drop_index(Index::drop().name("idx_users_email").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_users_role").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_series_title").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_series_featured").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_series_release_year").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_videos_title").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_videos_rating").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_videos_available").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_video_categories_video").to_owned())
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_video_categories_category")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(Index::drop().name("idx_video_actors_video").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_video_actors_actor").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_video_directors_video").to_owned())
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_video_directors_director")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(Index::drop().name("idx_watch_history_user").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_watch_history_video").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_favorites_user").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_favorites_video").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_ratings_user").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_ratings_video").to_owned())
            .await?;

        Ok(())
    }
}
