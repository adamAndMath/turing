use std::collections::{ VecDeque, HashMap };
use std::fmt::{ self, Display, Formatter };
use std::hash::Hash;

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

    pub fn current(&self) -> &Sym {
        &self.tape[self.pos]
    }

    fn write(&mut self, sym: Sym) {
        self.tape[self.pos] = sym;
    }

    fn mov(&mut self, dir: Dir, def: &Sym) where Sym: Clone {
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

    pub fn print(&self, view: usize) where Sym: Display {
        let min = if self.pos < view { 0 } else { self.pos - view };
        let max = self.tape.len().max(self.pos + view + 1);
        for sym in self.tape.iter().take(max).skip(min) {
            print!("{}", sym);
        }
        println!();
        println!("{:>1$}", "^", self.pos - min + 1);
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

pub struct Turing<Sym, Mem> {
    map: HashMap<(Sym, Mem), (Dir, Sym, Mem)>,
    default: Sym,
    initial: Mem,
    accepted: Mem,
}

impl<Sym, Mem> Turing<Sym, Mem> {
    pub fn new(map: HashMap<(Sym, Mem), (Dir, Sym, Mem)>, default: Sym, initial: Mem, accepted: Mem) -> Self {
        Turing { map, default, initial, accepted }
    }

    fn step(&self, mut state: Tape<Sym>, mem: Mem) -> Option<(Tape<Sym>, Mem)>
        where Sym: Clone + Eq + Hash + Default, Mem: Clone + Eq + Hash {
        let (dir, sym, mem) = self.map.get(&(state.current().clone(), mem))?;
        state.write(sym.clone());
        state.mov(*dir, &self.default);
        Some((state, mem.clone()))
    }

    pub fn run(&self, mut tape: Tape<Sym>) -> Option<Tape<Sym>>
        where Sym: Clone + Eq + Hash + Default, Mem: Clone + Eq + Hash {
        let mut mem = self.initial.clone();
        while mem != self.accepted {
            let state = self.step(tape, mem)?;
            tape = state.0;
            mem = state.1;
        }
        Some(tape)
    }

    pub fn debug<F: Fn(&Tape<Sym>, &Mem)>(&self, mut tape: Tape<Sym>, peek: F) -> Option<Tape<Sym>>
        where Sym: Clone + Eq + Hash + Default + Display, Mem: Clone + Eq + Hash + Display {
        let mut mem = self.initial.clone();
        while mem != self.accepted {
            peek(&tape, &mem);
            let state = self.step(tape, mem)?;
            tape = state.0;
            mem = state.1;
        }
        Some(tape)
    }
}

#[macro_export]
macro_rules! turing {
    ($default:expr ; $initial:expr ; $accept:expr ; $(($mem:expr) { $($sym:expr => ($dir:ident, $sym_new:expr, $mem_new:expr))+ },)+) => (
        turing!($default ; $initial ; $accept ; $(($mem) { $($sym => ($dir, $sym_new, $mem_new))+ }),+)
    );
    ($default:expr ; $initial:expr ; $accept:expr ; $(($mem:expr) { $($sym:expr => ($dir:ident, $sym_new:expr, $mem_new:expr))+ }),+) => ({
        let mut map = std::collections::HashMap::new();
        $($(
            map.insert(($sym, $mem), (crate::turing::Dir::$dir, $sym_new, $mem_new));
        )+)+
        crate::turing::Turing::new(map, $default, $initial, $accept)
    })
}