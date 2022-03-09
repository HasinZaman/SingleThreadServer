//import HTTPRequest
mod HTTPRequest;
mod FileReader;
mod SQLReader;
mod HTTPResponse;
mod Server;

use crate::Server::MethodLogic;

fn main() {

    let logic : MethodLogic = MethodLogic{
        get : MethodLogic::default_logic(),
        head : MethodLogic::default_logic(),
        post : MethodLogic::default_logic(),
        put : MethodLogic::default_logic(),
        delete : MethodLogic::default_logic(),
        connect : MethodLogic::default_logic(),
        option : MethodLogic::default_logic(),
        trace : MethodLogic::default_logic(),
        patch : MethodLogic::default_logic(),
    };

    Server::start(logic);
}

