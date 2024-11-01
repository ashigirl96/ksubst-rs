use ksubst::substitute;
use std::collections::HashMap;

fn main() {
    let template =
        "VAR=${VAR} VAR.=${VAR.} VAR-=${VAR-}\nVAR2=${VAR2} VAR2.=${VAR2.} VAR-=${VAR2-}";
    let mut variables = HashMap::new();
    variables.insert("VAR".to_string(), "hoge".to_string());
    variables.insert("VAR2".to_string(), "".to_string());

    let result = substitute(template, &variables).unwrap();
    println!("{}", result); // Output: hoge hoge. hoge-
}
