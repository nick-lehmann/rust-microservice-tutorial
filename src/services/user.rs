struct UserService {
    users: RwLock<Vec<User>>,
}

impl UserService {
    pub fn new() -> UserService {
        UserService {
            users: RwLock::new(Vec::new()),
        }
    }

    pub async fn list_users(&self) -> Vec<User> {
        self.users.read().await.clone()
    }

    pub async fn create_user(&self, user: User) -> User {
        self.users.write().await.push(user.clone());
        user
    }
}
