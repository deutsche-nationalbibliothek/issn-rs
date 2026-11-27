//! This library provides functions for working with ISSNs. The library
//! focuses on a simple API, high performance, and minimal dependencies
//! on other libraries.
//!
//! Please keep in mind that even if an ISSN is syntactically correct,
//! it is not necessarily associated with a resource or actually
//! assigned by an ISSN National Center.

use std::fmt::{self, Display};
use std::str::FromStr;

use arrayvec::ArrayVec;

/// An International Standard Serial Number (ISSN).
///
/// # Parsing and printing
///
/// The `Issn` type provides the trait implementations of
/// [`std::str::FromStr`] and [`std::fmt::Display`]:
///
/// ```rust
/// use issn::Issn;
///
/// let issn: Issn = "1434-4653".parse()?;
/// assert_eq!(issn.to_string(), "1434-4653");
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, PartialEq)]
pub struct Issn {
    /// The individual digits of an ISSN are stored as a fixed-length
    /// ArrayVec. The individual digits are not converted into their
    /// numerical values in order to ensure fast conversion to a
    /// string.
    ///
    /// It is guaranteed that the first seven digits will always be in
    /// the range from `0` (U+0030) to `9` (U+0039) and that the eighth
    /// position will contain a valid check digit from `0` (U+0030) to
    /// `9` (U+0039) or `X` (U+0058).
    digits: ArrayVec<u8, 8>,
}

impl Issn {
    /// Creates a new ISSN from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use issn::Issn;
    ///
    /// let _issn = Issn::new(b"1434-4653")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn new<B: AsRef<[u8]>>(input: B) -> Result<Self, Error> {
        Parser::new(input.as_ref()).parse()
    }

    /// Creates a new ISSN by specifying all digits (including the
    /// checksum).
    ///
    /// # Note
    ///
    /// This function expects the individual digits to be numbers (`u8`)
    /// that are not in their ASCII representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use issn::Issn;
    ///
    /// let _issn = Issn::from_digits([1, 4, 3, 4, 4, 6, 5, 3])?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_digits(digits: [u8; 8]) -> Result<Self, Error> {
        let digits = ArrayVec::from_iter(
            digits.iter().map(|digit| digit + b'0'),
        );

        for i in 0..8 {
            if !digits[i].is_ascii_digit() {
                return Err(Error {
                    kind: ErrorKind::Digit {
                        value: digits[i],
                        position: i,
                    },
                });
            }
        }

        let issn = Issn { digits };
        issn.validate()?;

        Ok(issn)
    }

    /// Returns the ISSN formatted in a single groups of eight digits
    /// without a separator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use issn::Issn;
    ///
    /// let issn: Issn = "2194-8860".parse()?;
    /// assert_eq!(issn.unhyphenated(), "21948860");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn unhyphenated(&self) -> &str {
        // SAFETY: All individual values come from the ASCII alphabet,
        // and the byte slice can be safely converted to a Unicode
        // string.
        unsafe { str::from_utf8_unchecked(&self.digits) }
    }

    /// This function verifies the validity of the provided ISSN by
    /// checking the checksum in the last position.
    ///
    /// The checksum is calculated as follows: First, the weighted sum
    /// of the first seven digits is calculated using the weights 8, 7,
    /// ..., 2. Finally, the remainder is calculated by dividing by 11
    /// (`mod 11`). If the remainder is `0`, the checksum is ‘0’ (48).
    /// Otherwise, the remainder is subtracted from 11; if the result is
    /// 10, the checksum is b'X' (88); otherwise, it is the ASCII
    /// equivalent of the result.
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn validate(&self) -> Result<(), Error> {
        const WEIGHTS: [u64; 7] = [8, 7, 6, 5, 4, 3, 2];

        let sum = self.digits[0..7]
            .iter()
            .zip(WEIGHTS)
            .map(|(digit, weight)| (*digit - b'0') as u64 * weight)
            .sum::<u64>();

        let r = (sum % 11) as u8;
        let checksum = if r == 0 {
            b'0'
        } else {
            match 11 - r {
                10 => b'X',
                value => b'0' + value,
            }
        };

        if self.digits[7] != checksum {
            return Err(Error {
                kind: ErrorKind::Checksum {
                    value: self.digits[7],
                    checksum,
                },
            });
        }

        Ok(())
    }
}

impl Display for Issn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}",
            unsafe { str::from_utf8_unchecked(&self.digits[0..4]) },
            unsafe { str::from_utf8_unchecked(&self.digits[4..8]) }
        )
    }
}

impl FromStr for Issn {
    type Err = Error;

    /// Parses a ISSN from a string slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use issn::Issn;
    ///
    /// let _issn = Issn::from_str("1434-4653")?;
    /// let _issn: Issn = "1434-4653".parse()?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s.as_bytes()).parse()
    }
}

/// An error that can occur in this crate.
#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

impl std::error::Error for Error {}

/// The underlying kind of a [`Error`].
#[derive(Debug, PartialEq)]
enum ErrorKind {
    // An invalid digit
    Digit { value: u8, position: usize },
    // An invalid checksum digit
    ChecksumDigit { value: u8 },
    // An invalid checksum
    Checksum { value: u8, checksum: u8 },
    // The input is too small
    Length { position: usize },
    // An invalid separator
    Separator { value: u8 },
    // The input too long
    Size { len: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;

        match self.kind {
            Digit { value, position } => write!(
                f,
                "expected ASCII digit at position {position}, but got `{value}`"
            ),
            ChecksumDigit { value } => write!(
                f,
                "expected ASCII digit or 'X' in the last position, but got `{value}`"
            ),
            Checksum { value, checksum } => write!(
                f,
                "invalid checksum, expected `{checksum}`, but got `{value}`",
            ),
            Length { position } => write!(
                f,
                "expected value at position {position}, but found end of input",
            ),
            Separator { value } => {
                write!(
                    f,
                    "expected `-` as separator, but got `{value}`"
                )
            }
            Size { len } => {
                write!(
                    f,
                    "expected input of 9 bytes, but got `{len}` bytes"
                )
            }
        }
    }
}

/// A ISSN parser.
struct Parser<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new ISSN parser.
    fn new<I: ?Sized + AsRef<[u8]>>(input: &'a I) -> Parser<'a> {
        Parser {
            input: input.as_ref(),
            position: 0,
        }
    }

    /// Parses the input and returns an ISSN if successful; otherwise,
    /// it returns an error.
    fn parse(&mut self) -> Result<Issn, Error> {
        let mut digits = ArrayVec::new();

        // parses the first group of four ASCII digits
        digits
            .try_extend_from_slice(self.parse_ascii_digits(4)?)
            .expect("enough capacity");

        // parses the separator '-'
        self.parse_separator()?;

        // parse the second group of four ASCII digits
        digits
            .try_extend_from_slice(self.parse_ascii_digits(3)?)
            .unwrap();

        // parse checksum
        digits.push(self.parse_checksum()?);

        let issn = Issn { digits };

        // perform checksum validation
        issn.validate()?;

        // At this point, a complete, valid ISSN has been parsed, and it
        // is expected that the input contains no further bytes.
        if !self.is_done() {
            return Err(Error {
                kind: ErrorKind::Size {
                    len: self.input.len(),
                },
            });
        }

        Ok(issn)
    }

    /// parses n consecutive ASCII digits
    fn parse_ascii_digits(&mut self, n: usize) -> Result<&[u8], Error> {
        let start = self.position;
        let mut end = start;

        for _ in 0..n {
            self.ensure_not_done()?;
            let byte = self.byte();

            if !byte.is_ascii_digit() {
                return Err(Error {
                    kind: ErrorKind::Digit {
                        value: byte,
                        position: self.position,
                    },
                });
            }

            self.consume();
            end += 1;
        }

        Ok(&self.input[start..end])
    }

    /// Parses the checksum
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn parse_checksum(&mut self) -> Result<u8, Error> {
        self.ensure_not_done()?;
        let value = self.byte();

        if !matches!(value, b'0'..=b'9' | b'X') {
            return Err(Error {
                kind: ErrorKind::ChecksumDigit { value },
            });
        }

        self.consume();
        Ok(value)
    }

    /// Parses the separator b'-'
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn parse_separator(&mut self) -> Result<(), Error> {
        self.ensure_not_done()?;

        if self.byte() != b'-' {
            return Err(Error {
                kind: ErrorKind::Separator { value: self.byte() },
            });
        }

        self.consume();
        Ok(())
    }

    /// Returns true if the end of input is reached
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_done(&self) -> bool {
        self.position == self.input.len()
    }

    /// Returns an error if the end of input is reached, otherwise the
    /// unit type.
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn ensure_not_done(&self) -> Result<(), Error> {
        if self.is_done() {
            return Err(Error {
                kind: ErrorKind::Length {
                    position: self.position,
                },
            });
        }

        Ok(())
    }

    /// Continue to the next byte
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn consume(&mut self) {
        self.position += 1;
    }

    /// Returns the current byte
    ///
    /// This function panics when a position beyond the end of the
    /// input is accessed.
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn byte(&self) -> u8 {
        self.input[self.position]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_issn() {
        macro_rules! parse_success {
            ($input:expr, $bytes:expr) => {
                assert_eq!(
                    Parser::new($input).parse().unwrap(),
                    Issn {
                        digits: ArrayVec::try_from(*$bytes).unwrap(),
                    }
                );
            };
        }

        parse_success!("0294-0000", b"02940000");
        parse_success!("0344-7871", b"03447871");
        parse_success!("0077-5762", b"00775762");
        parse_success!("0002-8703", b"00028703");
        parse_success!("0160-6514", b"01606514");
        parse_success!("0548-3085", b"05483085");
        parse_success!("0160-6506", b"01606506");
        parse_success!("0276-9077", b"02769077");
        parse_success!("0177-8048", b"01778048");
        parse_success!("0942-3869", b"09423869");
        parse_success!("0201-193X", b"0201193X");
    }

    #[test]
    fn test_invalid_digit() {
        macro_rules! parse_error {
            ($input:expr, $value:expr, $position:expr) => {
                assert_eq!(
                    Parser::new($input).parse().unwrap_err(),
                    Error {
                        kind: ErrorKind::Digit {
                            position: $position,
                            value: $value,
                        }
                    }
                );
            };
        }

        parse_error!("a434-4653", b'a', 0);
        parse_error!("1a34-4653", b'a', 1);
        parse_error!("14a4-4653", b'a', 2);
        parse_error!("143a-4653", b'a', 3);
        parse_error!("1434-a653", b'a', 5);
        parse_error!("1434-4a53", b'a', 6);
        parse_error!("1434-46a3", b'a', 7);
    }

    #[test]
    fn test_invalid_separator() {
        macro_rules! parse_error {
            ($input:expr, $value:expr) => {
                assert_eq!(
                    Parser::new($input).parse().unwrap_err(),
                    Error {
                        kind: ErrorKind::Separator { value: $value }
                    }
                );
            };
        }

        parse_error!("1434 4653", b' ');
        parse_error!("1434:4653", b':');
    }

    #[test]
    fn test_invalid_checksum_digit() {
        macro_rules! parse_error {
            ($input:expr, $value:expr) => {
                assert_eq!(
                    Parser::new($input).parse().unwrap_err(),
                    Error {
                        kind: ErrorKind::ChecksumDigit {
                            value: $value
                        }
                    }
                );
            };
        }

        parse_error!("1434-465Y", b'Y');
    }

    #[test]
    fn test_invalid_checksum() {
        macro_rules! parse_error {
            ($input:expr, $value:expr, $checksum:expr) => {
                assert_eq!(
                    Parser::new($input).parse().unwrap_err(),
                    Error {
                        kind: ErrorKind::Checksum {
                            checksum: $checksum,
                            value: $value,
                        }
                    }
                );
            };
        }

        parse_error!("0294-0001", b'1', b'0');
        parse_error!("0344-7872", b'2', b'1');
        parse_error!("0077-5763", b'3', b'2');
        parse_error!("0002-8704", b'4', b'3');
        parse_error!("0160-6515", b'5', b'4');
        parse_error!("0548-3086", b'6', b'5');
        parse_error!("0160-6507", b'7', b'6');
        parse_error!("0276-9078", b'8', b'7');
        parse_error!("0177-8049", b'9', b'8');
        parse_error!("0942-386X", b'X', b'9');
        parse_error!("0201-1930", b'0', b'X');
    }

    #[test]
    fn test_invalid_length() {
        macro_rules! parse_error {
            ($input:expr, $position:expr) => {
                assert_eq!(
                    Parser::new($input).parse().unwrap_err(),
                    Error {
                        kind: ErrorKind::Length {
                            position: $position
                        }
                    }
                );
            };
        }

        parse_error!("", 0);
        parse_error!("1", 1);
        parse_error!("14", 2);
        parse_error!("143", 3);
        parse_error!("1434", 4);
        parse_error!("1434-", 5);
        parse_error!("1434-4", 6);
        parse_error!("1434-46", 7);
        parse_error!("1434-465", 8);
    }

    #[test]
    fn test_invalid_size() {
        macro_rules! parse_error {
            ($input:expr, $length:expr) => {
                assert_eq!(
                    Parser::new($input).parse().unwrap_err(),
                    Error {
                        kind: ErrorKind::Size { len: $length }
                    }
                );
            };
        }

        parse_error!("1434-4653 (ISSN)", 16);
        parse_error!("1434-4653_", 10);
    }
}
