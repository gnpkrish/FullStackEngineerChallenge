use diesel::deserialize;
use diesel::pg::Pg;
use diesel::serialize;
use diesel::sql_types::Text;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Copy,
    Clone,
    AsExpression,
    FromSqlRow,
    juniper::GraphQLEnum,
)]
#[sql_type = "Text"]
// #[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[graphql(name = "Employee")]
    Employee,
    #[graphql(name = "Admin")]
    Admin,
}
impl serialize::ToSql<Text, Pg> for UserRole {
    fn to_sql<W: std::io::Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        match *self {
            UserRole::Employee => out.write_all(b"employee")?,
            UserRole::Admin => out.write_all(b"admin")?,
        }
        Ok(serialize::IsNull::No)
    }
}
impl deserialize::FromSql<Text, Pg> for UserRole {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"employee" => Ok(UserRole::Employee),
            b"admin" => Ok(UserRole::Admin),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
