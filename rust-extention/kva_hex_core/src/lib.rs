use std::cmp::max;

use num::{abs, Signed, ToPrimitive};
const TAU:f64 = 6.2831853071;

///Hex32 is a common type of Hex, used for common values.
pub type Hex32 = Hex<i32>;
///Hex8 is a common type of Hex, used for small values.
pub type Hex8 = Hex<i8>;

pub mod direction{
    use super::*;
    pub const DIRECTIONS: [Hex8; 12] = [
        // flat-side directions (neighbors)
        Hex {q: 0,r:-1},
        Hex {q: 1,r:-1},
        Hex {q: 1,r: 0},
        Hex {q: 0,r: 1},
        Hex {q:-1,r: 1},
        Hex {q:-1,r: 0},
        // pointy directions (neibour of neighbor)
        Hex { q: 1, r: -2 },
        Hex { q: 2, r: -1 },
        Hex { q: 1, r: 1 },
        Hex { q: -1, r: 2 },
        Hex { q: -2, r: 1 },
        Hex { q: -1, r: -1 },
    ];
    ///Special trait used to allow broad implemtation of From<Hex8> on common signed rational numbers.
    trait NotI8 {}
    impl NotI8 for i16 {}
    impl NotI8 for i32 {}
    impl NotI8 for i64 {}
    impl NotI8 for i128 {}
    impl NotI8 for isize {}
    impl NotI8 for f32 {}
    impl NotI8 for f64 {}

    /// Converts Hex8, which is used for directions and other short values of Hex into other signed primitive implementations of Hex.
    /// Conversion From i8 is expected to always work, as i8 is the smallest signed primitive number.
    impl<T> From<Hex8> for Hex<T> 
    where T: NotI8 + From<i8>{
        fn from(hex: Hex8) -> Self {
            Hex {
                q: T::from(hex.q),
                r: T::from(hex.r),
            }
        }
    }

        ///Special trait used to allow broad implemtation of TryFrom<Hex32> on common signed rational numbers.
        trait NotI32 {}
        impl NotI32 for i8 {}
        impl NotI32 for i16 {}
        impl NotI32 for i64 {}
        impl NotI32 for i128 {}
        impl NotI32 for isize {}
        impl NotI32 for f32 {}
        impl NotI32 for f64 {}

        /// Tries to convert Hex32, the default implementation, to any other signed primitive implementation of Hex.
        /// Conversion fail for i16, i8 or f32. Conversions for those should implement error handling for those.
        /// Conversions into larger values should pass, and could be safly unwrapped.
        impl<T> TryFrom<Hex32> for Hex<T>
            where T: NotI32 + TryFrom<i32> {
                type Error = ();
            
                fn try_from(value: Hex32) -> Result<Self, Self::Error> {
                    Ok(Hex {
                        q: T::try_from(value.q).map_err(|_| ())?,
                        r: T::try_from(value.r).map_err(|_| ())?,
                    })
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
        pub fn get(self:Pointy)->Hex8 {
            get_dir_all(self as i32)
        }
    }
    impl Flat {
        ///Returns Hexagon coordinate for this direction
        pub fn get(self:Flat)->Hex8 {
            get_dir_all(self as i32)
        }
    }
    pub fn get_dir_all(d:i32)->Hex8 {
        let mut d = d;
        while d >= 12{d -= 12}
        while d <  0 {d += 12}
        let d:usize = usize::try_from(d).expect("d should be wrapped to range (0 .. 12)");
        return DIRECTIONS[d];
    }
    pub fn get_dir(d:i32)->Hex8 {
        let mut d = d;
        while d >= 6 {d -= 6}
        while d <  0 {d += 6}
        let d:usize = usize::try_from(d).expect("d should be wrapped to range (0 .. 6)");
        return DIRECTIONS[d];
    }
    pub fn radian_get_dir(r:f64)->Hex8 {
        let d:i32 = (6.0 * r / 6.28).round() as i32;
        return get_dir(d);
    }
    pub fn radian_get_dir_all(r:f64)->Hex8 {
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
T: Signed + Copy + ToPrimitive + Ord{
    pub fn as_array(&self) -> [T;3]{return [self.q, self.r, self.s()]}
    /// the s coordinate.
    pub fn s(&self) -> T {-self.q - self.r}
    pub fn to_xy(&self, flat:bool) -> (f32,f32) {
        let q = self.q.to_f32().expect("Failed converting q to f32");
        let r = self.r.to_f32().expect("Failed converting r to f32");
        if flat{
            (
                // X = (     3./2 * hex.q                    )
                3.0/2.0 * q,
                3f32.sqrt()/2.0 * q + 3f32.sqrt() * r
                // Y = (sqrt(3)/2 * hex.q  +  sqrt(3) * hex.r)
            )
        }
        else {
            (
                // X = (sqrt(3) * hex.q  +  sqrt(3)/2 * hex.r)
                3f32.sqrt() * q + 3f32.sqrt()/2f32 * r ,
                // Y = (                         3./2 * hex.r)
                3.0/2.0 * r
            )
        }
    }
    pub fn get_layer(self) -> T {
        max(abs(self.s()), max(abs(self.q), abs(self.r)))
    }
}
// impl <T> Hex<T> where 
// T: Signed + Copy + TryInto<f64> {
    
// }
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
    use std::{fmt::Display/* , io::Error */};

    use num::{integer::Roots, ToPrimitive};
    use crate::direction::get_dir;

    use super::*;
    impl<T> TryFrom<Spiral> for Hex<T>
    where T:Signed + Ord + TryFrom<i32> + Copy + From<Hex32>{
        type Error = T::Error;
    
        fn try_from(value: Spiral) -> Result<Self, Self::Error> {
            let hexi32 = spiral_to_hex(value);
            Ok(Hex {
                q: T::try_from(hexi32.q)?,
                r: T::try_from(hexi32.r)?,
            })
        }
    }
    impl<T> TryFrom<Hex<T>> for Spiral
    where T:Signed + Ord + ToPrimitive + Copy + Display{
        type Error = ();
    
        fn try_from(value: Hex<T>) -> Result<Self, Self::Error> {
            let tryconvert = hex_to_spiral(value);
            match tryconvert{
                Err(_) => Err(()),
                Ok(val) => Ok(val),
            }
        }
    }
    impl From<usize> for Spiral {
        fn from(value: usize) -> Self {
            let value = value.to_i32()
                .expect("Index:usize expected be less than limit of i32");
            let mut spiral = Spiral{layer:0, posision:0};
            if value <= 0 {return spiral;} //index 0 is the origin/center tile.

            //Layer around the origin tile.
            spiral.layer = (( ((12 * value + 9) as f32).sqrt() - 3f32) /6f32).ceil().to_i32()
                .expect("spiral.layer as f32 is now expected to be above 0 and rounded up to nearest integer.");
            
            //The tile's index, minus number of tiles before this layer.
            spiral.posision = value - (3 * spiral.layer * (spiral.layer - 1) + 1);
            return spiral;
        }
    }

    pub fn spiral_index_to_hex(index:usize)->Hex32 {
        if index <= 0 {
            return Hex{q:0, r:0};
        }
        spiral_to_hex(Spiral::from(index))
    }
    pub fn spiral_to_hex(spiral:Spiral)->Hex32 {
        //hex direction from center tile to segment start
        let d1:Hex32 = get_dir(spiral.segment()).into();
        //Hex direction following segment positive direction
        let d2:Hex32 = get_dir(spiral.segment()+2).into();
        d1 * spiral.layer + d2 * (spiral.posision % spiral.layer)
    }
    pub fn hex_to_spiral<T>(hex:Hex<T>)->Result<Spiral, ()>
    where T:Signed + Ord + ToPrimitive + Copy + Display
    {
        use std::cmp::max;
        let mut spiral = Spiral{layer:0, posision:0};
        spiral.layer = (max(hex.q.abs(), max(hex.r.abs(), hex.s().abs())))
            .to_i32().ok_or(())?;
        if spiral.layer <= 0{
            return Ok(spiral);
        }
        let q:i32 = hex.q.to_i32().ok_or(())?;
        let r:i32 = hex.r.to_i32().ok_or(())?;
        let s:i32 = hex.s().to_i32().ok_or(())?;
        let l = spiral.layer;
        
        // Determine the segment index (0-5) and the position within that segment (segpos).
        // This is based on which coordinate equals either `layer` (l) or `-layer` (-l), which always happens for one coordinate.
        // In "corner" cases where two coordinates match `layer` and `-layer` respectively, either match could be used.
        //      In a corner, the third coordinate always equal 0.
        //      The first matching case is selected, but both produce the same position value.
        let (segment, segpos) =
        {  
            if        r == -l {
                (0, q)
            } else if q ==  l {
                (1, -s)
            } else if s == -l {
                (2, r)
            } else if r ==  l {
                (3, -q)
            } else if q == -l {
                (4, s)
            } else if s ==  l {
                (5, -r)
            } else {
                unreachable!("One of the q, r, s coordinates must be equal to `layer` or `-layer`")
            }
        };
        // The final spiral position is calculated as:
        // segment length (l) * segment index + segment position within that segment.
        // the segment length is equal to the layer. Segments grow by one tile per layer.
        spiral.posision = l * segment + segpos;

        return Ok(spiral);
    }
    pub fn hex_to_spiral_index<T>(hex:Hex<T>)->usize
    where T:Signed + Ord + ToPrimitive + Copy + Display{
        let tryspiral = hex_to_spiral(hex);
        let spiral;
        match tryspiral {
            Ok(val) => spiral = val,
            Err(_) => panic!("Hex may have too extreme values! (out of range for usize)"),
        }
        if spiral.layer <= 0{
            return 0;
        }
        //deconstruct layer to number of tiles before layer. Add posision. There's the index.
        let index:usize = (3 * spiral.layer * (spiral.layer - 1) + 1 + spiral.posision).try_into()
            .expect("Value always positive and less than limit of usize");
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
        pub fn s_posision(&self)->i32{
            self.posision % self.layer
        }
        pub fn layer_len(&self)->i32{
            self.layer*6
        }
        /// changes the Spiral posision by steps, and wraps the result.
        pub fn rotate_tiles(&mut self, steps:i32){
            self.posision += steps;
            while self.posision < 0 {
                self.posision += self.layer_len();
            } while self.posision >= self.layer_len() {
                self.posision -= self.layer_len();
            }
        }
        /// Changes the posision by one edge og the hexagon
        pub fn rotate_hex(&mut self, seg_steps:i32){
            self.rotate_tiles(seg_steps*self.layer);
        }
        pub fn radians_to_steps(self, r:f64)->i32 {
            let steps:f64 = self.layer as f64*6.0 * r/TAU;
            return steps as i32;
        }
        pub fn rotate_radians(&mut self, r:f64){
            self.rotate_tiles(self.radians_to_steps(r));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spiral_indicies_to_hex() {
        assert_eq!(spiral::spiral_index_to_hex(0), Hex { q: 0, r: 0 });
        assert_eq!(spiral::spiral_index_to_hex(1), Hex { q: 0, r: -1 });
        assert_eq!(spiral::spiral_index_to_hex(8), Hex { q: 1, r: -2 });
        assert_eq!(spiral::spiral_index_to_hex(11), Hex { q: 2, r: 0 });
        assert_eq!(spiral::spiral_index_to_hex(30), Hex { q: -2, r: 3 });
        assert_eq!(spiral::spiral_index_to_hex(48), Hex { q: 1, r: 3 });
        assert_eq!(spiral::spiral_index_to_hex(49), Hex { q: 0, r: 4 });
        assert_eq!(spiral::spiral_index_to_hex(84), Hex { q: -5, r: 2 });
    }
    #[test]
    fn hex_to_spiral_indicies() {
        assert_eq!(0, spiral::hex_to_spiral_index( Hex { q: 0, r: 0 }));
        assert_eq!(1, spiral::hex_to_spiral_index( Hex { q: 0, r: -1 }));
        assert_eq!(8, spiral::hex_to_spiral_index( Hex { q: 1, r: -2 }));
        assert_eq!(11, spiral::hex_to_spiral_index( Hex { q: 2, r: 0 }));
        assert_eq!(30, spiral::hex_to_spiral_index( Hex { q: -2, r: 3 }));
        assert_eq!(48, spiral::hex_to_spiral_index( Hex { q: 1, r: 3 }));
        assert_eq!(49, spiral::hex_to_spiral_index( Hex { q: 0, r: 4 }));
        assert_eq!(84, spiral::hex_to_spiral_index( Hex { q: -5, r: 2 }));
    }
}
