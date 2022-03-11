use std::env;
use mysql::*;
use mysql::prelude::*;
use std::vec::Vec;

#[derive(Debug)]
pub struct DataBase {
    pub db_host : String,
    pub db_port : String,
    pub db_name : String,
    pub db_username : String,
    pub db_password : String,
}
impl DataBase {
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

    pub fn execute<B, F : Fn(mysql::Row)->B>(&self, command : &str, row_logic : F) -> Vec<B> {
        let mut conn = self.get_conn();

        let stmp = conn.prep(command).unwrap();

        let result = conn.exec_iter(stmp, ())
            .unwrap()
            .map(|wrapped_row| row_logic(wrapped_row.unwrap()));

        let mut return_val : Vec<B> = Vec::new();

        for r in result{
            return_val.push(r);
        }

        return_val
    }
}

pub fn get_connection(){
    let args: Vec<String> = env::args().collect();

    let get_env = |key| {
        match env::var(key){
            Ok(val) => return val,
            Err(err) => panic!("Error"),
        }
    };
    let sql_user = get_env("sql_user");
    let sql_password = get_env("sql_password");
    let sql_addr = get_env("sql_addr");
    let sql_port = get_env("sql_port");

    //"mysql://root:password@localhost:3307/db_name";
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

fn get_env_var() -> Option<DataBase>{
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