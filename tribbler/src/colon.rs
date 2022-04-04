//! module with functions to escape and unescape strings used in trib

/// Escapes a string with backslashes and colons
///
/// ```rust
/// use tribbler::colon::*;
/// assert_eq!("Testing||Escape|;Sequences", escape("Testing|Escape:Sequences"));
/// assert_eq!("Testing|Escape:Sequences", unescape(escape("Testing|Escape:Sequences")));
/// ```
pub fn escape<T: Into<String>>(s: T) -> String {
    s.into().replace('|', "||").replace(':', "|;")
}

/// Unescapes a string with the values used in [escape]. See [escape] for
/// examples.
pub fn unescape<T: Into<String>>(s: T) -> String {
    let mut out = vec![];
    let mut escaping = false;
    for x in s.into().chars() {
        if !escaping {
            if x == '|' {
                escaping = true;
            } else {
                out.push(x);
            }
        } else {
            if x == ';' {
                out.push(':');
            } else if x == '|' {
                out.push('|');
            } else {
                // should not occur
                out.push(x);
            }
            escaping = false;
        }
    }
    out.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::{escape, unescape};

    fn check(s: &str) {
        assert_eq!(unescape(escape(s)), s);
    }

    #[test]
    fn t1() {
        check("|");
    }
    #[test]
    fn t2() {
        check("||");
    }
    #[test]
    fn t3() {
        check("|||");
    }
    #[test]
    fn t4() {
        check("a|:a");
    }
    #[test]
    fn t5() {
        check("a::a");
    }
    #[test]
    fn t6() {
        check("a:|a");
    }
    #[test]
    fn t7() {
        check("    ");
    }
    #[test]
    fn t8() {
        check("::||::||;;||;;||;:");
    }
}
