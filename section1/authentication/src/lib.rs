use std::collections::HashMap;

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
} 

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Std failed to acquire data");
    input.trim().to_string()
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Clone, Debug)]
pub struct User {
    username: String,
    password: String,
    role: LoginRole,
}

impl User{
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users() -> HashMap<String, User>{
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    return users;
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
    let users = get_users();
    
    // WHILE USING HASHMAP
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        }else {
            return Some(LoginAction::Denied)
        }
    }

    //WHILE USING VECTOR
    // if let Some(user) = users.iter().find(|user| user.username == username){
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone()));
    //     }else {
    //         return Some(LoginAction::Denied)
    //     }
    // }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user(){
        assert_eq!("Hello User1", greet_user("User1"))
    }
}
