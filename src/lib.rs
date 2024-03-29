//! # simple_url_parser
//!
//! a simple parser for URL
//!
//! ``` rust
//! fn main(){
//!     let mock_url = "https://lb:123456@www.google.com:123/blog/01?a=1&b=2#132456";
//!     let url_obj = URL::parse(mock_url).unwrap();
//!
//!    assert_eq!(url_obj.scheme, "https:");
//!     assert_eq!(url_obj.username, "lb");
//!     assert_eq!(url_obj.password, "123456");
//!     assert_eq!(url_obj.host, "www.google.com");
//!     assert_eq!(url_obj.port, "123");
//!     assert_eq!(url_obj.path, "/blog/01");
//!     assert_eq!(url_obj.query, "?a=1&b=2");
//!     assert_eq!(url_obj.hash, "#132456");
//!
//!     let url_str = URL::stringify(&url_obj);
//!     assert_eq!(url_str, mock_url);
//! }
//! ```
//!
//! ### Thanks
//!
//! [nom](https://github.com/Geal/nom)
//!
//!
//!

use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{alphanumeric0, char};
use nom::combinator::{opt, peek};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

fn key_value(i: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        take_while(|c: char| c.is_alphabetic() || c == '.'),
        opt(char(':')),
        alphanumeric0,
    )(i)
}

fn end_with<'a>(split: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |i| terminated(take_until(split), tag(split))(i)
}

/// URL class
/// format code:
/// [scheme:]//[user[:password]@]host[:port][/path][?query][#hash]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct URL {
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub origin: String,
    pub host: String,
    pub port: String,
    pub path: String,
    pub query: String,
    pub hash: String,
}

impl URL {
    /// parse string to struct
    ///
    /// ### example
    /// ``` rust
    /// URL::parse("https://lb:123456@www.google.com:123/blog/01?a=1&b=2#132456").unwrap();
    /// ```
    pub fn parse(i: &'static str) -> Result<URL, Box<dyn std::error::Error>> {
        let (i, scheme) = URL::parse_scheme(i)?;
        let (i, (username, password)) = URL::parse_username_password(i)?;
        let (i, (host, port)) = URL::parse_host_port(i)?;
        let (i, path) = URL::parse_path(i)?;
        let (i, query) = URL::parse_query(i)?;
        let (_, hash) = URL::parse_hash(i)?;

        let origin = if port.is_empty() {
            host.to_owned()
        } else {
            format!("{}:{}", host, port)
        };

        Ok(URL {
            scheme: scheme.strip_suffix(':').unwrap_or_default().to_owned(),
            username: String::from(username),
            password: String::from(password),
            origin,
            host: String::from(host),
            port: String::from(port),
            path: String::from(path),
            query: String::from(query),
            hash: String::from(hash),
        })
    }

    /// parse struct to string
    ///
    /// ### example
    /// ``` rust
    /// URL::stringify(&url_obj);
    /// ```
    pub fn stringify(obj: &URL) -> String {
        let mut link: String = format!("{}://", obj.scheme);
        if !obj.username.is_empty() {
            link.push_str(&obj.username);
            if !obj.password.is_empty() {
                link.push_str(&format!(":{}@", obj.password));
            }
        }

        format!(
            "{}{}{}{}{}",
            link, obj.origin, obj.path, obj.query, obj.hash
        )
    }

    fn parse_scheme(i: &str) -> IResult<&str, &str> {
        end_with("//")(i)
    }

    fn parse_username_password(i: &str) -> IResult<&str, (&str, &str)> {
        let (i, pattern) = opt(end_with("@"))(i)?;
        if let Some(pattern) = pattern {
            let (_, tulp) = key_value(pattern)?;
            return Ok((i, tulp));
        }
        Ok((i, ("", "")))
    }

    fn parse_host_port(i: &str) -> IResult<&str, (&str, &str)> {
        terminated(key_value, peek(opt(tag("/"))))(i)
    }

    fn parse_path(i: &str) -> IResult<&str, &str> {
        let chars = "#?";
        take_while(move |c| !chars.contains(c))(i)
    }

    fn parse_query(i: &str) -> IResult<&str, &str> {
        preceded(peek(opt(tag("?"))), take_while(|c| c != '#'))(i)
    }

    fn parse_hash(i: &str) -> IResult<&str, &str> {
        preceded(peek(opt(tag("#"))), take_while(|c: char| c != ' '))(i)
    }
}
