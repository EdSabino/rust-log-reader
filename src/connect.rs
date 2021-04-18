use postgres::{ Client, NoTls, Error };

pub fn connect_to_db() -> Result<Client, Error> {
    Client::connect("postgresql://postgres:postgres@localhost:5432/log", NoTls)
}

