/// Takes a reference to a String, returns nothing, but
/// prints whether the contents of the String is plural or singular.
pub fn inspect(s: &String) {
    if s.ends_with('s') {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

/// Takes a *mutable* reference to a String and adds an "s" to
/// the String if it doesn't already end with "s".
pub fn change(s: &mut String) {
    if !s.ends_with('s') {
        s.push('s');
    }
}

/// accepts ownership of (consumes) a String and returns a bool
/// indicating whether or not the String both starts with a "b" AND contains an "a".
pub fn eat(s: String) -> bool {
    s.starts_with('b') && s.contains('a')
}

/// Takes a mutable reference to a String and
/// ignores what is in the string and replaces the contents of the string with the String
/// "sparkly".
pub fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
}
