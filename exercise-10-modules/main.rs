/*
MODULES

They allow you to split code into logical units with custom visibility for contained elements
 */

mod my_module {
    fn private_function() {}
    pub fn public_function() {
        private_function();
    }

    pub mod nested_public_module {
        pub(in crate::my_module) fn public_function_in_my_module() {} // Public to `my_module` only!
    pub(super) fn public_function_to_parent_module() {} // Public to the parent module only (in this case, `my_module`)!
    pub(crate) fn public_function_to_crate() {} // Public to this crate only!
    fn private_function() {}
        fn public_function() {
            self::private_function(); // "self::" to resolve ambiguity and reference the current module
            super::private_function(); // "super::" to resolve ambiguity and reference the parent module
        }
    }

    struct PrivateStruct;
    pub struct PublicStructWithPrivateFields {
        private: i32,
    }
    pub struct PublicStructWithPublicFields {
        pub public: i32,
    }

    impl PublicStructWithPrivateFields {
        fn private_function(&mut self) {
            self.private += 1;
        }
        pub fn public_function(&mut self) {
            self.private += 1;
        }
    }
}

// FILE-MODULE MAPPING

// Rust maps files and folders to modules
mod module_1;

fn test_file_modules () {
    // module_1::module_2::function(); // Cannot do, module_2 is private within module_1
    module_1::module_3::function();
}

fn main() {
    use crate::my_module::public_function; // "use" allows easier access to things
    public_function(); // Like this
    use crate::my_module::PublicStructWithPublicFields as TestStruct; // it also allows renaming with "as"
    let test_struct = TestStruct { public: 12 };

    test_file_modules();
}