//
// compile : rustc borrow.rs 
//
// tests to show the utility of Borrow and AsRef
//
//
use std::fmt::Debug;
use std::borrow::Borrow;

fn main() {


    // The AsRef trait is a conversion trait.
    // It’s used for converting some value to a reference in generic code.
    // This trait exist only for container like, String, Box, Vec
    {
        // function with AsRef
        pub fn func_with_asref_trait<T : AsRef<str> + Debug>(input : T) {
            let x_ref = input.as_ref();
            println!("{:?}", x_ref);
        }

        let x : String = "kikou".to_string();

        func_with_asref_trait(x);
        func_with_asref_trait("kikou");

        pub fn func_without_trait<T : Debug>(input : &T) {
            println!("{:?}", &input);
        }

        let z : i32 = 10;
        func_without_trait(&z);

    }

    // The Borrow trait is a conversion trait.
    // Borrow is very similar to, but different than, AsRef.
    {
        // function with Borrow
        pub fn func_with_borrow_trait<T : Borrow<T> + Debug>(input : T) {
            let x_ref = input.borrow();
            println!("{:?}", x_ref);
        }

        // function with ref
        pub fn func_with_ref<T : Debug>(input : T) {
            println!("{:?}", input);
        }

        let x : String = "kikou".to_string();
        let y : i32 = 10;

        #[derive(Debug)]
        struct A {
            x : i32,
            y : i32,
        }
        impl Borrow<i32> for A {
            #[inline]
            fn borrow(&self) -> &i32 {
                &self.x
            }
        }

        let a : A = A{ x: 20, y: 10};
        func_with_borrow_trait(x);
        func_with_borrow_trait("kikou");
        func_with_borrow_trait(&y);
        func_with_borrow_trait(10);
        func_with_borrow_trait(10);
        func_with_borrow_trait(a);

        let b : A = A{ x: 20, y: 10};
        let var1 : String = "kikou".to_string();
        let var2 : i32 = 10;
        func_with_ref(var1);
        func_with_ref("kikou");
        func_with_ref(var2);
        func_with_ref(var2);
        func_with_ref(b);
    }

    {
        // function with Borrow
        pub fn func_with_borrow_trait<T : Borrow<str> + Debug>(input : T) {
            assert_eq!("kikou", input.borrow());
        }

        let var3 : String = "kikou".to_string();
        let var4 = "kikou";


        func_with_borrow_trait(var3);
        func_with_borrow_trait(var4);
    }
}
