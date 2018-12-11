#[allow(dead_code)] // ignore compiler warnings for unused fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    struct_init_basic();
    struct_init_and_change();

    let _built_user_1 = build_user_struct(
            String::from("test123@gmail.com"), String::from("username1"));
    let _built_user_2 = build_user_struct_with_shorthand(
            String::from("test123@gmail.com"), String::from("username1"));

    struct_init_from_other_instance();

    tuple_structs_no_named_fields_for_types();
}

fn struct_init_basic() {
    let _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

fn struct_init_and_change() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}


fn build_user_struct(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    } // no semicolon here: returns this new user instance!
}

fn build_user_struct_with_shorthand(email: String, username: String) -> User {
    User {
        email, // No value specified - will use 'email' param
        username,
        active: true,
        sign_in_count: 1,
    } // no semicolon here: returns this new user instance!
}

fn struct_init_from_other_instance() {
    // Same user1 as in struct_init_basic().
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Now let's build another instance
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername456"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // We can make this easier by using the syntax .. to fill remaining fields:
    let _user3 = User {
        email: String::from("athird@example.com"),
        username: String::from("athirdusername789"),
        ..user1 // rest of fields should just be same as defined in user1
    };
}

// used by tuple_structs_no_named_fields_for_types().
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs_no_named_fields_for_types() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

// We can also define unit-like structs without any fields, but we'll come back
// to this later on.

// Note about ownership and lifetimes:
// In this file we used Strings rather than &str because of lifetimes (ch 10).
// This means basically that the following will generate compile errors:
// See ch 10 for more info but basically for now, we'll just use owned types
// like String rather than references like &str.

/*
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
*/
