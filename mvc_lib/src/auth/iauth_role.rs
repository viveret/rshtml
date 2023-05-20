
// AuthRole interface
pub trait IAuthRole {
    // get the name of the role
    fn get_name(self: &Self) -> String;
}