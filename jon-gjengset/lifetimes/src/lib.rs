// LINK: https://www.youtube.com/watch?v=rAl-9HwD858
#[derive(Debug)]
// What if instead of the delimiter being a string we want it to be anything
// that can find itself a string
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D> 
where 
    D: Delimiter
{
    type Item = &'haystack str;
    
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    let delim = format!("{}", c);
    StrSplit::new(s, &*delim)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("Hello world", 'o'), "Hell");
}

#[test]
fn create_str_split()   {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail()   {
    let haystack = "a b c d ";
    let letters:Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""])
}