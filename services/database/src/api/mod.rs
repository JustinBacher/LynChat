pub mod endpoints;

use endpoints::main;
use sea_orm::DatabaseConnection;

pub async fn run_server(db: DatabaseConnection, port: u16) -> std::io::Result<()> {
    main(db, port).await
}