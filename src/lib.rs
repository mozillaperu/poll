extern crate postgres;
extern crate chrono;
extern crate native_tls;

use postgres::{Connection, TlsMode};
use postgres::tls::native_tls::NativeTls;
use std::env;

pub mod models;

pub fn cn() -> Connection {
    //let negotiator = NativeTls::new().unwrap(); //only for production
    Connection::connect("postgres://dev:1234@localhost:5432/poll", TlsMode::None).unwrap() //TlsMode::Require(&negotiator)
}
