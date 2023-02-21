use xorm::IntoModel;
use xorm_macro::IntoModel;

#[derive(IntoModel)]
struct UserInformation;

fn main() {
    UserInformation::find_by_pk();
}
