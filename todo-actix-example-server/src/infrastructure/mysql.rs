use actix::Actor;
use actix::SyncContext;
use actix_web::error::ErrorInternalServerError;
use actix_web::Error as AWError;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

type MPool = Pool<ConnectionManager<MysqlConnection>>;
type MPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn init_pool(database_url: &str) -> Result<MPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub struct MySqlCli {
    pool: MPool,
}

impl MySqlCli {
    pub fn new(mpool: MPool) -> Self {
        MySqlCli { pool: mpool }
    }

    pub fn get_conn(&self) -> Result<MPooledConnection, AWError> {
        self.pool.get().map_err(|e| ErrorInternalServerError(e))
    }
}

impl Actor for MySqlCli {
    type Context = SyncContext<Self>;
}
