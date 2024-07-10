use std::{collections::HashMap, path::Path};
use serde::{Deserialize, Serialize};

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
} 

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Std failed to acquire data");
    input.trim().to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    password: String,
    pub role: LoginRole,
}

impl User{
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

pub fn save_user(user: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&user).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_default_users() -> HashMap<String, User>{
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("user".to_string(), User::new("user", "password", LoginRole::User));
    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists(){
        let users_json: String = std::fs::read_to_string(users_path).unwrap();
        serde_json::from_str(&users_json).unwrap()
    }else {
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("user", "password", LoginRole::User),
//     ]
// }

// fn get_admin_users(){
//     let users: Vec<String> = get_users()
//         .into_iter()
//         .filter(|u| u.role == LoginRole::Admin)
//         .map(|u| u.username)
//         .collect();
// }

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);
    let users = get_users();

    // WHILE USING HASHMAP
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        }else {
            return Some(LoginAction::Denied)
        }
    }
    None
    // WHILE USING VECTOR
    // if let Some(user) = users.iter().find(|user| user.username == username){
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone()));
    //     }else {
    //         return Some(LoginAction::Denied)
    //     }
    // }
    // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user(){
        assert_eq!("Hello User1", greet_user("User1"))
    }
}
