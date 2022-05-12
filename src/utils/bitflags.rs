#[macro_export]
macro_rules! count{
    ($x: expr) => (1);
    ($x: expr, $($z: expr),+) => {1+ (crate::count!($($z),*)<<1)}
}

macro_rules! bitflags{
    (
  // use vis type for visibility keyword and ident for struct name
        $vis:vis $struct_name:ident : $flag_type:ty{$(
            // vis for field visibility, ident for field name and ty for field data type
            $flag_name:ident = $flag_val:expr
        ),*}
    ) => {
            #[derive(Clone,Copy)]
            pub struct $struct_name {
                bits:$flag_type
            }
            impl $struct_name  {
                $(
                    // vis for field visibility, ident for field name and ty for field data type
                    pub const $flag_name : Self = Self{bits: $flag_val};
                )*

                pub const ALL:Self = Self{bits:(crate::count!($(Self::$flag_name),*))};
                pub const NONE:Self = Self{bits:0};

                pub fn count_active(&self)->usize{
                    use std::ops::BitAnd;
                    $((if Self::$flag_name.bits.bitand(self.bits) == 0 {0}else{1})+)*0
                }

                pub fn iter(&self)->Iter{
                    Iter(&self,1)
                }
                pub fn iter_all(&self)->IterAll{
                    IterAll(&self,1)
                }
            }

            pub struct IterAll<'a>(&'a$struct_name,$flag_type);
            impl Iterator for IterAll<'_> {
                type Item = ($struct_name,bool);
                fn next(&mut self)->Option<Self::Item>{
                    use std::ops::BitAnd;
                    if( self.1 == 0 ){ None }
                    else{
                        if $struct_name::ALL.bits.bitand(self.1) == 0 {
                            self.1 = 0;
                            None
                        }else {
                            let bits = self.1;
                            self.1 = self.1<<1;
                            Some(($struct_name { bits }, self.0.bits.bitand(bits) != 0))
                        }
                    }
                }
            }

            pub struct Iter<'a>(&'a $struct_name,$flag_type);

            impl Iterator for Iter<'_> {
                type Item = $struct_name;
                fn next(&mut self)->Option<Self::Item>{
                    use std::ops::BitAnd;
                    if( self.1 == 0 ){ None}
                    else{
                        loop {
                            if (self.1 != 0) && self.0.bits.bitand(self.1)==0{
                                self.1 = self.1<<1;
                            }else{
                                break
                            }
                        };

                        if $struct_name::ALL.bits.bitand(self.1) == 0 {
                            self.1 = 0;
                            None
                        }else{
                            let bits = self.1;
                            self.1 = self.1<<1;
                            Some($struct_name { bits })
                        }
                    }
                }
            }

            impl  std::hash::Hash for $struct_name {
                fn hash<H>(&self, h: &mut H) where H: std::hash::Hasher { self.bits.hash(h) }
            }


            impl  std::ops::Shl<usize> for $struct_name {
                type Output=Self;
                fn shl(self, n: usize) -> Self::Output {
                    Self{bits:self.bits.shl(n)}
                }
            }

            impl  std::ops::Shr<usize> for $struct_name {
                type Output=Self;
                fn shr(self, n: usize) -> Self::Output {
                    Self{bits:self.bits.shr(n)}
                }
            }

            impl std::ops::BitAnd for $struct_name {
                type Output=Self;
                fn bitand(self, rhs: Self) -> Self::Output {
                    Self{bits:self.bits.bitand(rhs.bits)}
                }
            }
            impl std::ops::BitOr for $struct_name {
                type Output=Self;
                fn bitor(self, rhs: Self) -> Self::Output {
                    Self{bits:self.bits.bitor(rhs.bits)}
                }
            }
            impl std::ops::Sub for $struct_name {
                type Output=Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    use std::ops::{BitAnd,Not};
                    Self{bits:self.bits.bitand(rhs.bits.not()).bitand(Self::ALL.bits)}
                }
            }
            impl std::ops::Not for $struct_name {
                type Output=Self;
                fn not(self) -> Self::Output {
                    use std::ops::{BitAnd};
                    Self{bits:self.bits.not().bitand(Self::ALL.bits)}
                }
            }
            impl std::cmp::PartialEq for $struct_name {
                fn eq(&self,rhs:&Self) -> bool {
                    self.bits.eq(&rhs.bits)
                }
            }
            impl std::cmp::Eq for $struct_name {}
            impl From<$flag_type> for $struct_name{
                fn from(bits: $flag_type    ) -> Self { Self{bits} }
            }
            impl std::fmt::Debug for $struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    let debug_string =
                        if *self == $struct_name::ALL {"ALL".to_string()}
                        else if *self == $struct_name::NONE {"NONE".to_string()}
                        else{
                            use std::ops::BitAnd;
                            let vec = vec![$((Self::$flag_name,stringify!($flag_name))),*].iter().filter(|(f,_)|{
                                self.bits.bitand(f.bits)==f.bits
                            }).map(|pair|pair.1).collect::<Vec<&str>>();
                            if vec.len()==1 {
                                vec[0].to_string()
                            }else{
                                format!("{{{}}}", vec.join(" | "))

                            }
                        };
                    write!(f, "{}::{}",stringify!($struct_name), debug_string)
                }
            }
        }
}

pub(crate) use bitflags;

#[cfg(test)]
pub(self) mod test {

    bitflags! (pub(self) Flags:u8{A=1,B=2,C=4});
    impl Flags {
        pub fn intersects_with(&self, other: Self) -> bool {
            use std::ops::BitAnd;
            self.bits.bitand(other.bits) != 0
        }
    }
    #[test]
    fn test() {
        let e1 = Flags::A | Flags::C;
        let e2 = Flags::B | Flags::C;
        // print
        assert_eq!((e1 | e2), Flags::A | Flags::B | Flags::C); // union
        assert_eq!((e1 & e2), Flags::C); // intersection
        assert_eq!((e1 - e2).bits, Flags::A.bits); // set difference
        assert_eq!((e1 - e2), Flags::A); // set difference
        assert_eq!(!e2, Flags::A); // set complement
    }
    #[test]
    fn test2() {
        let all = Flags::A | Flags::B | Flags::C;
        // print
        assert!(all.intersects_with(Flags::A)); // union
        assert!(!all.intersects_with(Flags::NONE)); // union
        assert!(all.intersects_with(Flags::A)); // union
        assert!(all.intersects_with(Flags::B)); // union
        assert!(all.intersects_with(Flags::C)); // union
        assert_eq!(Flags::NONE, 0.into()); // union
        assert_eq!(all, Flags::ALL); // union
    }
    #[test]
    fn test_count() {
        assert_eq!(Flags::ALL.count_active(), 3);

        assert_eq!((Flags::A | Flags::B).count_active(), 2);
        assert_eq!((Flags::A | Flags::C).count_active(), 2);
        assert_eq!((Flags::B | Flags::C).count_active(), 2);

        assert_eq!(Flags::A.count_active(), 1);
        assert_eq!(Flags::B.count_active(), 1);
        assert_eq!(Flags::C.count_active(), 1);

        assert_eq!(Flags::NONE.count_active(), 0);
    }

    #[test]
    fn test_iter_all() {
        let mut iter_all = Flags::ALL.iter_all();

        assert_eq!(iter_all.next(), Some((Flags::A, true)));
        assert_eq!(iter_all.next(), Some((Flags::B, true)));
        assert_eq!(iter_all.next(), Some((Flags::C, true)));
        assert_eq!(iter_all.next(), None);

        let mut iter_none = Flags::NONE.iter_all();

        assert_eq!(iter_none.next(), Some((Flags::A, false)));
        assert_eq!(iter_none.next(), Some((Flags::B, false)));
        assert_eq!(iter_none.next(), Some((Flags::C, false)));
        assert_eq!(iter_none.next(), None);
    }
    #[test]
    fn test3() {
        let all = Flags::ALL;
        let mut all_iter = all.iter();

        assert_eq!(all_iter.next(), Some(Flags::A));
        assert_eq!(all_iter.next(), Some(Flags::B));
        assert_eq!(all_iter.next(), Some(Flags::C));
        assert_eq!(all_iter.next(), None);
    }
}
