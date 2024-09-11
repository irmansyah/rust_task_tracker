pub fn is_email(email: &str) -> bool {
    /*
     *  email validation:
     *    - must be a valid email
     */

    let re = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    re.is_match(email)
}

pub fn is_password(password: &str) -> bool {
    /*
     *  password validation:
     *    - must be at least 8 characters
     */

    let re = regex::Regex::new(r"^\S{8,}$").unwrap();
    re.is_match(password)
}

pub fn is_username(username: &str) -> bool {
    /*
     *  username validation:
     *    - must be at least 3 characters
     *    - must contain only letters, numbers, and underscores
     *    - must start with a letter
     *    - must not be longer than 20 characters
     */

    let re = regex::Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap();
    re.is_match(username)
}
