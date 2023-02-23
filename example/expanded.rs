#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tokio_postgres;
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
async fn create() -> Result<String, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(
            "postgres://opeolluwa:thunderstorm@localhost/postgres",
            tokio_postgres::NoTls,
        )
        .await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            {
                ::std::io::_eprint(format_args!("connection error: {0}\n", e));
            };
        }
    });
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;
    let value: &str = rows[0].get(0);
    {
        ::std::io::_print(format_args!("{0}\n", value));
    };
    Ok({
        let res = ::alloc::fmt::format(
            format_args!(
                "create a record  {0}",
                "INSERT name, age INTO UserInformation VALUES $VALUES "
            ),
        );
        res
    })
}
fn main() -> Result<(), ()> {
    let body = async {
        let rr = UserInformation::create().await.ok().unwrap();
        {
            ::std::io::_print(format_args!(" the CREATE {0}\n", rr));
        };
        UserInformation::find_or_create();
        UserInformation::find_by_pk();
        UserInformation::destroy();
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
