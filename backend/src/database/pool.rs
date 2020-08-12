use super::{ConnectionManager, Pool, PoolError};

embed_migrations!();


fn init_pool_and_migrate(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");
    embedded_migrations::run(&pool.get().expect("[migrate] Failed to get connection.")).expect("Filed to migrate");
    Ok(pool)
}

pub(crate) fn establish_connection(opt: crate::cli_args::Opt) -> Pool {
    init_pool_and_migrate(&opt.database_url).expect("Failed to init pool")
}
