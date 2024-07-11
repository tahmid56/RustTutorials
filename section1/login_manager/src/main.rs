use authentication::{get_users, save_user, LoginRole, User, hash_password};
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

fn add_user(username: String, password: String, admin: bool){
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    }else {
        LoginRole::User
    };
    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_user(users);
}

fn delete_user(username: String){
    let mut users = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_user(users);
    }else {
        println!("{username} doesn't exist!");
    }
}

fn edit_user_password(username: String, new_password: String){
    let mut users = get_users();
    if users.contains_key(&username){
        if let Some(user) = users.get_mut(&username) {
            user.password = hash_password(&new_password);
            save_user(users);
        }
    }else{
        println!("{username} doesn't exist");
    }

}

#[derive(Subcommand)]
enum Commands {
    //List all users
    List,
    //Add User
    Add {
        username: String,
        password: String,
        #[arg(long)]
        admin: Option<bool>
    },
    ChangePassword{
        username: String,
        password: String,
    },
    // Delete a user
    Delete {
        username: String
    }
}
fn main() {

    let cli = Args::parse();
    match cli.command {
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        },
        Some(Commands::List) => {
            list_users();
        },
        Some(Commands::Delete { username }) => {
            delete_user(username);
        },
        Some(Commands::ChangePassword { username, password }) => {
            edit_user_password(username, password);
        },
        None => {
            println!("Run with --help to see instructions.")
        }
    }
}
