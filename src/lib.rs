use std::collections::{ VecDeque, HashMap };
use std::fmt::{ self, Display, Formatter };
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Left,
    Stay,
    Right,
}

pub struct State<Sym, Mem> {
    tape: VecDeque<Sym>,
    pos: usize,
    mem: Mem,
}

impl<Sym, Mem> State<Sym, Mem> {
    pub fn new(tape: VecDeque<Sym>, mem: Mem) -> Self {
        State { tape, mem, pos: 0 }
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

    pub fn print(&self, view: usize) where Sym: Display, Mem: Display {
        let min = if self.pos < view { 0 } else { self.pos - view };
        let max = self.tape.len().max(self.pos + view + 1);
        println!("mem: {}", self.mem);
        for sym in self.tape.iter().take(max).skip(min) {
            print!("{}", sym);
        }
        println!();
        println!("{:>1$}", "^", self.pos - min + 1);
    }
}

impl<Sym: Display, Mem: Display> Display for State<Sym, Mem> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "mem: {}", self.mem)?;
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
    accepted: Mem,
}

impl<Sym, Mem> Turing<Sym, Mem> {
    pub fn new(map: HashMap<(Sym, Mem), (Dir, Sym, Mem)>, default: Sym, accepted: Mem) -> Self {
        Turing { map, default, accepted }
    }

    fn step(&self, mut state: State<Sym, Mem>) -> Option<State<Sym, Mem>>
        where Sym: Clone + Eq + Hash + Default, Mem: Clone + Eq + Hash {
        let (dir, sym, mem) = self.map.get(&(state.current().clone(), state.mem.clone()))?;
        state.write(sym.clone());
        state.mem = mem.clone();
        state.mov(*dir, &self.default);
        Some(state)
    }

    pub fn run(&self, mut state: State<Sym, Mem>) -> Option<State<Sym, Mem>>
        where Sym: Clone + Eq + Hash + Default, Mem: Clone + Eq + Hash {
        while state.mem != self.accepted {
            state = self.step(state)?;
        }
        Some(state)
    }

    pub fn debug<F: Fn(&State<Sym, Mem>)>(&self, mut state: State<Sym, Mem>, peek: F) -> Option<State<Sym, Mem>>
        where Sym: Clone + Eq + Hash + Default + Display, Mem: Clone + Eq + Hash + Display {
        while state.mem != self.accepted {
            peek(&state);
            state = self.step(state)?;
        }
        Some(state)
    }
}

#[macro_export]
macro_rules! turing {
    ($default:expr ; $accept:expr ; $(($mem:expr) { $($sym:expr => ($dir:ident, $sym_new:expr, $mem_new:expr))+ },)+) => (
        turing!($default ; $accept ; $(($mem) { $($sym => ($dir, $sym_new, $mem_new))+ }),+)
    );
    ($default:expr ; $accept:expr ; $(($mem:expr) { $($sym:expr => ($dir:ident, $sym_new:expr, $mem_new:expr))+ }),+) => ({
        let mut map = std::collections::HashMap::new();
        $($(
            map.insert(($sym, $mem), (crate::turing::Dir::$dir, $sym_new, $mem_new));
        )+)+
        crate::turing::Turing::new(map, $default, $accept)
    })
}