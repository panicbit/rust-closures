macro_rules! impl_state_accessors {
    () => {
        pub fn state(&self) -> &S {
            &self.state
        }

        pub fn state_mut(&mut self) -> &mut S {
            &mut self.state
        }

        pub fn into_state(self) -> S {
            self.state
        }
    }
}

macro_rules! impl_closure_once {
    ($name:ident, $($args:tt)*) => {
        pub struct $name<S, R, $($args)*> {
            state: S,
            f: fn(S, $($args)*) -> R,
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> $name<S, R, $($args)*> {
            pub fn new(state: S, f: fn(S, $($args)*) -> R) -> Self {
                $name {
                    state,
                    f,
                }
            }

            impl_state_accessors!();
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnOnce<($($args)*)> for $name<S, R, $($args)*> {
            type Output = R;

            extern "rust-call" fn call_once(self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(self.state, $($args)*)
            }
        }
    }
}

macro_rules! impl_closure_mut {
    ($name:ident, $($args:tt)*) => {
        pub struct $name<S, R, $($args)*> {
            state: S,
            f: fn(&mut S, $($args)*) -> R,
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> $name<S, R, $($args)*> {
            pub fn new(state: S, f: fn(&mut S, $($args)*) -> R) -> Self {
                $name {
                    state,
                    f,
                }
            }

            impl_state_accessors!();
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnOnce<($($args)*)> for $name<S, R, $($args)*> {
            type Output = R;

            extern "rust-call" fn call_once(mut self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(&mut self.state, $($args)*)
            }
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnMut<($($args)*)> for $name<S, R, $($args)*> {
            extern "rust-call" fn call_mut(&mut self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(&mut self.state, $($args)*)
            }
        }
    }
}

macro_rules! impl_closure {
    ($name:ident, $($args:tt)*) => {
        pub struct $name<S, R, $($args)*> {
            state: S,
            f: fn(&S, $($args)*) -> R,
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> $name<S, R, $($args)*> {
            pub fn new(state: S, f: fn(&S, $($args)*) -> R) -> Self {
                $name {
                    state,
                    f,
                }
            }

            impl_state_accessors!();
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnOnce<($($args)*)> for $name<S, R, $($args)*> {
            type Output = R;

            extern "rust-call" fn call_once(self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(&self.state, $($args)*)
            }
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnMut<($($args)*)> for $name<S, R, $($args)*> {
            extern "rust-call" fn call_mut(&mut self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(&self.state, $($args)*)
            }
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::Fn<($($args)*)> for $name<S, R, $($args)*> {
            extern "rust-call" fn call(&self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(&self.state, $($args)*)
            }
        }
    }
}

macro_rules! impl_rec_closure_once {
    ($name:ident, $($args:tt)*) => {
        pub struct $name<S, R, $($args)*> {
            state: S,
            f: fn($name<S, R, $($args)*>, $($args)*) -> R,
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> $name<S, R, $($args)*> {
            pub fn new(state: S, f: fn($name<S, R, $($args)*>, $($args)*) -> R) -> Self {
                $name {
                    state,
                    f,
                }
            }

            impl_state_accessors!();
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnOnce<($($args)*)> for $name<S, R, $($args)*> {
            type Output = R;

            extern "rust-call" fn call_once(self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(self, $($args)*)
            }
        }
    }
}

macro_rules! impl_rec_closure_mut {
    ($name:ident, $($args:tt)*) => {
        pub struct $name<S, R, $($args)*> {
            state: S,
            f: fn(&mut $name<S, R, $($args)*>, $($args)*) -> R,
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> $name<S, R, $($args)*> {
            pub fn new(state: S, f: fn(&mut $name<S, R, $($args)*>, $($args)*) -> R) -> Self {
                $name {
                    state,
                    f,
                }
            }

            impl_state_accessors!();
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnOnce<($($args)*)> for $name<S, R, $($args)*> {
            type Output = R;

            extern "rust-call" fn call_once(mut self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(&mut self, $($args)*)
            }
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnMut<($($args)*)> for $name<S, R, $($args)*> {
            extern "rust-call" fn call_mut(&mut self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(self, $($args)*)
            }
        }
    }
}

macro_rules! impl_rec_closure {
    ($name:ident, $($args:tt)*) => {
        pub struct $name<S, R, $($args)*> {
            state: S,
            f: fn(&$name<S, R, $($args)*>, $($args)*) -> R,
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> $name<S, R, $($args)*> {
            pub fn new(state: S, f: fn(&$name<S, R, $($args)*>, $($args)*) -> R) -> Self {
                $name {
                    state,
                    f,
                }
            }

            impl_state_accessors!();
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnOnce<($($args)*)> for $name<S, R, $($args)*> {
            type Output = R;

            extern "rust-call" fn call_once(self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(&self, $($args)*)
            }
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::FnMut<($($args)*)> for $name<S, R, $($args)*> {
            extern "rust-call" fn call_mut(&mut self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(self, $($args)*)
            }
        }

        #[allow(non_snake_case)]
        impl<S, R, $($args)*> ::core::ops::Fn<($($args)*)> for $name<S, R, $($args)*> {
            extern "rust-call" fn call(&self, ($($args)*): ($($args)*)) -> Self::Output {
                (self.f)(self, $($args)*)
            }
        }
    }
}

macro_rules! impl_traits {
    ($name:ident, $($args:tt)*) => {
        // PartialEq
        impl<S, R, $($args)*> ::core::cmp::PartialEq for $name<S, R, $($args)*>
            where S: PartialEq
        {
            fn eq(&self, other: &Self) -> bool {
                self.state == other.state && self.f as usize == other.f as usize
            }
        }

        // Eq
        impl<S, R, $($args)*> ::core::cmp::Eq for $name<S, R, $($args)*>
            where S: Eq
        {
        }

        // Clone
        impl<S, R, $($args)*> ::core::clone::Clone for $name<S, R, $($args)*>
            where S: Clone
        {
            fn clone(&self) -> Self {
                $name {
                    state: self.state.clone(),
                    f: self.f,
                }
            }
        }
    }
}

macro_rules! impl_closures {
    ($once_name:ident, $mut_name:ident, $name:ident, $rec_once_name:ident, $rec_mut_name:ident, $rec_name:ident, $($args:tt)*) => {
        impl_closure_once!($once_name, $($args)*);
        impl_closure_mut!($mut_name, $($args)*);
        impl_closure!($name, $($args)*);

        impl_rec_closure_once!($rec_once_name, $($args)*);
        impl_rec_closure_mut!($rec_mut_name, $($args)*);
        impl_rec_closure!($rec_name, $($args)*);

        impl_traits!($once_name, $($args)*);
        impl_traits!($mut_name, $($args)*);
        impl_traits!($name, $($args)*);

        impl_traits!($rec_once_name, $($args)*);
        impl_traits!($rec_mut_name, $($args)*);
        impl_traits!($rec_name, $($args)*);
    }
}