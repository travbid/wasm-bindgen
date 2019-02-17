use core::mem;

use crate::convert::slices::WasmSlice;
use crate::convert::RefFromWasmAbi;
use crate::convert::{FromWasmAbi, GlobalStack, IntoWasmAbi, ReturnWasmAbi, Stack};
use crate::describe::{inform, WasmDescribe, FUNCTION};
use crate::throw_str;

macro_rules! stack_closures {
    ($( ($cnt:tt $invoke:ident $invoke_mut:ident $($var:ident)*) )*) => ($(
        impl<'a, 'b, $($var,)* R> IntoWasmAbi for &'a (Fn($($var),*) -> R + 'b)
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            type Abi = WasmSlice;

            fn into_abi(self, _extra: &mut Stack) -> WasmSlice {
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    WasmSlice { ptr: a as u32, len: b as u32 }
                }
            }
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn $invoke<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
            a: usize,
            b: usize,
            $($var: <$var as FromWasmAbi>::Abi),*
        ) -> <R as ReturnWasmAbi>::Abi {
            if a == 0 {
                throw_str("closure invoked recursively or destroyed already");
            }
            // Scope all local variables before we call `return_abi` to
            // ensure they're all destroyed as `return_abi` may throw
            let ret = {
                let f: &Fn($($var),*) -> R = mem::transmute((a, b));
                let mut _stack = GlobalStack::new();
                $(
                    let $var = <$var as FromWasmAbi>::from_abi($var, &mut _stack);
                )*
                f($($var),*)
            };
            ret.return_abi(&mut GlobalStack::new())
        }

        impl<'a, $($var,)* R> WasmDescribe for Fn($($var),*) -> R + 'a
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            fn describe() {
                inform(FUNCTION);
                inform($invoke::<$($var,)* R> as u32);
                inform($cnt);
                $(<$var as WasmDescribe>::describe();)*
                <R as WasmDescribe>::describe();
            }
        }

        impl<'a, 'b, $($var,)* R> IntoWasmAbi for &'a mut (FnMut($($var),*) -> R + 'b)
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            type Abi = WasmSlice;

            fn into_abi(self, _extra: &mut Stack) -> WasmSlice {
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    WasmSlice { ptr: a as u32, len: b as u32 }
                }
            }
        }

        #[allow(non_snake_case)]
        unsafe extern "C" fn $invoke_mut<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
            a: usize,
            b: usize,
            $($var: <$var as FromWasmAbi>::Abi),*
        ) -> <R as ReturnWasmAbi>::Abi {
            if a == 0 {
                throw_str("closure invoked recursively or destroyed already");
            }
            // Scope all local variables before we call `return_abi` to
            // ensure they're all destroyed as `return_abi` may throw
            let ret = {
                let f: &mut FnMut($($var),*) -> R = mem::transmute((a, b));
                let mut _stack = GlobalStack::new();
                $(
                    let $var = <$var as FromWasmAbi>::from_abi($var, &mut _stack);
                )*
                f($($var),*)
            };
            ret.return_abi(&mut GlobalStack::new())
        }

        impl<'a, $($var,)* R> WasmDescribe for FnMut($($var),*) -> R + 'a
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            fn describe() {
                inform(FUNCTION);
                inform($invoke_mut::<$($var,)* R> as u32);
                inform($cnt);
                $(<$var as WasmDescribe>::describe();)*
                <R as WasmDescribe>::describe();
            }
        }
    )*)
}

stack_closures! {
    (0 invoke0 invoke0_mut)
    (1 invoke1 invoke1_mut A)
    (2 invoke2 invoke2_mut A B)
    (3 invoke3 invoke3_mut A B C)
    (4 invoke4 invoke4_mut A B C D)
    (5 invoke5 invoke5_mut A B C D E)
    (6 invoke6 invoke6_mut A B C D E F)
    (7 invoke7 invoke7_mut A B C D E F G)
    (8 invoke8 invoke8_mut A B C D E F G H)
}

impl<'a, 'b, A, R> IntoWasmAbi for &'a (Fn(&A) -> R + 'b)
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi,
{
    type Abi = WasmSlice;

    fn into_abi(self, _extra: &mut Stack) -> WasmSlice {
        unsafe {
            let (a, b): (usize, usize) = mem::transmute(self);
            WasmSlice {
                ptr: a as u32,
                len: b as u32,
            }
        }
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn invoke1_ref<A: RefFromWasmAbi, R: ReturnWasmAbi>(
    a: usize,
    b: usize,
    arg: <A as RefFromWasmAbi>::Abi,
) -> <R as ReturnWasmAbi>::Abi {
    if a == 0 {
        throw_str("closure invoked recursively or destroyed already");
    }
    // Scope all local variables before we call `return_abi` to
    // ensure they're all destroyed as `return_abi` may throw
    let ret = {
        let f: &Fn(&A) -> R = mem::transmute((a, b));
        let mut _stack = GlobalStack::new();
        let arg = <A as RefFromWasmAbi>::ref_from_abi(arg, &mut _stack);
        f(&*arg)
    };
    ret.return_abi(&mut GlobalStack::new())
}

impl<'a, A, R> WasmDescribe for Fn(&A) -> R + 'a
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi,
{
    fn describe() {
        inform(FUNCTION);
        inform(invoke1_ref::<A, R> as u32);
        inform(1);
        <&A as WasmDescribe>::describe();
        <R as WasmDescribe>::describe();
    }
}

impl<'a, 'b, A, R> IntoWasmAbi for &'a mut (FnMut(&A) -> R + 'b)
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi,
{
    type Abi = WasmSlice;

    fn into_abi(self, _extra: &mut Stack) -> WasmSlice {
        unsafe {
            let (a, b): (usize, usize) = mem::transmute(self);
            WasmSlice {
                ptr: a as u32,
                len: b as u32,
            }
        }
    }
}

#[allow(non_snake_case)]
unsafe extern "C" fn invoke1_mut_ref<A: RefFromWasmAbi, R: ReturnWasmAbi>(
    a: usize,
    b: usize,
    arg: <A as RefFromWasmAbi>::Abi,
) -> <R as ReturnWasmAbi>::Abi {
    if a == 0 {
        throw_str("closure invoked recursively or destroyed already");
    }
    // Scope all local variables before we call `return_abi` to
    // ensure they're all destroyed as `return_abi` may throw
    let ret = {
        let f: &mut FnMut(&A) -> R = mem::transmute((a, b));
        let mut _stack = GlobalStack::new();
        let arg = <A as RefFromWasmAbi>::ref_from_abi(arg, &mut _stack);
        f(&*arg)
    };
    ret.return_abi(&mut GlobalStack::new())
}

impl<'a, A, R> WasmDescribe for FnMut(&A) -> R + 'a
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi,
{
    fn describe() {
        inform(FUNCTION);
        inform(invoke1_mut_ref::<A, R> as u32);
        inform(1);
        <&A as WasmDescribe>::describe();
        <R as WasmDescribe>::describe();
    }
}
