use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HASH_STRIP: Regex = Regex::new(r#"::\w+$"#).unwrap();
}

pub fn format_fn_name(fn_name: &str) -> String {
    let mut new_name = String::from("");
    //let hash_strip = Regex::new(r#"::\w+$"#).unwrap();

    let demangle_name = match rustc_demangle::try_demangle(&fn_name) {
        Ok(name) => name.to_string(),
        Err(_) => fn_name.to_string(),
    };

    new_name = fn_name.to_string();
    //new_name = demangle_name;
    //new_name = HASH_STRIP.replace(&demangle_name, "").to_string();
    new_name = new_name
        .replace(".", "")
        .replace("<", "_")
        .replace(">", "_")
        .replace("&", "_")
        .replace("+", "_")
        .replace(",", "_")
        .replace("::", "cc")
        .replace(":", "c")
        .replace("*", "s")
        .replace("%", "p")
        .replace("~", "t")
        .replace("/", "fs")
        .replace("\\", "bs")
        .replace(" ", "_")
        .replace("{", "")
        .replace("}", "")
        .replace("(", "")
        .replace(")", "")
        .replace("[", "")
        .replace("]", "")
        .replace("=", "eq")
        .replace("|", "_")
        .replace("@", "at")
        .replace("#", "h")
        .replace("-", "_");
    // this is needed since some compilers emit just numbers for fnames
    format!("func_{}", new_name)
}
