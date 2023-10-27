use actix::{Actor, Addr, Message, SyncArbiter, SyncContext, Handler};
use actix_web::Error;
use log::{error, debug};
use rusqlite::Connection;
use serde_derive::{Serialize, Deserialize};
use ddns_common::IPV6Info;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new(path: String) -> Db {
        let c_result = Connection::open(path);
        let c = match c_result {
            Ok(conn) => conn,
            Err(_error) => panic!("Error when open a sqlite database"),
        };
        Db { conn: c }
    }

    fn get(&mut self, name: &String) -> rusqlite::Result<String> {
        debug!("get name {}", name);
        let conn = &self.conn;
        let mut stmt = conn.prepare("SELECT id, name, ipv6 FROM ipv6_info where name=?1")?;
        let mut ipv6_iter = stmt.query_map([name], |row| {
            Ok(IPV6Info{
                id: row.get(0)?,
                name: row.get(1)?,
                ipv6: row.get(2)?,
            })
        })?;
        if let Some(Ok(ipv6_info)) = ipv6_iter.next() {
            Ok(ipv6_info.ipv6)
        } else {
            Ok("empty".to_string())
        }
    }


}

impl Actor for Db {
    type Context = SyncContext<Self>;
}

#[derive(Clone)]
pub struct State {
    inner: Addr<Db>,
}

impl State {
    pub fn init(s: String) -> Self {
        let db_addr = SyncArbiter::start(1, move || Db::new(s.clone()));
        let state = State { inner: db_addr };
        state
    }

    pub fn get(&self) -> &Addr<Db> {
        &self.inner
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetIPV6 {
    pub name: String,
}
impl Message for GetIPV6 {
    type Result = Result<String, Error>;
}

impl Handler<GetIPV6> for Db {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GetIPV6, _ctx: &mut Self::Context) -> Self::Result {
        let ipv6_res = self.get(&msg.name);
        match ipv6_res {
            Ok(ipv6) => Ok(ipv6),
            Err(e) => {
                error!("Error on get ipv6, Error {}", e);
                Ok("".to_string())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateIPV6 {
    pub name: String,
    pub ipv6: String,
}
impl Message for UpdateIPV6 {
    type Result = Result<(), Error>;
}

impl Handler<UpdateIPV6> for Db {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: UpdateIPV6, _ctx: &mut Self::Context) -> Self::Result {
        debug!("update {:?}", msg);
        let conn = &mut (self.conn);
        let execute_res = 
        conn.execute("UPDATE ipv6_info SET ipv6=?1 WHERE name=?2;  WHERE (Select Changes() = 0);", [&msg.ipv6, &msg.name]);
        let size = match execute_res {
            Ok(size) => size,
            Err(e) => {
                error!("update error, {}", e);
                return Ok(())
            }
        };
        if size == 0 {
            let _ = conn.execute("INSERT INTO ipv6_info(name, ipv6) select ?2, ?1", [&msg.ipv6, &msg.name]);
        }
        Ok(())
    }
}
