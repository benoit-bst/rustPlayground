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
	
    // Static Method (without self in input)
    LibTest::test();

    core.set_x(10).set_y(5);
    //let x: i64 = core2.x();
    let mut x = core.x();
    x = x + 1;
    println!("x : {:?}", x);

}

#[test]
fn test_1() {
    let _a = 0;
}
