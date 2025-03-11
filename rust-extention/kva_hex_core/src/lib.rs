use num::Signed;

pub mod direction{
    use super::*;
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
    ///Special trait used to allow broad implemtation of From<Hex<i8>> on common signed rational numbers.
    trait NotI8 {}
    impl NotI8 for i16 {}
    impl NotI8 for i32 {}
    impl NotI8 for i64 {}
    impl NotI8 for i128 {}
    impl NotI8 for isize {}
    impl NotI8 for f32 {}
    impl NotI8 for f64 {}

    impl<T> From<Hex<i8>> for Hex<T> 
    where T: NotI8 + From<i8>{
        fn from(hex: Hex<i8>) -> Self {
            Hex {
                q: T::from(hex.q),
                r: T::from(hex.r),
            }
        }
    }
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
        pub fn get(self:Pointy)->Hex<i8> {
            get_dir_all(self as i32)
        }
    }
    impl Flat {
        ///Returns Hexagon coordinate for this direction
        pub fn get(self:Flat)->Hex<i8> {
            get_dir_all(self as i32)
        }
    }
    pub fn get_dir_all(d:i32)->Hex<i8> {
        let mut d = d;
        while d > 12{d -= 12}
        while d < 0 {d += 12}
        let d:usize = usize::try_from(d).expect("d should be wrapped to range (0 .. 12)");
        return DIRECTIONS[d];
    }
    pub fn get_dir(d:i32)->Hex<i8> {
        let mut d = d;
        while d > 6 {d -= 6}
        while d < 0 {d += 6}
        let d:usize = usize::try_from(d).expect("d should be wrapped to range (0 .. 6)");
        return DIRECTIONS[d];
    }
    pub fn radian_get_dir(r:f64)->Hex<i8> {
        let d:i32 = (6.0 * r / 6.28).round() as i32;
        return get_dir(d);
    }
    pub fn radian_get_dir_all(r:f64)->Hex<i8> {
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
    pub fn s(self) -> T {-self.q - self.r}

    // pub fn try_convert<U>(self) -> Result<Hex<T>, U::Error>
    //     where T: TryFrom<U>+Copy
    // {
    //     Ok(Hex {
    //         q: self.q.try_into()?,
    //         r: self.r.try_into()?,
    //     })
    // }
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
/// multiplication for hexagon coordiantes, supporting common numerals.
impl<T, U> std::ops::Mul<U> for Hex<T>
    where
        T: Copy + std::ops::Mul<Output = T>,
        U: Copy + Into<T>,
    {
        type Output = Hex<T>;
        fn mul(self, rhs: U) -> Self::Output {
            let scalear:T = rhs.into();
            Hex{
                q : self.q * scalear,
                r : self.r * scalear
        }
    }
}

pub mod spiral
{
    use std::fmt::Error;

    use num::{integer::Roots, ToPrimitive};

    use crate::direction::get_dir;

    use super::*;
    pub fn spiral_index_to_hex(index:usize)->Hex<i32>{
        let index = index.to_i32()
            .expect("Index:usize expected be less than limit of i32");
        if index <= 0 {
            return Hex{q:0, r:0};
        }


        let mut spiral = Spiral{layer:0, posision:0};
        //Layer around the origin tile.
        spiral.layer = (((12 * index + 9).sqrt() - 3) as f32 /6.0).ceil().to_i32()
            .expect("spiral.layer as f32 is now expected to be above 0 and rounded up to nearest integer.");
        //The tile's index, minus tumber of tiles before this layer.
        spiral.posision = index - (3 * spiral.layer * (spiral.layer - 1) + 1);

        let d1:Hex<i32> = get_dir(spiral.segment()).into();
        let d2:Hex<i32> = get_dir(spiral.segment()).into();
        d1 * spiral.layer + d2 * (spiral.posision % spiral.layer)

/*	var count: int = 3 * layer * (layer - 1) + 1  # Count of tiles in all previous layers

	#clockwise posision arond the layer circle, defined with segment then clockwise position from segment orign
	var segment: int = floor((ti-count) / layer) #segment of the layer (0 to 5)
	var pos: int = (ti-count) % layer #segment posision

	return segposlayer_to_QRS([segment, pos, layer])
	#var q = get_dir(segment)[0] * layer + get_dir(segment+2)[0]*pos
	#var r = get_dir(segment)[1] * layer + get_dir(segment+2)[1]*pos
	#var s = get_dir(segment)[2] * layer + get_dir(segment+2)[2]*pos */

    }
    pub fn hex_to_spiral<T>(hex:Hex<T>)->Result<Spiral, T::Error>
    where T:Signed + Ord + TryInto<i32> + Copy{
        use std::cmp::max;
        let mut spiral = Spiral{layer:0, posision:0};
        spiral.layer = (max(hex.q.abs(), max(hex.r.abs(), hex.s().abs()))).try_into()?;
        if spiral.layer <= 0{
            return spiral;
        }
        let q:i32 = hex.q.try_into()?;
        let r:i32 = hex.r.try_into()?;
        let s:i32 = hex.s().try_into()?;
        let l = spiral.layer;

        if        r == -l {
            spiral.posision =  q + l * 0;
        } else if q ==  l {
            spiral.posision = -s + l * 1;
        } else if s == -l {
            spiral.posision =  r + l * 2;
        } else if r ==  l {
            spiral.posision = -r + l * 3;
        } else if q == -l {
            spiral.posision =  s + l * 4;
        } else if s ==  l {
            spiral.posision = -r + l * 5;
        }

        return Ok(spiral);
    }
    pub fn hex_to_spiral_index<T>(hex:Hex<T>)->Result<usize,T::Error>
    where T:Signed + Ord + TryInto<i32> + Copy{
        let spiral = hex_to_spiral(hex)?;
        if spiral.layer <= 0{
            return Ok(0);
        }
        //deconstruct layer to number of tiles before layer. Add posision. There's the index.
        let index: Result<usize, Error> = (3 * spiral.layer * (spiral.layer - 1) + 1 + spiral.posision).try_into();
        return index;
    }
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Spiral{
        pub layer:i32,
        pub posision:i32
    }
    impl Spiral{
        pub fn segment(self)->i32{
            if self.layer <= 0 {
                return 0;
            }
            self.posision/self.layer
        }
        pub fn s_posision(self)->i32{
            self.posision % self.layer
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // #[test]
    // fn array_of_hexagons() {
    //     let hexes = [Hex{q:0,r:0}, Hex{q:1,r:0}];
    // }

/*	print("Testing ti to QRS")
	test_ti_to_qrs(0, 0, 0, 0)
	test_ti_to_qrs(1, 0, -1, 1)
	test_ti_to_qrs(8, 1, -2, 1)
	test_ti_to_qrs(11, 2, 0, -2)
	test_ti_to_qrs(30, -2, 3, -1)
	test_ti_to_qrs(48, 1, 3, -4)
	test_ti_to_qrs(49, 0, 4, -4)
	test_ti_to_qrs(84, -5, 2, 3)
	print("testing QRS to ti")
	test_qrs_to_ti(0, 0, 0, 0)
	test_qrs_to_ti(1, 0, -1, 1)
	test_qrs_to_ti(8, 1, -2, 1)
	test_qrs_to_ti(11, 2, 0, -2)
	test_qrs_to_ti(30, -2, 3, -1)
	test_qrs_to_ti(48, 1, 3, -4)
	test_qrs_to_ti(49, 0, 4, -4)
	test_qrs_to_ti(84, -5, 2, 3) */

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
