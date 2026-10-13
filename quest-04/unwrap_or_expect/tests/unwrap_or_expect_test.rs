use unwrap_or_expect::*;

#[test]
fn test_ok() {
    let url = "server.com";

    assert_eq!(fetch_data(Ok(url), Security::Unknown), url);
    assert_eq!(fetch_data(Ok(url), Security::Message), url);
    assert_eq!(fetch_data(Ok(url), Security::Warning), url);
    assert_eq!(fetch_data(Ok(url), Security::NotFound), url);
}

#[test]
fn test_warning() {
    assert_eq!(
        fetch_data(Err("connection failed"), Security::Warning),
        "WARNING: check the server"
    );
}

#[test]
fn test_not_found() {
    assert_eq!(
        fetch_data(Err("server2.com"), Security::NotFound),
        "Not found: server2.com"
    );
}

#[test]
fn test_unexpected_url_err() {
    assert_eq!(
        fetch_data(Err("invalid URL"), Security::UnexpectedUrl),
        "invalid URL"
    );
}

#[test]
#[should_panic]
fn test_unknown_panic() {
    fetch_data(Err("connection failed"), Security::Unknown);
}

#[test]
#[should_panic(expected = "ERROR: program stops")]
fn test_message_panic() {
    fetch_data(Err("connection failed"), Security::Message);
}

#[test]
#[should_panic(expected = "malicious_server.com")]
fn test_unexpected_url_panic() {
    fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
}
