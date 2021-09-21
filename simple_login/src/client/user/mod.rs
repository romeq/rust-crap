use std::result::Result;

pub struct User {
    user:       String,
    password:   String,
}

impl User {
    pub fn new(user: String, password: String) -> User {
        User {
            user,
            password
        }
    }
    fn admin_user() -> User {
        User {
            user: String::from("admin"),
            password: String::from("password13")
        }
    }
}

pub fn login(u: &User) -> Result<&str, &str> {
    let admin_account = User::admin_user();

    if u.user != admin_account.user {
        return Err("Invalid username")
    } else if u.password != admin_account.password {
        return Err("Invalid password")
    }


    Ok("Login succeeded.")
}