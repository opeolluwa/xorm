use xorm::IntoModel;
use xorm_macro::IntoModel;

#[derive(IntoModel)]
struct UserInformation {
    name: String,
    age: u64,
}



fn main() {
    // create a new user
    UserInformation::create();
    //find or create a new user
    UserInformation::find_or_create();
    // find user by primary key
    UserInformation::find_by_pk();
    // delete a model record
    UserInformation::destroy();
}
