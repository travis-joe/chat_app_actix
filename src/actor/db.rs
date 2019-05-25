use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::models::Msg;
use crate::schema;

use actix_web::error::Error;
use chrono::Local;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde_derive::Deserialize;
use uuid::Uuid;

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: string,
}

#[derive(Debug, Deserialize)]
pub struct Form {
    pub username: String,
    pub body: String,
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Message for Form {
    type Result = Result<Msg, Error>;
}

impl Message for User {
    type Result = Result<Vec<Msg>, Error>;
}

