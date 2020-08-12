use crate::database::sql_types::UserRole;
use crate::schema::*;
use crate::user::util::{make_hash, make_salt};
use chrono::*;
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
// #[graphql(description="Login user details")]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    // #[graphql(skip)]
    pub hash: Vec<u8>,
    // #[graphql(skip)]
    pub salt: String,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
}

#[juniper::object(description = "Login user details")]
impl User {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn email(&self) -> &str {
        self.email.as_str()
    }
    pub fn first_name(&self) -> &str {
        self.first_name.as_str()
    }
    pub fn last_name(&self) -> Option<&String> {
        self.last_name.as_ref()
    }
    pub fn role(&self) -> UserRole {
        self.role
    }
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct InsertableUser {
    pub id: Uuid,
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct EditableUser {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct UserData {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUser {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedUser(Some(slim_user))
    }
}

impl From<UserData> for InsertableUser {
    fn from(user_data: UserData) -> Self {
        let UserData {
            email,
            password,
            first_name,
            last_name,
            ..
        } = user_data;

        let salt = make_salt();
        let hash = make_hash(&password, &salt).to_vec();
        Self {
            id: Uuid::new_v4(),
            email,
            hash,
            first_name,
            last_name,
            created_at: chrono::Local::now().naive_local(),
            salt,
            role: UserRole::Employee,
        }
    }
}

// impl From<UserData> for EditableUser {
//     fn from(user_data: UserData) -> Self {
//         let UserData {
//             email,
//             // password,
//             first_name,
//             last_name,
//             role,
//             ..
//         } = user_data;

//         // let salt = make_salt();
//         // let hash = make_hash(&password, &salt).to_vec();
//         Self {
//             email,
//             first_name,
//             last_name: last_name.unwrap_or("".to_string()),
//             role,
//         }
//     }
// }
impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            id,
            email,
            first_name,
            last_name,
            role,
            ..
        } = user;

        Self {
            id,
            email,
            first_name,
            last_name: last_name.unwrap_or("".to_string()),
            role,
        }
    }
}
