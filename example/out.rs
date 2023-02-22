#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use xorm::IntoModel;
use xorm_macro::IntoModel;
#[allow(unused_variables, dead_code)]
struct UserInformation {
    name: String,
    age: u64,
}
impl IntoModel for UserInformation {
    fn find_by_pk() {
        {
            ::std::io::_print(
                format_args!("{0}\n", "SELECT name, age FROM UserInformation"),
            );
        };
    }
    fn find_or_create() {
        {
            ::std::io::_print(
                format_args!(
                    "{0}\n",
                    "INSERT INTO UserInformation SELECT \"name, age\" FROM ( SELECT * FROM UserInformation WHERE $UNIQUE_KEY)"
                ),
            );
        };
    }
    fn destroy() {
        {
            ::std::io::_print(
                format_args!(
                    " delete a record {0}\n",
                    "DELETE FROM UserInformation WHERE $CONDITION"
                ),
            );
        };
    }
}
fn create() {
    {
        ::std::io::_print(
            format_args!(
                " delete a record {0}\n",
                "INSERT name, age INTO UserInformation VALUES $VALUES "
            ),
        );
    };
}
fn main() {
    UserInformation::create();
    UserInformation::find_or_create();
    UserInformation::find_by_pk();
    UserInformation::destroy();
}
