
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Somecoolusername123"),
        email: String::from("coolemail@bro.cool"),
        sign_in_count: 1,
    };
    //  Doesn't work because we're immutable. 
    //user1.email = String::from("anothercoolemail@email.cool");
    let newuser = build_user(String::from("mycoolemail@email.cool"), String::from("mycoolusername"));

    println!("New User email: {}", newuser.email);

    // --snip--
    let user2 = User {
        email: String::from("anothercool@email.cool"),
        ..user1
    };
    // user 1 now not useable


    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("Black: {},{},{}", black.0, black.1, black.2);
    println!("Origin: {},{},{}", origin.0, origin.1, origin.2);
    // unit structs with no feilds

    struct AlwaysEqual;

    let subject = AlwaysEqual;


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }

}