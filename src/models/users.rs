use crate::{
    models::{emails::Email, password_requirements::PasswordRequirements},
    schema::users,
    util::handle_blocking_err,
    web::DbConnection,
};

use actix_web::web::block;
use argon2::{self, Config};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A telescope user.
#[derive(Insertable, Queryable, Debug, Clone, Serialize, Deserialize, Associations)]
#[table_name = "users"]
pub struct User {
    /// User's universally unique identifier
    pub id: Uuid,
    /// User's name.
    pub name: String,
    /// Optionally, a link to the user's avatar (profile picture).
    ///
    /// Use the default statically served avatar photo if this is not available.
    avi_location: Option<String>,
    /// The user's bio. This is in commonmark markdown format.
    pub bio: String,
    // FIXME: Discord & Mattermost integration.
    /// A link to the user's Github
    pub github_link: Option<String>,
    /// The user's discord or mattermost chat handle.
    /// (Since RCOS transfered to discord, this is in limbo)
    pub chat_handle: Option<String>,
    /// Is this user a telescope admin.
    pub sysadmin: bool,
    /// The hashed user password.
    pub hashed_pwd: String,
    /// The moment that the account was created.
    pub account_created: DateTime<Utc>,
}

/// Rust only user operations and constants.
impl User {
    /// Number of bytes in a password hash
    const HASH_LENGTH: u32 = 32;

    /// Create the argon config we use for telescope.
    fn make_argon_config<'a>() -> Config<'a> {
        let mut argon_cfg = Config::default();
        argon_cfg.hash_length = Self::HASH_LENGTH;

        // Strongest argon version
        argon_cfg.variant = argon2::Variant::Argon2id;
        argon_cfg.version = argon2::Version::Version13;

        // two lane parallel
        argon_cfg.lanes = 2;
        argon_cfg.thread_mode = argon2::ThreadMode::Parallel;

        argon_cfg
    }

    /// Create a new user from a name and a password. Randomly generate a UUID.
    /// Do not set any user info yet. Fail if password does nto meet requirements.
    pub fn new<T: Into<String>>(name: T, password: &str) -> Result<Self, PasswordRequirements> {
        let reqs: PasswordRequirements = PasswordRequirements::for_password(password);

        if !reqs.satisfied() {
            return Err(reqs);
        }

        let uuid = Uuid::new_v4();

        let hashed_pwd = argon2::hash_encoded(
            password.as_bytes(),
            &uuid.as_bytes()[..],
            &Self::make_argon_config(),
        )
        .unwrap();

        Ok(Self {
            id: uuid,
            name: name.into(),
            avi_location: None,
            bio: String::default(),
            github_link: None,
            chat_handle: None,
            sysadmin: false,
            hashed_pwd,
            account_created: Utc::now(),
        })
    }

    /// Format a uuid into a lowercase hyphenated string.
    pub fn format_uuid(id: Uuid) -> String {
        id.to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string()
    }

    /// Format the associated user id into a string.
    pub fn id_str(&self) -> String {
        Self::format_uuid(self.id)
    }

    /// Resolve the location for the user's profile picture.
    pub async fn picture_url(&self, conn: DbConnection) -> String {
        // Check for existing specification.
        if self.avi_location.is_some() {
            return self.avi_location.clone().unwrap();
        } else {
            // get user emails.
            let emails: Vec<Email> = self.get_emails_from_db(conn).await;
            // Use the first one to generate the Gravitar Hash (Users should
            // always have at least 1 email).
            let email_str: String = emails.first().unwrap().email.as_str().trim().to_lowercase();

            let gravatar_hash = md5::compute(email_str);
            format!(
                "https://www.gravatar.com/avatar/{:x}?d=identicon&s=600",
                gravatar_hash
            )
        }
    }

    /// Get a user from the database by user id asynchronously.
    ///
    /// Return none if user is not found.
    pub async fn get_from_db_by_id(conn: DbConnection, uid: Uuid) -> Option<User> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;

        block(move || users.find(uid).first::<User>(&conn).optional())
            .await
            .map_err(|e| {
                error!("Could not get user from database: {}", e);
                e
            })
            .unwrap()
    }

    // TODO: Test?
    /// Get a user's emails from the database. Return empty vector if there are no
    /// emails found. Returned emails will be sorted by visibility, and then
    /// alphabetically.
    pub async fn get_emails_from_db_by_id(conn: DbConnection, uid: Uuid) -> Vec<Email> {
        use crate::schema::emails::dsl::*;
        use diesel::prelude::*;

        block::<_, Vec<Email>, _>(move || {
            emails
                .filter(user_id.eq(uid))
                .order((is_visible.asc(), email.asc()))
                .load(&conn)
        })
        .await
        .map_err(|e| {
            error!("Could not query database: {}", e);
            e
        })
        .unwrap()
    }

    /// See the get_emails_from_db_by_id
    pub async fn get_emails_from_db(&self, conn: DbConnection) -> Vec<Email> {
        User::get_emails_from_db_by_id(conn, self.id).await
    }

    /// Store the user in the database. On conflict, return error.
    pub async fn store(self, conn: DbConnection) -> Result<(), String> {
        block::<_, usize, _>(move || {
            use crate::schema::users::dsl::*;
            use diesel::prelude::*;
            diesel::insert_into(users).values(&self).execute(&conn)
        })
        .await
        .map_err(|e| handle_blocking_err(e, "Could not add user to database."))
        .map(|n| trace!("Added {} user(s) to database.", n))
    }

    /// Get all users from database.
    pub async fn get_all_from_db(conn: DbConnection) -> Result<Vec<User>, String> {
        block::<_, Vec<User>, _>(move || {
            use crate::schema::users::dsl::*;
            use diesel::prelude::*;
            users.load(&conn)
        })
        .await
        .map_err(|e| handle_blocking_err(e, "Could not load users from database."))
    }
}
