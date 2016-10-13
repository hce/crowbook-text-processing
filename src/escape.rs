// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use common::{NB_CHAR, NB_CHAR_NARROW, NB_CHAR_EM};

use std::borrow::Cow;
use regex::Regex;

/// Escape non breaking spaces for HTML, so there is no problem for
/// displaying them if the font or browser doesn't know what to do
/// with them (particularly the narrow non breaking space which isn't
/// very well supported).
pub fn escape_nb_spaces<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if let Some(first) = input.chars().position(|c| match c {
        NB_CHAR | NB_CHAR_NARROW | NB_CHAR_EM => true,
        _ => false,
    }) {
        let mut chars = input.chars().collect::<Vec<_>>();
        let rest = chars.split_off(first);
        let mut output = chars.into_iter().collect::<String>();
        for c in rest {
            match c {
                NB_CHAR_NARROW => output.push_str(r#"<span class = "nnbsp">&#8201;</span>"#),
                NB_CHAR_EM => output.push_str(r#"<span class = "ensp">&#8194;</span>"#),
                NB_CHAR => output.push_str(r#"<span class = "nbsp">&#160;</span>"#),
                _ => output.push(c),
            }
        }
        Cow::Owned(output)
    } else {
        input.into()
    }
}


/// Escape non breaking spaces for LaTeX, replacing them with `~`.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::escape::escape_nb_spaces_tex;
/// let s = escape_nb_spaces_tex("Des espaces insécables ? Ça alors !");
/// assert_eq!(&s, "Des espaces insécables~? Ça alors~!");
/// ```
pub fn escape_nb_spaces_tex<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if let Some(first) = input.chars().position(|c| match c {
        NB_CHAR | NB_CHAR_NARROW | NB_CHAR_EM => true,
        _ => false,
    }) {
        let mut chars = input.chars().collect::<Vec<_>>();
        let rest = chars.split_off(first);
        let mut output = chars.into_iter().collect::<String>();
        for c in rest {
            match c {
                NB_CHAR_NARROW => output.push('~'),
                NB_CHAR_EM => output.push('~'),
                NB_CHAR => output.push('~'),
                _ => output.push(c),
            }
        }
        Cow::Owned(output)
    } else {
        input.into()
    }
}


/// Escape characters `<`, `>`, and `&`
///
/// **Warning**: this function was written for escaping text in a markdown
/// text processor that is designed to run on a local machine, so the content
/// can actually be trusted, it should *not* be used for untrusted content.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::escape::escape_html;
/// let s = escape_html("<foo> & <bar>");
/// assert_eq!(&s, "&lt;foo&gt; &amp; &lt;bar&gt;");
/// ```
pub fn escape_html<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[<>&]").unwrap();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some((first, _)) = first {
        let len = input.len();
        let mut output = Vec::with_capacity(len + len / 2);
        output.extend_from_slice(input[0..first].as_bytes());
        let rest = input[first..].bytes();
        for c in rest {
            match c {
                b'<' => output.extend_from_slice(b"&lt;"),
                b'>' => output.extend_from_slice(b"&gt;"),
                b'&' => output.extend_from_slice(b"&amp;"),
                _ => output.push(c),
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}

/// Escape quotes
///
/// Simply replace `"` by `'`
pub fn escape_quotes<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if input.contains('"') {
        let mut output = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '"' => output.push('\''),
                _ => output.push(c),
            }
        }
        Cow::Owned(output)
    } else {
        input
    }
}


/// Escape characters for LaTeX
///
/// # Example
///
/// ```
/// use crowbook_text_processing::escape::escape_tex;
/// let s = escape_tex("command --foo # calls command with option foo");
/// assert_eq!(&s, r"command -{}-foo \# calls command with option foo");
/// ```
pub fn escape_tex<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    const REGEX_LITERAL: &'static str = r"[&%$#_\x7E\x2D\{\}\^\\]";
    lazy_static! {
       static ref REGEX: Regex = Regex::new(REGEX_LITERAL).unwrap();
    }

    let first = REGEX.find(&input);
    if let Some((first, _)) = first {
        let len = input.len();
        let mut output = Vec::with_capacity(len + len / 2);
        output.extend_from_slice(input[0..first].as_bytes());
        let mut bytes: Vec<_> = input[first..].bytes().collect();
        bytes.push(b' '); // add a dummy char for call to .windows()
        // for &[c, next] in chars.windows(2) { // still experimental, uncomment when stable
        for win in bytes.windows(2) {
            let c = win[0];
            let next = win[1];
            match c {
                b'-' => {
                    if next == b'-' {
                        // if next char is also a -, to avoid tex ligatures
                        output.extend_from_slice(br"-{}");
                    } else {
                        output.push(c);
                    }
                }
                b'&' => output.extend_from_slice(br"\&"),
                b'%' => output.extend_from_slice(br"\%"),
                b'$' => output.extend_from_slice(br"\$"),
                b'#' => output.extend_from_slice(br"\#"),
                b'_' => output.extend_from_slice(br"\_"),
                b'{' => output.extend_from_slice(br"\{"),
                b'}' => output.extend_from_slice(br"\}"),
                b'~' => output.extend_from_slice(br"\textasciitilde{}"),
                b'^' => output.extend_from_slice(br"\textasciicircum{}"),
                b'\\' => output.extend_from_slice(br"\textbackslash{}"),
                _ => output.push(c),
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}


#[test]
fn html_escape_nothing() {
    let s = "Some string without any character to escape";
    let result = escape_html(s);
    assert_eq!(s, &result);
}

#[test]
fn tex_escape_nothing() {
    let s = "Some string without any character to escape";
    let result = escape_tex(s);
    assert_eq!(s, &result);
}

#[test]
fn nb_spaces_escape_nothing() {
    let s = "Some string without any character to escape";
    let result = escape_nb_spaces(s);
    assert_eq!(s, &result);
}

#[test]
fn tex_nb_spaces_escape_nothing() {
    let s = "Some string without any character to escape";
    let result = escape_nb_spaces_tex(s);
    assert_eq!(s, &result);
}

#[test]
fn quotes_escape_nothing() {
    let s = "Some string without any character to escape";
    let result = escape_quotes(s);
    assert_eq!(s, &result);
}

#[test]
fn html_escape_1() {
    let s = "<p>Some characters need escaping & something</p>";
    let expected = "&lt;p&gt;Some characters need escaping &amp; something&lt;/p&gt;";
    let actual = escape_html(s);
    assert_eq!(expected, &actual);
}

#[test]
fn html_escape_2() {
    let actual = escape_html("<foo> & <bar>");
    let expected = "&lt;foo&gt; &amp; &lt;bar&gt;";
    assert_eq!(&actual, expected);
}

#[test]
fn tex_escape_braces() {
    let actual = escape_tex(r"\foo{bar}");
    let expected = r"\textbackslash{}foo\{bar\}";
    assert_eq!(&actual, expected);
}

#[test]
fn tex_escape_dashes() {
    let actual = escape_tex("--foo, ---bar");
    let expected = r"-{}-foo, -{}-{}-bar";
    assert_eq!(&actual, expected);
}

#[test]
fn tex_escape_numbers() {
    let actual = escape_tex(r"30000$ is 10% of number #1 income");
    let expected = r"30000\$ is 10\% of number \#1 income";
    assert_eq!(&actual, expected);
}

#[test]
fn quotes_escape() {
    let actual = escape_quotes(r#"Some text with "quotes""#);
    let expected = r#"Some text with 'quotes'"#;
    assert_eq!(&actual, expected);
}

#[test]
fn nb_spaces_escape() {
    let actual = escape_nb_spaces("This contains non breaking spaces");
    let expected = "This<span class = \"nbsp\">&#160;</span>contains\
                    <span class = \"nnbsp\">&#8201;</span>non breaking spaces";
    assert_eq!(&actual, expected);
}
