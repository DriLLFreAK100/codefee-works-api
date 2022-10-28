use postgres::{Client, Error, NoTls};
use std::thread;

fn create_tables() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://admin:123@localhost:9000/todo", NoTls)?;

    client.batch_execute(
        "
            CREATE TABLE IF NOT EXISTS todo (
                id              SERIAL PRIMARY KEY,
                title           VARCHAR NOT NULL,
                description     VARCHAR,
                status          SMALLINT
            )
        ",
    )?;

    Ok(())
}

pub fn configure() {
    thread::spawn(|| {
        create_tables().expect("Configure DB Issue");
    })
    .join()
    .expect("Panic!");
}
