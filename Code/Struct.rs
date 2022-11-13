struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,  
}

fn main() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true;
    sign_in_count: 1,
  };
  
  user1.email = String::from("anotheremail@example.com");
  
  let user2 = User {
    email: String::from("anotheremail@example.com"),
    username: user1.username, 
    active: true,
    sign_in_count: user.sign_in_count,
  }
  
}

fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1,
  }
}

fn using_field_init_shorthand(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
