// author : Benoit Brisset
// Creaton date : 22/01/2018
// Corp : Kisio Digital

/// LibTest is a library used to perform
/// mathematical action between 2 int 64
/// like addition.
pub struct LibTest {
    x: i64,
    y: i64,
}

impl LibTest {
    /// Construct LibTest
	pub fn new() -> LibTest {
		LibTest {
		    x: 0,
		    y: 0,
		}
	}

    /// Return x param
    pub fn x(&mut self) -> i64 {
      	self.x
    }

    /// Return x param
    pub fn y(&mut self) -> i64 {
      	self.y
    }
    
    /// Set x param
    pub fn set_x(&mut self, x: i64){
      	self.x = x;
    }

    /// Set x param
    pub fn set_y(&mut self, y: i64 ){
      	self.y = y;
    }

    /// Set a and y params
    pub fn set_param(&mut self, x: i64, y: i64 ){
      	self.x = x;
      	self.y = y;
    }

    /// Addition of x by y
    ///
    /// # Example :
    ///
    /// ```
    /// let mut lib_test = lib_test::LibTest::new();
    /// lib_test.set_x(2);
		/// lib_test.set_y(5);
		/// assert!(7 == lib_test.add());
    /// ```
    pub fn add(&mut self) -> i64 {
      	self.x + self.y
    }
}

#[test]
fn test_add() {
        let mut lib_test = LibTest::new();
        lib_test.set_x(2);
        lib_test.set_y(5);
        assert!(7 == lib_test.add());
}