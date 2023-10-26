struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}
fn main (){
let user1=build_user(String::from("kariukiamschel9@gmail.com"),String::from("Amschel Kariuki"));
println!("The user is called {}",user1.username);
}
fn build_user(email:String, username:String)->User{
    User{
        active:true,
        username,
        email,
        sign_in_count:1

    }

}