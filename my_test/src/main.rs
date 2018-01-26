extern crate log;
extern crate lib_test;

use lib_test::LibTest;

fn main() {

    // Init logger

    // mimir::logger_init();
    // info!("creating templates");

    // core tools
    let mut core = LibTest::new();
    core.set_x(2);
		core.set_y(5);
		core.add();

}

#[test]
fn test_1() {
	  let _a = 0;
}