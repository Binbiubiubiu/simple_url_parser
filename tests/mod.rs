use simple_url_parser::URL;

#[test]
fn test_url_parser() {
    let mock_url = "https://lb:123456@www.google.com:123/blog/01?a=1&b=2#132456";
    let url_obj = URL::parse(mock_url).unwrap();

    assert_eq!(url_obj.scheme, "https");
    assert_eq!(url_obj.username, "lb");
    assert_eq!(url_obj.password, "123456");
    assert_eq!(url_obj.host, "www.google.com");
    assert_eq!(url_obj.port, "123");
    assert_eq!(url_obj.path, "/blog/01");
    assert_eq!(url_obj.query, "?a=1&b=2");
    assert_eq!(url_obj.hash, "#132456");

    let url_str = URL::stringify(&url_obj);
    assert_eq!(url_str, mock_url);
}

#[test]
fn test_fix_issue_3() {
    let mock_url = "http://example.org";
    let url_obj = URL::parse(mock_url).unwrap();

    assert_eq!(url_obj.scheme, "http:");
    assert_eq!(url_obj.host, "example.org");

    let url_str = URL::stringify(&url_obj);
    assert_eq!(url_str, mock_url);
}
