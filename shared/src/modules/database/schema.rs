use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

// ====================
// USERS
// ====================
pub mod users {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub email: String,
        pub password_hash: String,
        pub name: String,
        pub role: String,
        pub profile_picture_url: Option<String>,
        pub subscription_status: Option<String>,
        pub subscription_expires_at: Option<NaiveDateTime>,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// CATEGORIES
// ====================
pub mod categories {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "categories")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub name: String,
        pub description: Option<String>,
        pub created_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// VIDEOS
// ====================
pub mod videos {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "videos")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub title: String,
        pub description: Option<String>,
        pub duration_seconds: i32,
        pub release_year: Option<i32>,
        pub rating: f64,
        pub thumbnail_url: Option<String>,
        pub video_url: Option<String>,
        pub trailer_url: Option<String>,
        pub is_featured: bool,
        pub is_available: bool,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// ACTORS
// ====================
pub mod actors {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "actors")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub name: String,
        pub biography: Option<String>,
        pub birth_date: Option<NaiveDateTime>,
        pub profile_picture_url: Option<String>,
        pub created_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// DIRECTORS
// ====================
pub mod directors {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "directors")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub name: String,
        pub biography: Option<String>,
        pub birth_date: Option<NaiveDateTime>,
        pub profile_picture_url: Option<String>,
        pub created_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// VIDEO_CATEGORIES
// ====================
pub mod video_categories {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "video_categories")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub video_id: String,
        pub category_id: String,
        pub created_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// VIDEO_ACTORS
// ====================
pub mod video_actors {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "video_actors")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub video_id: String,
        pub actor_id: String,
        pub role_name: Option<String>,
        pub is_lead: bool,
        pub created_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// VIDEO_DIRECTORS
// ====================
pub mod video_directors {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "video_directors")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub video_id: String,
        pub director_id: String,
        pub created_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// ACCESS_GROUPS
// ====================
pub mod access_groups {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "access_groups")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub name: String,
        pub description: Option<String>,
        pub permissions: Option<String>, // JSON
        pub created_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// USERS_ACCESS_GROUPS
// ====================
pub mod users_access_groups {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "users_access_groups")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub user_id: String,
        pub access_group_id: String,
        pub assigned_at: Option<NaiveDateTime>,
        pub assigned_by: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// WATCH_HISTORY
// ====================
pub mod watch_history {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "watch_history")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub user_id: String,
        pub video_id: String,
        pub watched_seconds: i32,
        pub is_completed: bool,
        pub last_watched_at: Option<NaiveDateTime>,
        pub created_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// FAVORITES
// ====================
pub mod favorites {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "favorites")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub user_id: String,
        pub video_id: String,
        pub added_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ====================
// RATINGS
// ====================
pub mod ratings {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "ratings")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: String,
        pub user_id: String,
        pub video_id: String,
        pub rating: i32,
        pub comment: Option<String>,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
