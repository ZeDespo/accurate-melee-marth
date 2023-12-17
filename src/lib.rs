#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]

mod marth;
pub mod vars;

#[skyline::main(name = "melee_marth")]
pub fn main() {
    marth::install()
}
