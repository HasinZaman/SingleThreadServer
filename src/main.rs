mod server;

use crate::server::method_logic::MethodLogic;

fn main() {
    let logic: MethodLogic = MethodLogic {
        get: MethodLogic::default_get_logic(),
        head: MethodLogic::default_not_allowed_logic(),
        post: MethodLogic::default_not_allowed_logic(),
        put: MethodLogic::default_not_allowed_logic(),
        delete: MethodLogic::default_not_allowed_logic(),
        connect: MethodLogic::default_not_allowed_logic(),
        option: MethodLogic::default_not_allowed_logic(),
        trace: MethodLogic::default_not_allowed_logic(),
        patch: MethodLogic::default_not_allowed_logic(),
    };

    server::start(logic);
}
