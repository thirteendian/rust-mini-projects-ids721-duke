//build a MarcoPolo function that return Polo if you pass in Marco otherwise returns Not Marco
pub fn marco_polo(input: &str) -> String {
    if input == "Marco" {
        "Polo".to_string()
    } else {
        "Not Marco".to_string()
    }
}
