use clickhouse_rs::Pool;
use futures::Future;
use std::io::{stdin, stdout, Write};

fn main() {
    // Important: set pool_max and pool_min to 1
    //let database_url = "tcp://localhost:9001?pool_max=1&pool_min=1";

    //env_logger::init();

    let database_uri = std::env::args().nth(1).expect("please enter db uri");

    let pool = Pool::new(database_uri);

    let mut buf = String::new();

    loop {
        print!("press enter to run query:");
        stdout().flush().expect("could not flush stdout");

        stdin()
            .read_line(&mut buf)
            .expect("could not read line from stdin");

        let fut = pool
            .get_handle()
            .and_then(move |c| c.query("select 1;").fetch_all())
            .and_then(move |(_, block)| {
                println!("query success: {:?}", block);
                Ok(())
            })
            .map_err(|err| println!("database error: {}", err));

        tokio::run(fut);
    }
}
