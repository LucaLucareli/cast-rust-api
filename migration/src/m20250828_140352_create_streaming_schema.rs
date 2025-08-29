use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // USERS
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Role).string().not_null())
                    .col(ColumnDef::new(Users::ProfilePictureUrl).string().null())
                    .col(ColumnDef::new(Users::SubscriptionStatus).string().null())
                    .col(ColumnDef::new(Users::SubscriptionExpiresAt).date_time().null())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // CATEGORIES
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Categories::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(ColumnDef::new(Categories::Description).string().null())
                    .col(ColumnDef::new(Categories::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // VIDEOS
        manager
            .create_table(
                Table::create()
                    .table(Videos::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Videos::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Videos::Title).string().not_null())
                    .col(ColumnDef::new(Videos::Description).string().null())
                    .col(ColumnDef::new(Videos::DurationSeconds).integer().not_null())
                    .col(ColumnDef::new(Videos::ReleaseYear).integer().null())
                    .col(ColumnDef::new(Videos::Rating).double().not_null())
                    .col(ColumnDef::new(Videos::ThumbnailUrl).string().null())
                    .col(ColumnDef::new(Videos::VideoUrl).string().null())
                    .col(ColumnDef::new(Videos::TrailerUrl).string().null())
                    .col(ColumnDef::new(Videos::IsFeatured).boolean().not_null())
                    .col(ColumnDef::new(Videos::IsAvailable).boolean().not_null())
                    .col(ColumnDef::new(Videos::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Videos::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // ACTORS
        manager
            .create_table(
                Table::create()
                    .table(Actors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Actors::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Actors::Name).string().not_null())
                    .col(ColumnDef::new(Actors::Biography).string().null())
                    .col(ColumnDef::new(Actors::BirthDate).date_time().null())
                    .col(ColumnDef::new(Actors::ProfilePictureUrl).string().null())
                    .col(ColumnDef::new(Actors::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // DIRECTORS
        manager
            .create_table(
                Table::create()
                    .table(Directors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Directors::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Directors::Name).string().not_null())
                    .col(ColumnDef::new(Directors::Biography).string().null())
                    .col(ColumnDef::new(Directors::BirthDate).date_time().null())
                    .col(ColumnDef::new(Directors::ProfilePictureUrl).string().null())
                    .col(ColumnDef::new(Directors::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // VIDEO_CATEGORIES
        manager
            .create_table(
                Table::create()
                    .table(VideoCategories::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(VideoCategories::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(VideoCategories::VideoId).string().not_null())
                    .col(ColumnDef::new(VideoCategories::CategoryId).string().not_null())
                    .col(ColumnDef::new(VideoCategories::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // VIDEO_ACTORS
        manager
            .create_table(
                Table::create()
                    .table(VideoActors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(VideoActors::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(VideoActors::VideoId).string().not_null())
                    .col(ColumnDef::new(VideoActors::ActorId).string().not_null())
                    .col(ColumnDef::new(VideoActors::RoleName).string().null())
                    .col(ColumnDef::new(VideoActors::IsLead).boolean().not_null())
                    .col(ColumnDef::new(VideoActors::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // VIDEO_DIRECTORS
        manager
            .create_table(
                Table::create()
                    .table(VideoDirectors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(VideoDirectors::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(VideoDirectors::VideoId).string().not_null())
                    .col(ColumnDef::new(VideoDirectors::DirectorId).string().not_null())
                    .col(ColumnDef::new(VideoDirectors::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // ACCESS_GROUPS
        manager
            .create_table(
                Table::create()
                    .table(AccessGroups::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AccessGroups::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(AccessGroups::Name).string().not_null())
                    .col(ColumnDef::new(AccessGroups::Description).string().null())
                    .col(ColumnDef::new(AccessGroups::Permissions).string().null())
                    .col(ColumnDef::new(AccessGroups::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // USERS_ACCESS_GROUPS
        manager
            .create_table(
                Table::create()
                    .table(UsersAccessGroups::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UsersAccessGroups::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(UsersAccessGroups::UserId).string().not_null())
                    .col(ColumnDef::new(UsersAccessGroups::AccessGroupId).string().not_null())
                    .col(ColumnDef::new(UsersAccessGroups::AssignedAt).date_time().not_null())
                    .col(ColumnDef::new(UsersAccessGroups::AssignedBy).string().null())
                    .to_owned(),
            )
            .await?;

        // WATCH_HISTORY
        manager
            .create_table(
                Table::create()
                    .table(WatchHistory::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WatchHistory::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(WatchHistory::UserId).string().not_null())
                    .col(ColumnDef::new(WatchHistory::VideoId).string().not_null())
                    .col(ColumnDef::new(WatchHistory::WatchedSeconds).integer().not_null())
                    .col(ColumnDef::new(WatchHistory::IsCompleted).boolean().not_null())
                    .col(ColumnDef::new(WatchHistory::LastWatchedAt).date_time().not_null())
                    .col(ColumnDef::new(WatchHistory::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // FAVORITES
        manager
            .create_table(
                Table::create()
                    .table(Favorites::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Favorites::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Favorites::UserId).string().not_null())
                    .col(ColumnDef::new(Favorites::VideoId).string().not_null())
                    .col(ColumnDef::new(Favorites::AddedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // RATINGS
        manager
            .create_table(
                Table::create()
                    .table(Ratings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Ratings::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Ratings::UserId).string().not_null())
                    .col(ColumnDef::new(Ratings::VideoId).string().not_null())
                    .col(ColumnDef::new(Ratings::Rating).integer().not_null())
                    .col(ColumnDef::new(Ratings::Comment).string().null())
                    .col(ColumnDef::new(Ratings::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Ratings::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Ratings::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Favorites::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(WatchHistory::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(UsersAccessGroups::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(AccessGroups::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(VideoDirectors::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(VideoActors::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(VideoCategories::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Directors::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Actors::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Videos::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Categories::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        Ok(())
    }
}

// Idents (um enum para cada tabela)
#[derive(Iden)] enum Users { Table, Id, Email, PasswordHash, Name, Role, ProfilePictureUrl, SubscriptionStatus, SubscriptionExpiresAt, CreatedAt, UpdatedAt }
#[derive(Iden)] enum Categories { Table, Id, Name, Description, CreatedAt }
#[derive(Iden)] enum Videos { Table, Id, Title, Description, DurationSeconds, ReleaseYear, Rating, ThumbnailUrl, VideoUrl, TrailerUrl, IsFeatured, IsAvailable, CreatedAt, UpdatedAt }
#[derive(Iden)] enum Actors { Table, Id, Name, Biography, BirthDate, ProfilePictureUrl, CreatedAt }
#[derive(Iden)] enum Directors { Table, Id, Name, Biography, BirthDate, ProfilePictureUrl, CreatedAt }
#[derive(Iden)] enum VideoCategories { Table, Id, VideoId, CategoryId, CreatedAt }
#[derive(Iden)] enum VideoActors { Table, Id, VideoId, ActorId, RoleName, IsLead, CreatedAt }
#[derive(Iden)] enum VideoDirectors { Table, Id, VideoId, DirectorId, CreatedAt }
#[derive(Iden)] enum AccessGroups { Table, Id, Name, Description, Permissions, CreatedAt }
#[derive(Iden)] enum UsersAccessGroups { Table, Id, UserId, AccessGroupId, AssignedAt, AssignedBy }
#[derive(Iden)] enum WatchHistory { Table, Id, UserId, VideoId, WatchedSeconds, IsCompleted, LastWatchedAt, CreatedAt }
#[derive(Iden)] enum Favorites { Table, Id, UserId, VideoId, AddedAt }
#[derive(Iden)] enum Ratings { Table, Id, UserId, VideoId, Rating, Comment, CreatedAt, UpdatedAt }