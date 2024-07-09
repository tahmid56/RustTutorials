use authentication::{greet_user, read_line, LoginAction, LoginRole, login};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter username");
        let username = read_line();
        println!("Enter password");
        let password = read_line();
        match login(&username, &password){
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => {
                        println!("{}", greet_user(&username));
                        println!("Tried {} times", tries);
                        break;
                    },
                    LoginRole::User => {
                        println!("{}", greet_user(&username));
                        println!("Tried {} times", tries);
                        break;
                    },
                }
            },
            Some(LoginAction::Denied) => {
                println!("Try again");
            },
            None => {
                println!("New User!")
            }
        }
        tries += 1;
        if tries >= 3 {
            println!("Lost Attempt");
            break;
        }
    }
}
