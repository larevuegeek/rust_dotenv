


pub fn parse_var(value: String) -> String {

    if value.starts_with('"') && value.ends_with('"') {
        return value[1..value.len()-1].to_string();
    }

    value
}