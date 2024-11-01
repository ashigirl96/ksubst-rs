//! Variables substitution in string templates.
//!
//! This library provide helper functions for string manipulation,
//! taking values from a context **env**ironment map and **subst**ituting
//! all matching placeholders.
//!
//! Its name and logic is similar to the [`envsubst`] GNU utility, but
//! this only supports braces-delimited variables (i.e. `${foo}`) and
//! takes replacement values from an explicit map of variables.
//!
//! [`envsubst`]: https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html
//!
//! ## Example
//!
//! ```rust
//! let base_url = "${protocol}://${hostname}/${endpoint}";
//! assert!(envsubst::is_templated(base_url));
//!
//! let mut context = std::collections::HashMap::new();
//! context.insert("protocol".to_string(), "https".to_string());
//! context.insert("hostname".to_string(), "example.com".to_string());
//! context.insert("endpoint".to_string(), "login".to_string());
//! assert!(envsubst::validate_vars(&context).is_ok());
//!
//! let final_url = envsubst::substitute(base_url, &context).unwrap();
//! assert!(!envsubst::is_templated(&final_url));
//! assert_eq!(final_url, "https://example.com/login");
//! ```

#![allow(clippy::implicit_hasher)]

use regex::Regex;
use std::collections::HashMap;

/// Library errors.
#[derive(thiserror::Error, Debug)]
#[error("envsubst error: {0}")]
pub struct Error(String);

/// Substitute variables in a template string with optional suffix handling.
///
/// This function replaces tokens of the form `${VAR}`, `${VAR.}`, `${VAR-}` in the template string.
/// - If the variable `VAR` has a non-empty value, it replaces the placeholder with `value + suffix`.
/// - If the variable `VAR` has an empty value (`""`), it replaces the entire placeholder (including the suffix) with an empty string.
pub fn substitute<T>(template: T, variables: &HashMap<String, String>) -> Result<String, Error>
where
    T: Into<String>,
{
    let mut output = template.into();
    if variables.is_empty() {
        return Ok(output);
    }

    validate_vars(variables)?;

    // Regular expression to match placeholders like ${VAR}, ${VAR.}, ${VAR-}
    let re = Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*)([\.\-][^}]*)?\}").unwrap();

    output = re
        .replace_all(&output, |caps: &regex::Captures| {
            let var_name = caps.get(1).map_or("", |m| m.as_str());
            let suffix = caps.get(2).map_or("", |m| m.as_str());

            if let Some(value) = variables.get(var_name) {
                if !value.is_empty() {
                    format!("{}{}", value, suffix)
                } else {
                    "".to_string()
                }
            } else {
                // If variable is not found, leave the placeholder as is
                caps.get(0).unwrap().as_str().to_string()
            }
        })
        .to_string();

    Ok(output)
}

/// Check whether input string contains templated variables.
pub fn is_templated<S>(input: S) -> bool
where
    S: AsRef<str>,
{
    let re = Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*)([\.\-][^}]*)?\}").unwrap();
    re.is_match(input.as_ref())
}

/// Validate variables for substitution.
///
/// This check whether substitution variables are valid. In order to make
/// substitution deterministic, the following characters are not allowed
/// within variables names nor values: `$`, `{`, `}`.
pub fn validate_vars(variables: &HashMap<String, String>) -> Result<(), Error> {
    for (k, v) in variables {
        validate(k, "key")?;
        validate(v, "value")?;
    }
    Ok(())
}

/// Check whether `value` contains invalid characters.
fn validate<S>(value: S, kind: &str) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let forbidden = &["$", "{", "}"];
    for c in forbidden {
        if value.as_ref().contains(c) {
            let err_msg = format!(
                "variable {} '{}' contains forbidden character '{}'",
                kind,
                value.as_ref(),
                c
            );
            return Err(Error(err_msg));
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn basic_subst() {
        let template = "foo ${VAR} bar";
        let mut env = HashMap::new();
        env.insert("VAR".to_string(), "var".to_string());

        let out = substitute(template, &env).unwrap();
        let expected = "foo var bar";
        assert_eq!(out, expected);
    }

    #[test]
    fn template_check() {
        let plain = "foo";
        assert!(!is_templated(plain));

        let template = "foo ${VAR} bar";
        assert!(is_templated(template));

        let starting = "foo${";
        assert!(!is_templated(starting));

        let ending = "foo}";
        assert!(!is_templated(ending));
    }

    #[test]
    fn basic_empty_vars() {
        let template = "foo ${VAR} bar";
        let env = HashMap::new();

        let out = substitute(template, &env).unwrap();
        assert_eq!(out, template);
    }

    #[test]
    fn dollar_bracket() {
        let template = "foo ${ bar";
        let mut env = HashMap::new();
        env.insert("VAR".to_string(), "var".to_string());

        let out = substitute(template, &env).unwrap();
        assert_eq!(out, template);
    }

    #[test]
    fn invalid_vars() {
        let template = "foo ${VAR} bar";
        let mut env = HashMap::new();
        env.insert("${VAR}".to_string(), "var".to_string());

        substitute(template, &env).unwrap_err();

        let mut env = HashMap::new();
        env.insert("VAR".to_string(), "${VAR}".to_string());

        substitute(template, &env).unwrap_err();
    }

    #[test]
    fn test_substitute_with_suffix_non_empty_var() {
        let template = "${VAR} ${VAR.} ${VAR-}";
        let mut variables = HashMap::new();
        variables.insert("VAR".to_string(), "hoge".to_string());

        let result = substitute(template, &variables).unwrap();
        assert_eq!(result, "hoge hoge. hoge-");
    }

    #[test]
    fn test_substitute_with_suffix_empty_var() {
        let template = "${VAR} ${VAR.} ${VAR-}";
        let mut variables = HashMap::new();
        variables.insert("VAR".to_string(), "".to_string());

        let result = substitute(template, &variables).unwrap();
        assert_eq!(result, "  ");
    }

    #[test]
    fn test_substitute_with_missing_var() {
        let template = "${VAR} ${VAR.} ${VAR-}";
        let variables = HashMap::new();

        let result = substitute(template, &variables).unwrap();
        assert_eq!(result, "${VAR} ${VAR.} ${VAR-}");
    }

    #[test]
    fn test_substitute_with_complex_suffix() {
        let template = "${VAR.suffix} ${VAR-extra}";
        let mut variables = HashMap::new();
        variables.insert("VAR".to_string(), "value".to_string());

        let result = substitute(template, &variables).unwrap();
        assert_eq!(result, "value.suffix value-extra");
    }
}
