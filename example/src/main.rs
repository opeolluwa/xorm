use tokio_postgres;
use xorm::IntoModel;
use xorm_macro::IntoModel;

#[derive(IntoModel)]
#[allow(unused_variables, dead_code)]
struct UserInformation {
    name: String,
    age: u64,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    // create a new user
    let rr = UserInformation::create().await.ok().unwrap();
    println!(" the CREATE {}", rr);
    //find or create a new user
    UserInformation::find_or_create();
    // find user by primary key
    UserInformation::find_by_pk();
    // delete a model record
    UserInformation::destroy();

    Ok(())
}
