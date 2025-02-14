//@ compile-flags: --crate-type lib -O -C debug-assertions=yes

// Regression test for issue 118786

macro_rules! make_macro {
    ($macro_name:tt) => {
        macro_rules! $macro_name {
        //~^ ERROR macros that expand to items must be delimited with braces or followed by a semicolon
        //~| ERROR macro expansion ignores token `{` and any following
        //~| ERROR cannot find macro `macro_rules` in this scope
            () => {}
        }
    }
}

make_macro!((meow));
