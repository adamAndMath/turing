pub trait Space {
    type Sym: Clone + Eq;
    type Dir: Clone;

    fn read(&self) -> Self::Sym;
    fn write(&mut self, sym: Self::Sym);
    fn mov(&mut self, dir: &Self::Dir, def: &Self::Sym);
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

            fn mov(&mut self, dir: &Self::Dir, def: &Self::Sym) {
                $(self.$n.mov(&dir.$n, &def.$n);)+
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

            fn mov(&mut self, dir: &Self::Dir, def: &Self::Sym) {
                $(self[$n - $i - 1].mov(&dir[$n - $i - 1], &def[$n - $i - 1]);)+
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
            fn mov(&mut self, _: &Self::Dir, _: &Self::Sym) {}
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

    fn mov(&mut self, dir: &Self::Dir, def: &Self::Sym) {
        self.iter_mut().zip(dir).zip(def).for_each(|((t,d),s)|t.mov(d,s));
    }
}
