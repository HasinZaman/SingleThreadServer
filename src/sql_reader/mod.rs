use std::env;
use mysql::*;
use mysql::prelude::*;
use std::vec::Vec;

#[derive(Debug)]
pub struct DataBase {
    db_host : String,
    db_port : String,
    db_name : String,
    db_username : String,
    db_password : String,
}

struct addres {
    street : String
    //...
}

// excute(
//     "Select address, phone from users", 
//     |row| -> addres{row.index(0), row.index(1)}
// )

impl DataBase {
    pub fn execute<B, F : Fn(mysql::Row)->B>(&self, command : &str, row_logic : F) -> Vec<B> {
        let mut conn = self.get_conn();

        let stmp = conn.prep(command).unwrap();

        return conn.exec_iter(stmp, ())
            .unwrap()
            .map(|wrapped_row| row_logic(wrapped_row.unwrap()))
            .collect();
    }

    fn get_conn(&self) -> mysql::PooledConn {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.db_username,
            self.db_password,
            self.db_host,
            self.db_port,
            self.db_name
        );

        println!("{}", url);

        let url : Opts = Opts::from_url(&url).unwrap();

        let pool = Pool::new(url).unwrap();

        pool.get_conn().unwrap()
    }
}

macro_rules! env_var_to_variable {
    ($key : literal, $var : ident) => {
        match env::var($key) {
            Err(_err) => {
                println!("{} not assigned", $key);
                return Option::None
            },
            Ok(ok) => {$var = ok},
        }
    };
}

fn get_database() -> Option<DataBase>{
    let db_host : String;
    let db_port : String;
    let db_name : String;
    let db_username : String;
    let db_password : String;

    env_var_to_variable!("DB_host", db_host);
    env_var_to_variable!("DB_port", db_port);
    env_var_to_variable!("DB_name", db_name);
    env_var_to_variable!("DB_username", db_username);
    env_var_to_variable!("DB_password", db_password);

    Option::Some(
        DataBase{
            db_host : db_host,
            db_port : db_port,
            db_name : db_name,
            db_username : db_username,
            db_password : db_password,
        }
    )
}