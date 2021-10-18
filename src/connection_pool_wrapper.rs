use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use shaku::{Component, Module};

type Manager = ConnectionManager<SqliteConnection>;

pub struct DBConnectionPool {
    pool: Pool<Manager>,
}

impl DBConnectionPool {
    pub fn get(&self) -> PooledConnection<Manager> {
        self.pool.get().unwrap()
    }
}

impl<M> Component<M> for DBConnectionPool
where
    M: Module,
{
    type Interface = DBConnectionPool;

    type Parameters = String;

    fn build(
        _context: &mut shaku::ModuleBuildContext<M>,
        params: Self::Parameters,
    ) -> Box<Self::Interface> {
        let pool = Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(params.as_str()))
            .expect("Error while creating the database connection pool");
        Box::new(Self { pool })
    }
}
