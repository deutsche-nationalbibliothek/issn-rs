use std::str::FromStr;

use issn::Issn;

type TestResult = Result<(), issn::Error>;

#[test]
fn test_issn_new() -> TestResult {
    let _issn = Issn::new(b"2191-4664")?;
    let _issn = Issn::new(b"0720-6763")?;

    assert!(Issn::new(b"2191-4665").is_err());
    Ok(())
}

#[test]
fn test_issn_unhyphenated() -> TestResult {
    let issn = Issn::new(b"2191-4664")?;
    assert_eq!(issn.unhyphenated(), "21914664");
    Ok(())
}

#[test]
fn test_issn_to_string() -> TestResult {
    let issn = Issn::new(b"2191-4664")?;
    assert_eq!(issn.to_string(), "2191-4664");
    Ok(())
}

#[test]
fn test_issn_from_str() -> TestResult {
    let _issn = Issn::from_str("2191-4664")?;
    assert!(Issn::from_str("2191-4665").is_err());
    Ok(())
}

#[test]
fn test_issn_partial_eq() -> TestResult {
    let lhs = Issn::from_str("2191-4664")?;
    let rhs = Issn::new(b"2191-4664")?;

    assert_eq!(lhs, rhs);
    Ok(())
}
