use simple_login::client;
use simple_login::client::user as UserEngine;

fn main() {
    let user: String = client::io::ask_input("Username");
    let password: String = client::io::ask_input("Password");
    let current_user = UserEngine::User::new(
        user,
        password,
    );
    let login = UserEngine::login(&current_user);
    let login_result = match login {
        Ok(_) => "success",
        Err(_) => "failed"
    };

    println!("login {}", login_result);
}
