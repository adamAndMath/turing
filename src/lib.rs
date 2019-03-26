use std::collections::HashMap;
use std::hash::Hash;

pub mod space;
pub mod tape;

pub use space::Space;
pub use tape::Tape;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Left,
    Stay,
    Right,
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
        $crate::Turing::new(map, $default, $initial, $accept)
    });
}