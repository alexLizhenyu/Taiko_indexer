use anyhow::Result;
use diesel::{r2d2::ConnectionManager, PgConnection};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use std::env;

lazy_static! {
    static ref PG_POOL: PgPool = init_pgpool();
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn init_pgpool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .map_err(|err| panic!("create pg pool error: {}", err))
        .unwrap()
}

pub fn get_conn() -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
    let conn = PG_POOL.get()?;
    Ok(conn)
}
