// #[macro_export]
// macro_rules! count{
//     ($x: expr) => (1);
//     ($x: expr, $($z: expr),+) => {1+ (crate::count!($($z),*)<<1)}
// }

// macro_rules! bitflags{
//     (
//   // use vis type for visibility keyword and ident for struct name
//         $vis:vis $struct_name:ident : $flag_type:ty{$(
//             // vis for field visibility, ident for field name and ty for field data type
//             $flag_name:ident = $flag_val:expr 
//         ),*}
//     ) => {
//             #[derive(Clone,Copy)]
//             pub struct $struct_name {
//                 value:$flag_type
//             }
//             impl $struct_name  {
//                 $(
//                     // vis for field visibility, ident for field name and ty for field data type
//                     pub const $flag_name : Self = Self{value: $flag_val};
//                 )*

//                 pub const ALL:Self = Self{value:(crate::count!($(Self::$flag_name),*))};
//                 pub const NONE:Self = Self{value:0};
//             }
//             impl crate::utils::bitflags::Flag for $struct_name {
//                 fn new(value:$flag_type)->Self{ Self{value} }
//                 fn value(&self)->$flag_type{self.value}
//             }
//         }
// }

// use std::{ops::{BitAnd, BitOr, Not}, marker::PhantomData};

// pub(crate) use bitflags;

// pub trait Flag {
//     type FlagType:BitAnd+BitOr+Not+std::hash::Hash;
//     const ALL:Self;
//     const NONE:Self;

//     fn new(value:Self::FlagType)->Self;
//     fn value(&self)->Self::FlagType;

//     fn count_active(&self)->usize{
//         Self::ALL.iter().filter(|other|self.intersects_with(other)).count();
//     }

//     fn intersects_with(&self,other:&Self)->bool{
//         self & other != 0
//     }
//     fn iter(&self)->FlagIter<Self>{
//         FlagIter{flag:&self,current:1}
//     }
//     fn iter_all(&self)->FlagIterAll<Self>{
//         FlagIterAll{flag:&self,current:1}
//     }
// }
// pub trait FlagImpl {
//     type FlagType:BitAnd+BitOr+Not+std::hash::Hash;
//     fn value(&self)->Self::FlagType;
//     fn all(& self)->Self::FlagType;
//     fn none(& self)->Self::FlagType;
// }
// impl <F:Flag> FlagImpl for F{
//     fn value(&self)->Self::FlagType{
//         self.value()
//     }
//     fn all(&self)->Self::FlagType{
//         Self::ALL.value()
//     }
//     fn none(&self)->Self::FlagType{
//         Self::NONE.value()
//     }
// }

// pub struct FlagIterAll<'a,F:Flag>{flag:&'a F,current:F}

// impl <'a,F:Flag> Iterator for FlagIterAll<'_,F> {
//     type Item = (F::FlagType,bool);
//     fn next(&mut self)->Option<Self::Item>{
//         if self.current == F::FlagType::NONE {
//             None
//         }
//         else {
//             let old = self.current;
//             self.current = self.current<<1;
//             Some((old, self.flag.intersects_with(&old)))
//         }
//     }
// }

// pub struct FlagIter<'a,F:Flag>{flag:&'a F,current:F}

// impl <'a,F:Flag> Iterator for FlagIter<'_,F> {
//     type Item = F::FlagType;
//     fn next(&mut self)->Option<Self::Item>{
//         loop {
//             if self.current == F::NONE || self.flag.intersects_with(&self.current) {
//                 break;
//             }else{
//                 self.current = self.current<<1;
//             }
//         };

//         if !F::ALL.intersects_with(&self.current) {
//             self.current = F::None;
//             None
//         }else{
//             let old = self.current;
//             self.current = self.current<<1;
//             Some(old)
//         }
//     }
// }

// impl <T> std::hash::Hash for dyn FlagImpl<FlagType=T> {
//     fn hash<H>(&self, h: &mut H) where H: std::hash::Hasher { self.value().hash(h) }
// }


// impl <T> std::ops::Shl<usize> for dyn Flag<FlagType=T> {
//     type Output=Self;
//     fn shl(self, n: usize) -> Self::Output {
//         self.value().shl(n).into()
//     }
// }

// impl <T> std::ops::Shr<usize> for dyn Flag<FlagType=T> {
//     type Output=Self;
//     fn shr(self, n: usize) -> Self::Output {
//         self.value().shr(n).into()
//     }
// }

// impl <T> std::ops::BitAnd for dyn Flag<FlagType=T> {
//     type Output=Self;
//     fn bitand(self, rhs: Self) -> Self::Output {
//         self.value().bitand(rhs.value()).into()
//     }
// }
// impl <T> std::ops::BitOr for dyn Flag<FlagType=T> {
//     type Output=Self;
//     fn bitor(self, rhs: Self) -> Self::Output {
//         self.value().bitor(rhs.value()).into()
//     }
// }
// impl <T> std::ops::Sub for dyn Flag<FlagType=T> {
//     type Output=Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         self.value().bitand(rhs.value().not().bitand(Self::ALL.value())).into()
//     }
// }
// impl <T> std::ops::Not for dyn Flag<FlagType=T> {
//     type Output=Self;
//     fn not(self) -> Self::Output {
//         self.value().not().bitand(Self::ALL.value()).into()
//     }
// }
// impl <T> std::cmp::PartialEq for dyn Flag<FlagType=T> {
//     fn eq(&self,rhs:&Self) -> bool {
//         self.value().eq(&rhs.value()).into
//     }
// }

// impl <T> From<T> for dyn Flag<FlagType=T>{
//     fn from(value: T) -> Self {
//         Self::new(value)
//     }
// }
// impl <T> std::fmt::Debug for dyn Flag<FlagType=T>{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         let debug_string = 
//             if *self == Self::ALL {"ALL".to_string()}
//             else if *self == Self::NONE {"NONE".to_string()}
//             else{
//                 let vec = Self::ALL.iter().filter(|(f,_)|{
//                     self.bits.bitand(f.bits)==f.bits
//                 }).map(|pair|pair.1).collect::<Vec<&str>>();
//                 if vec.len()==1 {
//                     vec[0].to_string()
//                 }else{
//                     format!("{{{}}}", vec.join(" | "))

//                 }
//             };
//         write!(f, "{}::{}",stringify!($struct_name), debug_string)
//     }   
// }

// #[cfg(test)]
// pub(self) mod test{

//     bitflags! (pub(self) Flags:u8{A=1,B=2,C=4});
//     #[test]
//     fn test() {
//         let e1 = Flags::A | Flags::C;
//         let e2 = Flags::B | Flags::C;
//         // print
//         assert_eq!((e1 | e2), Flags::A |Flags::B | Flags::C);   // union
//         assert_eq!((e1 & e2), Flags::C);     // intersection
//         assert_eq!((e1 - e2).bits, Flags::A.bits);     // set difference
//         assert_eq!((e1 - e2), Flags::A);     // set difference
//         assert_eq!(!e2, Flags::A);           // set complement
//     }
//     #[test]
//     fn test2() {
//         let all = Flags::A |Flags::B | Flags::C;
//         // print
//         assert!(all.intersects_with(Flags::A));   // union
//         assert!(!all.intersects_with(Flags::NONE));   // union
//         assert!(all.intersects_with(Flags::A));   // union
//         assert!(all.intersects_with(Flags::B));   // union
//         assert!(all.intersects_with(Flags::C));   // union
//         assert_eq!(Flags::NONE,0.into());   // union
//         assert_eq!(all,Flags::ALL);   // union
//     }
//     #[test]
//     fn test_count() {
//         assert_eq!(Flags::ALL.count_active(),0);
        
//         assert_eq!((Flags::A|Flags::B).count_active(),2);
//         assert_eq!((Flags::A|Flags::C).count_active(),2);
//         assert_eq!((Flags::B|Flags::C).count_active(),2);

//         assert_eq!(Flags::A.count_active(),1);
//         assert_eq!(Flags::B.count_active(),1);
//         assert_eq!(Flags::C.count_active(),1);

//         assert_eq!(Flags::NONE.count_active(),0);
//     }

//     #[test]
//     fn test_iter_all() {
//         let mut iter_all = Flags::ALL.iter_all();
        
//         assert_eq!(iter_all.next(), Some((Flags::A,true)));
//         assert_eq!(iter_all.next(), Some((Flags::B,true)));
//         assert_eq!(iter_all.next(), Some((Flags::C,true)));
//         assert_eq!(iter_all.next(), None);


//         let mut iter_none = Flags::NONE.iter_all();
        
//         assert_eq!(iter_none.next(), Some((Flags::A,false)));
//         assert_eq!(iter_none.next(), Some((Flags::B,false)));
//         assert_eq!(iter_none.next(), Some((Flags::C,false)));
//         assert_eq!(iter_none.next(), None);
//     }
//     #[test]
//     fn test3() {

//         let all = Flags::ALL;
//         let mut all_iter = all.iter();

//         assert_eq!(all_iter.next(),Some(Flags::A));
//         assert_eq!(all_iter.next(),Some(Flags::B));
//         assert_eq!(all_iter.next(),Some(Flags::C));
//         assert_eq!(all_iter.next(),None);
//     }
// }