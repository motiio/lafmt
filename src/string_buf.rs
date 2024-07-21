pub struct StringBuf<'a> {
    buf: &'a str,
}

impl<'a> StringBuf<'a> {
    pub fn new(buf: &str) -> StringBuf {
        StringBuf { buf }
    }
    pub fn buf(&self) -> &'a str {
        self.buf
    }
    pub fn iter(&'a self) -> StringBufIterator {
        StringBufIterator {
            string_buf: self,
            pos: 0,
        }
    }
    pub fn iter_from(&'a self, pos: usize) -> StringBufIterator {
        if pos > self.buf.len() {
            panic!("Iter position index out of bounds")
        }
        StringBufIterator {
            string_buf: self,
            pos,
        }
    }
}

pub struct StringBufIterator<'a> {
    string_buf: &'a StringBuf<'a>,
    pos: usize,
}

impl<'a> StringBufIterator<'a> {
    pub fn prev(&self) -> Option<char> {
        self.string_buf.buf[..self.pos].chars().rev().next()
    }

    pub fn curr(&self) -> Option<char> {
        self.string_buf.buf[self.pos..].chars().next()
    }

    pub fn fetch_to_delim(&mut self, delimiter: &str) -> Option<&'a str> {
        if let Some(delim_pos) = self.string_buf.buf[self.pos..].find(delimiter) {
            let s = &self.string_buf.buf[self.pos..self.pos + delim_pos];
            self.pos += delim_pos + delimiter.len();
            Some(s)
        } else {
            self.pos = self.string_buf.buf.len();
            None
        }
    }
}

impl<'a> Iterator for StringBufIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.string_buf.buf.len() {
            return None;
        }

        if let Some(chr) = self.curr() {
            self.pos += chr.len_utf8();
            return Some(chr);
        }
        None
    }
}

#[cfg(test)]
mod test {

    use std::result;

    use super::StringBuf;

    #[test]
    pub fn test_normal_buff_iter() {
        let query = "select * from kek;\n";

        let buff = StringBuf::new(&query);

        let result = buff.iter().next();
        assert_eq!(Some('s'), result);

        let result = buff.iter().fetch_to_delim(" ");
        assert_eq!(Some("select"), result);

        let result = buff.iter().curr();
        assert_eq!(Some('s'), result);

        let buff_iter = buff.iter();

        let _ = buff_iter.curr();
        let result = buff_iter.curr();
        assert_eq!(Some('s'), result);

        let mut buff_iter = buff.iter();
        let _ = buff_iter.next();
        let result = buff_iter.next();
        assert_eq!(Some('e'), result);

        let mut buff_iter = buff.iter_from(3);
        let result = buff_iter.next();
        assert_eq!(Some('e'), result);

        let buff_iter = buff.iter_from(3);
        let result = buff_iter.prev();
        assert_eq!(Some('l'), result);

        let buff_iter = buff.iter_from(3);
        let _ = buff_iter.prev();
        let result = buff_iter.curr();
        assert_eq!(Some('e'), result);

        let mut buff_iter = buff.iter_from(9);
        let result = buff_iter.fetch_to_delim(" ");
        assert_eq!(Some("from"), result);

        let mut buff_iter = buff.iter_from(0);
        let result = buff_iter.fetch_to_delim("\n");
        assert_eq!(Some("select * from kek;"), result);

        let mut buff_iter = buff.iter_from(0);
        for _ in &mut buff_iter {}
        let result = buff_iter.curr();
        assert_eq!(None, result);
    }

    #[test]
    pub fn test_empty_buff_iter() {
        let query = "";

        let buff = StringBuf::new(&query);

        let result = buff.iter().next();
        assert_eq!(None, result);

        let result = buff.iter().fetch_to_delim(" ");
        assert_eq!(None, result);

        let result = buff.iter().curr();
        assert_eq!(None, result);
    }



    #[test]
    #[should_panic(expected = "Iter position index out of bounds")]
    pub fn test_empty_buff_iter_at_panic() {
        let query = "";

        let buff = StringBuf::new(&query);

        let _ = buff.iter_from(7).curr();
    }
}
