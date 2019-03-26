use std::collections::VecDeque;
use std::fmt::{ self, Display, Formatter };
use std::iter::FromIterator;

use crate::{ Dir, Space };

pub struct Tape<Sym> {
    tape: VecDeque<Sym>,
    pos: usize,
}

impl<Sym> FromIterator<Sym> for Tape<Sym> {
    fn from_iter<I: IntoIterator<Item = Sym>>(iter: I) -> Self {
        Tape {
            tape: iter.into_iter().collect(),
            pos: 0
        }
    }
}

impl<Sym: Clone + Eq> Space for Tape<Sym> {
    type Sym = Sym;
    type Dir = Dir;

    fn read(&self) -> Sym {
        self.tape[self.pos].clone()
    }

    fn write(&mut self, sym: Sym) {
        self.tape[self.pos] = sym;
    }

    fn mov(&mut self, dir: &Dir, def: &Sym) {
        match dir {
            Dir::Left if self.pos == 0 => self.tape.push_front(def.clone()),
            Dir::Left => self.pos -= 1,
            Dir::Stay => (),
            Dir::Right => {
                self.pos += 1;
                if self.pos == self.tape.len() {
                    self.tape.push_back(def.clone())
                }
            }
        }
    }
}

impl<Sym: Display> Display for Tape<Sym> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for sym in &self.tape {
            write!(f, "{}", sym)?;
        }

        writeln!(f, "")?;
        write!(f, "{:>1$}", "^", self.pos+1)
    }
}
