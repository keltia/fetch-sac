//! This module provides parsing functionality to extract structured data from HTML tables
//! containing SAC (System Area Code) information. It uses nom for parsing HTML elements
//! and scraper for higher-level HTML document traversal.
//!

use anyhow::Result;
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_until},
    character::complete::multispace0,
    sequence::{delimited, terminated},
    IResult, Parser,
};
use scraper::{Html, Selector};

/// Parses content within HTML elements, handling both strong tags and plain text.
///
/// # Arguments
///
/// * `input` - A string slice containing the content to parse
///
fn parse_content(input: &str) -> IResult<&str, &str> {
    alt((parse_strong, take_until("<"))).parse(input)
}

/// Parses content within table cells (td or th elements).
///
/// # Arguments
///
/// * `input` - A string slice containing the table cell to parse
///
fn parse_td(input: &str) -> IResult<&str, &str> {
    terminated(
        alt((
            delimited(tag_no_case("<td>"), parse_content, tag_no_case("</td>")),
            delimited(tag_no_case("<th>"), parse_content, tag_no_case("</th>")),
        )),
        multispace0,
    )
    .parse(input)
}

/// Parses content within strong tags.
///
/// # Arguments
///
/// * `input` - A string slice containing the strong element to parse
///
fn parse_strong(input: &str) -> IResult<&str, &str> {
    delimited(
        tag_no_case("<strong>"),
        parse_content,
        tag_no_case("</strong>"),
    )
    .parse(input)
}

fn parse_two(input: &str) -> IResult<&str, (&str, &str)> {
    (parse_td, parse_td).parse(input)
}

fn parse_three(input: &str) -> IResult<&str, (&str, &str)> {
    terminated((parse_td, parse_td), parse_td).parse(input)
}

/// Parses content within span tags.
///
/// # Arguments
///
/// * `input` - A string slice containing the span element to parse
///
fn parse_span(input: &str) -> IResult<&str, &str> {
    delimited(tag_no_case("<span>"), parse_content, tag_no_case("</span>")).parse(input)
}

/// Parses a complete HTML table row (`<tr>`) containing SAC information.
///
/// Takes a string containing an HTML table row and returns a tuple of two strings:
/// the SAC code and the corresponding country/area name.
///
/// # Arguments
///
/// * `input` - A string slice containing the HTML table row to parse
///
pub fn parse_tr(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        terminated(tag_no_case("<tr>"), multispace0),
        alt((parse_three, parse_two)),
        terminated(tag_no_case("</tr>"), multispace0),
    )
    .parse(input)
}

/// Extracts header information from an HTML document.
///
/// Parses the document to find span elements and advanced-title fields,
/// returning their text content as a vector of strings.
///
/// # Arguments
///
/// * `input` - Reference to an HTML document to parse
///
/// # Returns
///
/// A Result containing a vector of extracted header strings
///
pub fn parse_header(input: &Html) -> Result<Vec<String>> {
    let sel = Selector::parse("a > span, [class=field--type-advanced-title]").unwrap();
    let doc = input.select(&sel);
    let r = doc
        .filter(|e| !e.html().contains("class"))
        .map(|e| {
            let frag = e.html();
            let (_, r) = parse_span(&frag).unwrap();
            debug!("{}", r);
            r.to_owned()
        })
        .collect::<Vec<_>>();
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("<td>foo</td>", "foo")]
    #[case("<td><strong>foo</strong></td>", "foo")]
    #[case(
        "<td><strong>Binary Representation</strong></td>",
        "Binary Representation"
    )]
    fn test_parse_td(#[case] input: &str, #[case] res: &str) {
        let (_, r) = parse_td(input).unwrap();
        assert_eq!(res, r)
    }

    #[rstest]
    #[case("<span>foo</span>", "foo")]
    #[case("<span>EU Region</span>", "EU Region")]
    #[case("<span><strong>foo</strong></span>", "foo")]
    fn test_parse_span(#[case] input: &str, #[case] res: &str) {
        let (_, r) = parse_span(input).unwrap();
        assert_eq!(res, r)
    }

    #[test]
    fn test_parse_two() {
        let input = "<td>foo</td><td>bar</td>";

        let (_, (a, b)) = parse_two(input).unwrap();
        assert_eq!("foo", a);
        assert_eq!("bar", b);
    }

    #[test]
    fn test_parse_three() {
        let input = "<td>foo</td><td>bar</td><td>non</TD>";

        let (_, (a, b)) = parse_three(input).unwrap();
        assert_eq!("foo", a);
        assert_eq!("bar", b);
    }

    #[test]
    fn test_parse_tr_0() {
        let input = "<tr><td>foo</td><td>bar</td><td>non</td></tr>";

        let r = parse_tr(input);
        dbg!(&r);
        assert!(r.is_ok());

        let (_, (a, b)) = r.unwrap();
        assert_eq!("foo", a);
        assert_eq!("bar", b);
    }

    #[test]
    fn test_parse_tr_1() {
        let input = "<tr>\n\
        <td>foo</td><td>bar</td><td>non</td>\n\
        </tr>";

        let r = parse_tr(input);
        dbg!(&r);
        assert!(r.is_ok());

        let (_, (a, b)) = r.unwrap();
        assert_eq!("foo", a);
        assert_eq!("bar", b);
    }

    #[test]
    fn test_parse_tr_2() {
        let input = "<tr>\n\
        <td>foo</td><td>bar</td>\n\
        </tr>";

        let r = parse_tr(input);
        dbg!(&r);
        assert!(r.is_ok());

        let (_, (a, b)) = r.unwrap();
        assert_eq!("foo", a);
        assert_eq!("bar", b);
    }

    #[test]
    fn test_parse_tr_3() {
        let input = "<tr>\n\
        <td>foo</td>\n\
        <td>bar</td>\n\
        <td>non</td>\n\
        </tr>";

        let r = parse_tr(input);
        dbg!(&r);
        assert!(r.is_ok());

        let (_, (a, b)) = r.unwrap();
        assert_eq!("foo", a);
        assert_eq!("bar", b);
    }

    #[test]
    fn test_parse_tr_4() {
        let input = r##"<tr><td>94</td><td>Vietnam</td><td>1001 0100</td></tr>"##;

        let r = parse_tr(input);
        dbg!(&r);
        assert!(r.is_ok());

        let (_, (a, b)) = r.unwrap();
        assert_eq!("94", a);
        assert_eq!("Vietnam", b);
    }

    #[test]
    fn test_parse_tr_5() {
        let input = r##"<tr><td>94</td>
        <td>Vietnam</td>
        <td>1001 0100</td>
        </tr>"##;

        let r = parse_tr(input);
        dbg!(&r);
        assert!(r.is_ok());

        let (_, (a, b)) = r.unwrap();
        assert_eq!("94", a);
        assert_eq!("Vietnam", b);
    }

    #[test]
    fn test_parse_tr_6() {
        let input = r##"<tr><th>SAC(Hexa)</th>
    <th>Country/Geographical Area</th>
    <th>Binary Representation</th>
    </tr>
    "##;

        let r = parse_tr(input);
        dbg!(&r);
        assert!(r.is_ok());

        let (_, (a, b)) = r.unwrap();
        assert_eq!("SAC(Hexa)", a);
        assert_eq!("Country/Geographical Area", b);
    }
}
