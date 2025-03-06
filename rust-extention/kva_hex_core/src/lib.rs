use num::Num;
use num::Signed;

pub const DIRECTIONS: [Hex<i8>; 12] = [
    // flat-side directions (neighbors)
    Hex {q: 1,r:-1},
    Hex {q: 1,r: 0},
    Hex {q: 0,r: 1},
    Hex {q:-1,r: 1},
    Hex {q:-1,r: 0},
    Hex {q: 0,r:-1},
    // pointy directions (neibour of neighbor)
    Hex { q: 1, r: -2 },
    Hex { q: 2, r: -1 },
    Hex { q: 1, r: 1 },
    Hex { q: -1, r: 2 },
    Hex { q: -2, r: 1 },
    Hex { q: -1, r: -1 },
];

/// Directions for a hexagon oriented with a pointy north.
/// First six directions are flat-side directions, with immidiate neibors.
/// Final six directions are pointy-side directions with indirect neibors.
pub enum Pointy{
    NorthEast=0,
    East=1,
    SouthEast=2,
    SouthWest=3,
    West=4,
    NorthWest=5,

    North=6,
    EastNorth=7,
    EastSouth=8,
    South=9,
    WestSouth=10,
    WestNorth=11
}
pub enum Flat{
    North=0,
    EastNorth=1,
    EastSouth=2,
    South=3,
    WestSouth=4,
    WestNorth=5,

    NorthEast=6,
    East=7,
    SouthEast=8,
    SouthWest=9,
    West=10,
    NorthWest=11
}
impl Pointy {
    fn get(self:Pointy)->Hex<i8> {
        get_dir_all(self as i32)
    }
}
impl Flat {
    fn get(self:Flat)->Hex<i8> {
        get_dir_all(self as i32)
    }
}
fn get_dir_all(d:i32)->Hex<i8> {
    let mut d = d;
    while d > 12{d -= 12}
    while d < 0 {d += 12}
    let d:usize = usize::try_from(d).expect("d should be wrapped to range (0 .. 12)");
    return DIRECTIONS[d];
}
fn get_dir(d:i32)->Hex<i8> {
    let mut d = d;
    while d > 6 {d -= 6}
    while d < 0 {d += 6}
    let d:usize = usize::try_from(d).expect("d should be wrapped to range (0 .. 6)");
    return DIRECTIONS[d];
}
fn radian_get_dir(r:f64)->Hex<i8> {
    let d:i32 = (6.0 * r / 6.28).round() as i32;
    return get_dir(d);
}
fn radian_get_dir_all(r:f64)->Hex<i8> {
    let d:i32 = (12.0 * r / 6.28).round() as i32;
    return get_dir_all(d);
}

//supported for signed number types.
//Required for full feature support: <T: Signed + Copy + From<i8>> Hex<T>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hex<T> {
    pub q:T,
    pub r:T
}
impl<T> Hex<T> where 
T: Signed + Copy {
    fn s(&self) -> T {-self.q - self.r}
}
impl<T> std::ops::Add for Hex<T> where 
T: Signed + Copy {
    type Output = Hex<T>;
    fn add(self, rhs: Self) -> Self::Output {Hex{
        q : self.q + rhs.q,
        r : self.r + rhs.r
    }}
}
impl<T> std::ops::Sub for Hex<T> where 
T: Signed + Copy {
    type Output = Hex<T>;
    fn sub(self, rhs: Self) -> Self::Output {Hex{
        q : self.q - rhs.q,
        r : self.r - rhs.r
    }}
}
impl<T> std::ops::Mul<T> for Hex<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Hex<T>;

    fn mul(self, rhs: T) -> Self::Output {Hex{
        q : self.q * rhs,
        r : self.r * rhs
    }}
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
