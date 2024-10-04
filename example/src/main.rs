#[derive(serde_derive::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct _User1 {
    pub user_id: i64,
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub gender: String,
    #[serde(rename = "user_id")]
    pub user_id2: String,
    pub email: String,
    #[serde(rename = "given_name")]
    pub given_name: String,
    #[serde(rename = "family_name")]
    pub family_name: String,
    pub nickname: String,
    #[serde(rename = "last_ip")]
    pub last_ip: String,
    #[serde(rename = "logins_count")]
    pub logins_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
}

#[derive(serde_derive_next::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct _User2 {
    pub user_id: i64,
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub gender: String,
    #[serde(rename = "user_id")]
    pub user_id2: String,
    pub email: String,
    #[serde(rename = "given_name")]
    pub given_name: String,
    #[serde(rename = "family_name")]
    pub family_name: String,
    pub nickname: String,
    #[serde(rename = "last_ip")]
    pub last_ip: String,
    #[serde(rename = "logins_count")]
    pub logins_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
}

fn main() {}
