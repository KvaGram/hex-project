use num::Num;
use num::Signed;

pub mod direction{
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
        /// north-east
        NorthEast=0,
        /// east
        East=1,
        /// south-east
        SouthEast=2,
        /// south-west
        SouthWest=3,
        /// west
        West=4,
        /// north-west
        NorthWest=5,

        ///Diaginal(2 tiles) north
        DNorth=6,
        ///Diaginal(2 tiles) north-east
        DNorthEast=7,
        ///Diaginal(2 tiles) south-east
        DSouthEast=8,
        ///Diaginal(2 tiles) south
        DSouth=9,
        ///Diaginal(2 tiles) south-west
        DSouthWest=10,
        ///Diaginal(2 tiles) north-west
        DNorthWest=11
    }
    pub enum Flat{
        /// north
        North=0,
        /// north-east 
        NorthEast=1,
        /// south-east
        SouthEast=2,
        /// south
        South=3,
        /// south-west 
        SouthWest=4,
        /// north-west
        NorthWest=5,

        ///Diaginal(2 tiles) north-east
        DNorthEast=6,
        ///Diaginal(2 tiles) east
        DEast=7,
        ///Diaginal(2 tiles) south-east
        DSouthEast=8,
        ///Diaginal(2 tiles) south-west
        DSouthWest=9,
        ///Diaginal(2 tiles) west
        DWest=10,
        ///Diaginal(2 tiles) north-west
        DNorthWest=11
    }
    impl Pointy {
        ///Returns Hexagon coordinate for this direction
        fn get(self:Pointy)->Hex<i8> {
            get_dir_all(self as i32)
        }
    }
    impl Flat {
        ///Returns Hexagon coordinate for this direction
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
    /// the s coordinate.
    fn s(&self) -> T {-self.q - self.r}
}
///addition for hexagon coordinates
impl<T> std::ops::Add for Hex<T> where 
T: Signed + Copy {
    type Output = Hex<T>;
    fn add(self, rhs: Self) -> Self::Output {Hex{
        q : self.q + rhs.q,
        r : self.r + rhs.r
    }}
}
///subtraction for hexagon coordiantes
impl<T> std::ops::Sub for Hex<T> where 
T: Signed + Copy {
    type Output = Hex<T>;
    fn sub(self, rhs: Self) -> Self::Output {Hex{
        q : self.q - rhs.q,
        r : self.r - rhs.r
    }}
}
/// multiplication for hexagon coordiantes
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


pub mod spiral
{
    use num::{integer::Roots, Integer};

    use super::*;
    fn spiral_index_to_hex(index:usize)->Hex<i32>{
        if index <= 0 {
            return Hex{q:0, r:0};
        }
        let mut spiral = Spiral{layer:0, posision:0};
        //Layer around the origin tile.
        spiral.layer = (((12 * index + 9).sqrt() - 3)/6.0).ceil();
        //The tile's index, minus tumber of tiles before this layer.
        spiral.posision = index - (3 * spiral.layer * (spiral.layer - 1) + 1);
        let segment = spiral.posision/spiral.layer;
        return direction::get_dir(segment) * layer + direction::get_dir(segment+2) * (spiral.posision % spiral.layer)

/*	var count: int = 3 * layer * (layer - 1) + 1  # Count of tiles in all previous layers

	#clockwise posision arond the layer circle, defined with segment then clockwise position from segment orign
	var segment: int = floor((ti-count) / layer) #segment of the layer (0 to 5)
	var pos: int = (ti-count) % layer #segment posision

	return segposlayer_to_QRS([segment, pos, layer])
	#var q = get_dir(segment)[0] * layer + get_dir(segment+2)[0]*pos
	#var r = get_dir(segment)[1] * layer + get_dir(segment+2)[1]*pos
	#var s = get_dir(segment)[2] * layer + get_dir(segment+2)[2]*pos */

    }
     pub struct Spiral{
        pub layer:u32,
        pub posision:u32
    }
    impl Spiral{
        fn segment(self)->u32{
            if self.layer <= 0 {
                return 0;
            }
            self.posision/self.layer
        }
        fn sPosision(self)->u32{
            self.posision % self.layer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn array_of_hexagons() {
        let hexes = [Hex{q:0,r:0}, Hex{q:1,r:0}];
    }
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
