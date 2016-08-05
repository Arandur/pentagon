#[macro_export]
macro_rules! define_vector2 {
    ( struct $id:ident, $base:ty ) => {
        #[derive(Clone, Debug)]
        struct $id {
            pub x: $base,
            pub y: $base
        }

        impl_vector2!($id, $base);
    };
    ( pub struct $id:ident, $base:ty ) => {
        #[derive(Clone, Debug)]
        pub struct $id {
            pub x: $base,
            pub y: $base
        }

        impl_vector2!($id, $base);
    };
    ( struct $id:ident, $base:ty; $( $trait_:ident ),* ) => {
        define_vector2!(struct $id, $base);

        $(
            impl_vector2_trait!($id, $base, $trait_);
        )*
    };
    ( pub struct $id:ident, $base:ty; $( $trait_:ident ),* ) => {
        define_vector2!(pub struct $id, $base);

        $(
            impl_vector2_trait!($id, $base, $trait_);
        )*
    }
}

macro_rules! impl_vector2 {
    ( $id:ident, $base:ty ) => {
        impl_vector2_op!($id, $base, Add, add);
        impl_vector2_op!($id, $base, Sub, sub);
        impl_scalar2_op!($id, $base, Mul, mul);
        impl_vector2_assign_op!($id, $base, AddAssign, add_assign);
        impl_vector2_assign_op!($id, $base, SubAssign, sub_assign);
        impl_scalar2_assign_op!($id, $base, MulAssign, mul_assign);
    }
}

macro_rules! impl_vector2_op {
    ( $id:ident, $base:ty, $op:ident, $func:ident ) => {
        impl ::std::ops::$op for $id {
            type Output = $id;

            fn $func(self, rhs: $id) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self.x, rhs.x),
                    y: ::std::ops::$op::$func(self.y, rhs.y)
                }
            }
        }

        impl<'a> ::std::ops::$op<$id> for &'a $id {
            type Output = $id;

            fn $func(self, rhs: $id) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self.x, rhs.x),
                    y: ::std::ops::$op::$func(self.y, rhs.y)
                }
            }
        }

        impl<'a> ::std::ops::$op<&'a $id> for $id {
            type Output = $id;

            fn $func(self, rhs: &'a $id) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self.x, rhs.x),
                    y: ::std::ops::$op::$func(self.y, rhs.y)
                }
            }
        }

        impl<'a, 'b> ::std::ops::$op<&'a $id> for &'b $id {
            type Output = $id;

            fn $func(self, rhs: &'a $id) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self.x, rhs.x),
                    y: ::std::ops::$op::$func(self.y, rhs.y)
                }
            }
        }


    }
}

macro_rules! impl_scalar2_op {
    ( $id:ident, $base:ty, $op:ident, $func:ident ) => {
        impl ::std::ops::$op<$base> for $id {
            type Output = $id;

            fn $func(self, rhs: $base) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self.x, rhs),
                    y: ::std::ops::$op::$func(self.y, rhs)
                }
            }
        }

        impl ::std::ops::$op<$id> for $base {
            type Output = $id;

            fn $func(self, rhs: $id) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self, rhs.x),
                    y: ::std::ops::$op::$func(self, rhs.y)
                }
            }
        }

        impl<'a> ::std::ops::$op<$base> for &'a $id {
            type Output = $id;

            fn $func(self, rhs: $base) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self.x, rhs),
                    y: ::std::ops::$op::$func(self.y, rhs)
                }
            }
        }

        impl<'a> ::std::ops::$op<&'a $id> for $base {
            type Output = $id;

            fn $func(self, rhs: &'a $id) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self, rhs.x),
                    y: ::std::ops::$op::$func(self, rhs.y)
                }
            }
        }

        impl<'a> ::std::ops::$op<&'a $base> for $id {
            type Output = $id;

            fn $func(self, rhs: &'a $base) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self.x, rhs),
                    y: ::std::ops::$op::$func(self.y, rhs)
                }
            }
        }

        impl<'a> ::std::ops::$op<$id> for &'a $base {
            type Output = $id;

            fn $func(self, rhs: $id) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self, rhs.x),
                    y: ::std::ops::$op::$func(self, rhs.y)
                }
            }
        }

        impl<'a, 'b> ::std::ops::$op<&'a $base> for &'b $id {
            type Output = $id;

            fn $func(self, rhs: &'a $base) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self.x, rhs),
                    y: ::std::ops::$op::$func(self.y, rhs)
                }
            }
        }

        impl<'a, 'b> ::std::ops::$op<&'a $id> for &'b $base {
            type Output = $id;

            fn $func(self, rhs: &'a $id) -> $id {
                $id {
                    x: ::std::ops::$op::$func(self, rhs.x),
                    y: ::std::ops::$op::$func(self, rhs.y)
                }
            }
        }
    }
}

macro_rules! impl_vector2_assign_op {
    ( $id:ident, $base:ty, $op:ident, $func:ident ) => {
        impl ::std::ops::$op for $id {
            fn $func(&mut self, rhs: $id) {
                ::std::ops::$op::$func(&mut self.x, rhs.x);
                ::std::ops::$op::$func(&mut self.y, rhs.y);
            }
        }

        impl<'a> ::std::ops::$op<&'a $id> for $id {
            fn $func(&mut self, rhs: &'a $id) {
                ::std::ops::$op::$func(&mut self.x, rhs.x);
                ::std::ops::$op::$func(&mut self.y, rhs.y);
            }
        }
    }
}

macro_rules! impl_scalar2_assign_op {
    ( $id:ident, $base:ty, $op:ident, $func:ident ) => {
        impl ::std::ops::$op<$base> for $id {
            fn $func(&mut self, rhs: $base) {
                ::std::ops::$op::$func(&mut self.x, rhs);
                ::std::ops::$op::$func(&mut self.y, rhs);
            }
        }
    }
}

macro_rules! impl_vector2_trait {
    ( $id:ident, $base:ty, Div ) => {
        impl_scalar2_op!($id, $base, Div, div);
    };
    ( $id:ident, $base:ty, DivAssign ) => {
        impl_scalar2_assign_op!($id, $base, DivAssign, div_assign);
    };
    ( $id:ident, $base:ty, Copy ) => {
        impl ::std::marker::Copy for $id {}
    };
    ( $id:ident, $base:ty, PartialEq ) => {
        impl ::std::cmp::PartialEq for $id {
            fn eq(&self, rhs: &$id) -> bool {
                self.x == rhs.x && self.y == rhs.y
            }
        }
    };
    ( $id:ident, $base:ty, Eq ) => {
        impl ::std::cmp::Eq for $id {}
    };
    ( $id:ident, $base:ty, Default ) => {
        impl Default for $id {
            fn default() -> $id {
                $id {
                    x: ::std::default::Default::default(),
                    y: ::std::default::Default::default()
                }
            }
        }
    };
    ( $id:ident, $base:ty, Neg ) => {
        impl ::std::ops::Neg for $id {
            type Output = $id;

            fn neg(self) -> $id {
                $id {
                    x: -self.x,
                    y: -self.y
                }
            }
        }
    };
    ( $id:ident, $base:ty, Hash ) => {
        impl ::std::hash::Hash for $id {
            fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
                self.x.hash(state);
                self.y.hash(state);
            }
        }
    }
}
