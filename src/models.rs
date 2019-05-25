use crate::actix::prelude::{Addr, Message};
use crate::schema::messages;

use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, Message)]
#[table_name = "messages"]
pub struct Msg {
    pub id: Uuid,
    pub username: String,
    pub body: String,
    pub ts: NaiveDateTime,
}
