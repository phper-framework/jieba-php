mod jieba;

use jieba::make_jieba_class;
use phper::{modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_info("description", env!("CARGO_PKG_DESCRIPTION"));
    module.add_info("repository", env!("CARGO_PKG_REPOSITORY"));

    module.add_class(make_jieba_class());

    module
}
