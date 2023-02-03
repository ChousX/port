use std::collections::BTreeMap;

#[derive(Clone)]
pub struct UserDate{
    user_name: String,
    email: String,
    password: String
}

type Key = String;
type UserID = usize;
type Link = BTreeMap<Key, UserID>;

pub struct Accounts{
    email: Link,
    user_name: Link,
    data: Vec<UserDate>,
}

impl Accounts{
    pub fn add(&mut self, UserDate { user_name, email, password }: UserDate) -> Result<(),AccountError>{
        //checking if exists
        if self.email.contains_key(&email) || self.user_name.contains_key(&user_name){
            return Err(AccountError::EmailOrUsernameTaken);
        }
        let id = self.data.len();
        self.data.push(UserDate{
            user_name: user_name.clone(),
            email: email.clone(),
            password
        });
        self.user_name.insert(user_name, id);
        self.email.insert(email, id);
        Ok(())
    }

    pub fn login(&self, username: &str, password: &str) -> bool{
        if let Some(id) = self.user_name.get(username){
            let p = &self.data[*id].password;
            if p == username{
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub enum AccountError{
    EmailOrUsernameTaken
}