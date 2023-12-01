pub fn beautify_vec(vec: Vec<String>) -> Vec<String> {
    let mut vec: Vec<String> = vec;
    vec.retain(|x| !x.is_empty()); // removes empty lines
    vec.sort_by(|a, b| { // puts variable declarations first
        if a.starts_with("let") && !b.starts_with("let") {
            return std::cmp::Ordering::Less;
        } else if !a.starts_with("let") && b.starts_with("let") {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    });
    vec.sort_by(|a, b| { // puts comments first
        if a.starts_with("//") && !b.starts_with("//") {
            return std::cmp::Ordering::Less;
        } else if !a.starts_with("//") && b.starts_with("//") {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    });
    return vec;
}

pub fn beautify_statment(statment_vec: Vec<String>, without_comments: bool) -> String {
    let mut new_statment_vec: Vec<String> = Vec::new();
    if without_comments {
        for line in &statment_vec {
            if !line.starts_with("//") {
                new_statment_vec.push("\t".to_owned() + &line.to_owned() + ";");
            }
        }
    } else {
        for line in &statment_vec {
            if !line.starts_with("//") {
                new_statment_vec.push("\t".to_owned() + &line.to_owned() + ";");
            } else {
                new_statment_vec.push("\t".to_owned() + &line.to_string());
            }
        }
    }
    return new_statment_vec.join("\n");
}

pub fn beautify_as_type(line: String, type_: String) -> String{
    return line.replace(&format!(" as {}", type_), "");
}

pub fn beautify_last_line(last_line: String) -> String {
    let mut new_last_line: String = last_line;
    while new_last_line.contains("\t") || new_last_line.contains("\n") || new_last_line.contains("\r") || new_last_line.contains("  ") {
        new_last_line = new_last_line.replace("\t", " ");
        new_last_line = new_last_line.replace("\n", " ");
        new_last_line = new_last_line.replace("\r", " ");
        new_last_line = new_last_line.replace("  ", " ");
    }
    return new_last_line;
}