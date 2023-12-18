//! To simulate a 100% accurate Melee Marth, make Marth absolutely nasty in the air.

use {
    smash::{app::lua_bind::*, hash40, lua2cpp::*},
    smashline::*,
};

pub unsafe extern "C" fn melee_marth_frames(fighter: &mut L2CFighterCommon) {}

pub fn install() {
    Agent::new("marth")
        .on_line(Main, melee_marth_frames)
        .install();
}
