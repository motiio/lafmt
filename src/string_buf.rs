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
