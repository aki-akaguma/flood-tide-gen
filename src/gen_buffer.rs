#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenBuffer(String);
impl GenBuffer {
    pub fn with_capacity(sz: usize) -> Self {
        GenBuffer(String::with_capacity(sz))
    }
    pub fn into_string(self) -> String {
        self.0
    }
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string);
    }
    pub fn push_str_ary(&mut self, ary: &[&str]) {
        for s in ary {
            self.0.push_str(s);
        }
    }
}

impl std::ops::Add<&str> for GenBuffer {
    type Output = GenBuffer;
    #[inline]
    fn add(mut self, other: &str) -> GenBuffer {
        self.push_str(other);
        self
    }
}
impl std::ops::AddAssign<&str> for GenBuffer {
    #[inline]
    fn add_assign(&mut self, other: &str) {
        self.push_str(other);
    }
}

impl std::ops::AddAssign<&[&str]> for GenBuffer {
    #[inline]
    fn add_assign(&mut self, other: &[&str]) {
        self.push_str_ary(other);
    }
}
impl std::ops::AddAssign<&String> for GenBuffer {
    #[inline]
    fn add_assign(&mut self, other: &String) {
        self.push_str(other.as_str());
    }
}
