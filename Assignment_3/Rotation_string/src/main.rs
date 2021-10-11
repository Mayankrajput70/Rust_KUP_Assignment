fn main() {
    let string = String::from("mayank");
    let replace_from = "mayank";
    let replace_to = "knayam";
    let result = string.replace(replace_from, replace_to);
    println!("{} converted {}.", string, result);
}