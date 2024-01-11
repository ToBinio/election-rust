pub fn get_fitting_names(names: &[String], search: &String) -> Vec<String> {
    names
        .iter()
        .filter(|name| name.starts_with(search))
        .map(|name| name.to_string())
        .collect()
}
