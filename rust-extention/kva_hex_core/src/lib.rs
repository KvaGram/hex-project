use num::Num;
use num::Signed;

//supported for signed number types.
//Required for full feature support: <T: Signed + Copy + From<i8>> Hex<T>
pub struct Hex<T> {
    pub q:T,
    pub r:T
}
impl<T: Signed + Copy> Hex<T> {
    fn s(&self) -> T {-self.q - self.r}
}
impl<T: Signed> std::ops::Add for Hex<T> {
    type Output;

    fn add(self, rhs: Self) -> Self::Output {
        Hex {
            q : self.q + rhs.q,
            r : self.r + rhs.r
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
