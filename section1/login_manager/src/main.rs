use authentication::get_users;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

fn list_users(){
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");
    let users = get_users();
    users
        .iter()
        .for_each(|(_, user)| {
            println!("{:<20} {:<20?}", user.username, user.role)
    })
}

#[derive(Subcommand)]
enum Commands {
    //List all users
    List,
    //Add User
    Add {
        username: String,
        password: String,
        admin: Option<bool>
    }
}
fn main() {

    let cli = Args::parse();
    match cli.command {
        Some(Commands::Add { username, password, admin }) => {
            println!("Adding user {} with password {}", username, password);
        },
        Some(Commands::List) => {
            list_users();
        },
        None => {
            println!("Run with --help to see instructions.")
        }
    }
}
