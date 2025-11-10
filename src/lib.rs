#![allow(ambiguous_glob_reexports, unused_imports)]

mod configuration;
mod routes;
mod startup;

pub mod prelude {
    pub use crate::configuration::*;
    pub use crate::routes::{health_check::*, subscriptions::*};
    pub use crate::startup::*;
    pub use actix_web::{App, HttpResponse, HttpServer, dev::Server, web::*};
    pub use config::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::net::TcpListener;
    pub use sqlx::*;
    pub use uuid::*;
} 