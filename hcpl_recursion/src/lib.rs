#[macro_export]
macro_rules! _recursion__let_rec {
    ($f:ident = |$($arg_id:ident: $arg_ty:ty),*| -> $ret:ty $body:block) => {
        trait AlmostFTrait {
            fn call(&self, $($arg_id: $arg_ty),*) -> $ret;
        }
        struct Almost<F>(F);
        let almost_f = Almost(|almost_f: &dyn AlmostFTrait,  $($arg_id: $arg_ty),*| -> $ret {
            let $f = |$($arg_id: $arg_ty),*| {
                almost_f.call($($arg_id),*)
            };
            $body
        });
        impl<F: Fn(&dyn AlmostFTrait $(,$arg_ty)*) -> $ret> AlmostFTrait for Almost<F> {
            fn call(&self, $($arg_id: $arg_ty),*) -> $ret {
                (self.0)(self, $($arg_id),*)
            }
        }
        let $f = |$($arg_id: $arg_ty),*| {
            almost_f.call($($arg_id),*)
        };
    }
}

#[macro_export]
macro_rules! _recursion__let_rec_mut__impl {
    ($f:ident = [ $($cap_id:ident: $cap_ty:ty),* $(,)? ] |$($arg_id:ident: $arg_ty:ty),*| -> $ret:ty $body:block $dol:tt) => {
        hcpl_recursion::let_rec!(without_cap = |$($cap_id: &mut $cap_ty,)* $($arg_id: $arg_ty),*| -> $ret {
            macro_rules! $f {
                ($dol ($dol inner_args:tt),*) => {
                    without_cap($($cap_id,)* $dol ($dol inner_args,)*)
                }
            }
            $body
        });
        macro_rules! $f {
            ($dol ($dol inner_args:tt),*) => {
                without_cap($(&mut $cap_id,)* $dol ($dol inner_args,)*)
            }
        }
    };
}

#[macro_export]
macro_rules! _recursion__let_rec_mut {
    ($($args:tt)*) => {
        hcpl_recursion::_recursion__let_rec_mut__impl!($($args)* $);
    }
}

pub use crate::{
    _recursion__let_rec as let_rec,
    _recursion__let_rec_mut as let_rec_mut,
};
