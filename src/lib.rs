use std::collections::{ VecDeque, HashMap };
use std::fmt::{ self, Display, Formatter };
use std::hash::Hash;

pub trait Space {
    type Sym: Clone + Eq;
    type Dir: Clone;

    fn read(&self) -> Self::Sym;
    fn write(&mut self, sym: Self::Sym);
    fn mov(&mut self, dir: Self::Dir, def: &Self::Sym);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Left,
    Stay,
    Right,
}

pub struct Tape<Sym> {
    tape: VecDeque<Sym>,
    pos: usize,
}

impl<Sym> Tape<Sym> {
    pub fn new(tape: VecDeque<Sym>) -> Self {
        Tape { tape, pos: 0 }
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

    fn mov(&mut self, dir: Dir, def: &Sym) {
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

macro_rules! impl_tuple_space {
    ($($n:tt : $T:ident),+) => {
        impl<$($T: Space),+> Space for ($($T),+) {
            type Sym = ($($T::Sym),*);
            type Dir = ($($T::Dir),*);

            fn read(&self) -> Self::Sym {
                ($(self.$n.read()),+)
            }

            fn write(&mut self, sym: Self::Sym) {
                $(self.$n.write(sym.$n);)+
            }

            fn mov(&mut self, dir: Self::Dir, def: &Self::Sym) {
                $(self.$n.mov(dir.$n, &def.$n);)+
            }
        }
    };
}

macro_rules! impl_array_space {
    ($n:literal, $($i:literal),+) => {
        impl<T: Space> Space for [T;$n] {
            type Sym = [T::Sym;$n];
            type Dir = [T::Dir;$n];

            fn read(&self) -> Self::Sym {
                [$(self[$n - $i - 1].read()),+]
            }

            fn write(&mut self, sym: Self::Sym) {
                $(self[$n - $i - 1].write(sym[$n - $i - 1].clone());)+
            }

            fn mov(&mut self, dir: Self::Dir, def: &Self::Sym) {
                $(self[$n - $i - 1].mov(dir[$n - $i - 1].clone(), &def[$n - $i - 1]);)+
            }
        }

        impl_array_space!($($i),+);
    };
    ($n:literal) => {
        impl<T: Space> Space for [T;$n] {
            type Sym = [T::Sym;$n];
            type Dir = [T::Dir;$n];

            fn read(&self) -> Self::Sym { [] }
            fn write(&mut self, _: Self::Sym) {}
            fn mov(&mut self, _: Self::Dir, _: &Self::Sym) {}
        }
    };
}

impl_tuple_space!(0:T0,1:T1);
impl_tuple_space!(0:T0,1:T1,2:T2);
impl_tuple_space!(0:T0,1:T1,2:T2,3:T3);
impl_tuple_space!(0:T0,1:T1,2:T2,3:T3,4:T4);
impl_tuple_space!(0:T0,1:T1,2:T2,3:T3,4:T4,5:T5);
impl_tuple_space!(0:T0,1:T1,2:T2,3:T3,4:T4,5:T5,6:T6);
impl_tuple_space!(0:T0,1:T1,2:T2,3:T3,4:T4,5:T5,6:T6,7:T7);
impl_tuple_space!(0:T0,1:T1,2:T2,3:T3,4:T4,5:T5,6:T6,7:T7,8:T8);
impl_tuple_space!(0:T0,1:T1,2:T2,3:T3,4:T4,5:T5,6:T6,7:T7,8:T8,9:T9);
impl_array_space!(32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);

impl<T: Space> Space for Vec<T> {
    type Sym = Vec<T::Sym>;
    type Dir = Vec<T::Dir>;

    fn read(&self) -> Self::Sym {
        self.iter().map(|s|s.read()).collect()
    }

    fn write(&mut self, sym: Self::Sym) {
        self.iter_mut().zip(sym).for_each(|(t,s)|t.write(s));
    }

    fn mov(&mut self, dir: Self::Dir, def: &Self::Sym) {
        self.iter_mut().zip(dir).zip(def).for_each(|((t,d),s)|t.mov(d,s));
    }
}

pub struct Turing<S: Space, Mem> {
    map: HashMap<(S::Sym, Mem), (S::Dir, S::Sym, Mem)>,
    default: S::Sym,
    initial: Mem,
    accepted: Mem,
}

impl<S: Space, Mem> Turing<S, Mem> {
    pub fn new(map: HashMap<(S::Sym, Mem), (S::Dir, S::Sym, Mem)>, default: S::Sym, initial: Mem, accepted: Mem) -> Self {
        Turing { map, default, initial, accepted }
    }

    fn step(&self, mut space: S, mem: Mem) -> Option<(S, Mem)>
        where S::Sym: Hash, Mem: Clone + Eq + Hash {
        let (dir, sym, mem) = self.map.get(&(space.read(), mem))?;
        space.write(sym.clone());
        space.mov(dir.clone(), &self.default);
        Some((space, mem.clone()))
    }

    pub fn run(&self, mut space: S) -> Option<S>
        where S::Sym: Hash, Mem: Clone + Eq + Hash {
        let mut mem = self.initial.clone();
        while mem != self.accepted {
            let state = self.step(space, mem)?;
            space = state.0;
            mem = state.1;
        }
        Some(space)
    }

    pub fn debug<F: Fn(&S, &Mem)>(&self, mut space: S, peek: F) -> Option<S>
        where S::Sym: Hash, Mem: Clone + Eq + Hash {
        let mut mem = self.initial.clone();
        while mem != self.accepted {
            peek(&space, &mem);
            let state = self.step(space, mem)?;
            space = state.0;
            mem = state.1;
        }
        Some(space)
    }
}

#[macro_export]
macro_rules! turing {
    ($default:expr ; $initial:expr ; $accept:expr ; $(($mem:expr) { $($rest:tt)* },)+) => (
        turing!($default ; $initial ; $accept ; $(($mem) { $($rest)* }),+)
    );
    ($default:expr ; $initial:expr ; $accept:expr ; $(($mem:expr) { $($sym:expr => ($dir:expr, $sym_new:expr, $mem_new:expr))* }),+) => ({
        let mut map = std::collections::HashMap::new();
        $($(
            map.insert(($sym, $mem), ($dir, $sym_new, $mem_new));
        )+)+
        crate::turing::Turing::new(map, $default, $initial, $accept)
    });
}