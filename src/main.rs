#![feature(drain_filter)]

mod connect;
mod files;
mod table;
mod logs;
mod transaction;

use postgres::Client;
use table::Table;
use std::error::Error;
use files::load_config;
use logs::Logs;
use postgres::types::ToSql;
use std::marker::Sync;

fn main() {
    match connect::connect_to_db() {
        Ok(client) => {
            match start(client) {
                Ok(_) => println!("Finalized successfully"),
                Err(_) => panic!("An error has occured")
            }
        },
        Err(_) => panic!("Could not connect to db")
    }
}

fn start(mut client: Client) -> Result<(), Box<dyn Error>> {
    let configs = load_config::config();
    let mut logs = Logs::new(configs.get::<String>("file_name")?);
    logs.load();
    let table = Table::new(configs)?;
    
    sync_table(&table, &mut client, &logs);
    
    let mut state = table.parse_row_to_hash(&client.query(table.select().as_str(), &[])?[0]);
    let mut transactions = logs.parse_logs();
    let pk = logs.get_value(&table.pk);

    for commited in logs.commited {
        let tr = transactions.get_mut(&commited).unwrap();
        if !tr.finalized {
           tr.commit(&mut state);
        }
    }

    let update_statement = table.update_from_hash(pk, state);
    client.execute(update_statement.as_str(), &[]).unwrap();

    for (key, tr) in transactions.into_iter() {
        if tr.commited {

            if tr.finalized {
                println!("Transação {} não sofreu REDO", key.to_string());
            } else {
                println!("Transação {} sofreu REDO", key.to_string());
            }
        } else {
            println!("Transação {} foi ignorada", key.to_string());
        }
    }
    
    Ok(())
}

fn sync_table(table: &Table, client: &mut Client, logs: &Logs) {
    let update_statement = table.update(logs.get_value(&table.pk));
    let mut values_update = Vec::new();
    let values = logs.get_updatables(&table.pk);
    for i in 0..values.len() {
        values_update.push(&values[i] as &(dyn ToSql + Sync));
    }

    client.execute(update_statement.as_str(), &values_update[..]).unwrap();
}
