use std::str::FromStr;

use crate::utils::bitflags::bitflags;

bitflags!(Segments: u8 {
    A = 1,
    B = 2,
    C = 4,
    D = 8,
    E = 16,
    F = 32,
    G = 64
});

impl FromStr for Segments {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| match c {
                'a' => Segments::A,
                'b' => Segments::B,
                'c' => Segments::C,
                'd' => Segments::D,
                'e' => Segments::E,
                'f' => Segments::F,
                'g' => Segments::G,
                _ => Segments::NONE,
            })
            .reduce(|s1, s2| s1 | s2)
            .map_or(Err(()), Ok)
    }
}

#[derive(Debug)]

pub struct Entry {
    pub signal_patterns: [Segments; 10],
    pub outputs: [Segments; 4],
    pub could_be: [Segments; 7],
}

impl Entry {
    pub fn segments_for(n: usize) -> Segments {
        match n {
            0 => Segments::ALL - Segments::D,
            1 => Segments::C | Segments::F,
            2 => Segments::ALL - (Segments::B | Segments::F),
            3 => Segments::ALL - (Segments::B | Segments::E),
            4 => Segments::ALL - (Segments::A | Segments::E | Segments::G),
            5 => Segments::ALL - (Segments::C | Segments::E),
            6 => Segments::ALL - Segments::C,
            7 => Segments::A | Segments::C | Segments::F,
            8 => Segments::ALL,
            9 => Segments::ALL - Segments::E,
            _ => Segments::NONE,
        }
    }

    pub fn restrict(&mut self, index: usize, segments: &Segments) {
        if self.could_be[index].count_active() != 1 {
            self.could_be[index] = self.could_be[index] & *segments;
            if self.could_be[index].count_active() == 1 {
                let locked_in = !self.could_be[index];
                (0..7)
                    .filter(|i| *i != index)
                    .for_each(|i| self.restrict(i, &locked_in));
            }
        }
    }
    pub fn restrict_to(&mut self, expected: &Segments, found: &Segments) {
        expected
            .iter_all()
            .enumerate()
            .filter(|p| p.1 .1)
            .map(|p| p.0)
            .for_each(|i| self.restrict(i, found));
    }
    // fn adapt(&mut self){

    // }
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entry = Entry {
            signal_patterns: Default::default(),
            outputs: Default::default(),
            could_be: [Segments::ALL; 7],
        };
        let mut entry_str = s.split(" | ");
        if let Some(s)=entry_str.next() {
            let pattern_iter = s.split_whitespace();
            entry
                .signal_patterns
                .iter_mut()
                .zip(pattern_iter)
                .for_each(|(dest, src)| *dest = src.parse::<Segments>().unwrap());
        };
        if let Some(s)=entry_str.next() {
            let outputs_iter = s.split_whitespace();
            entry
                .outputs
                .iter_mut()
                .zip(outputs_iter)
                .for_each(|(dest, src)| *dest = src.parse::<Segments>().unwrap());
        };
        Ok(entry)
    }
}
