// Holds all the miscellaneous items that are useful to this crate. Globals, structs,
// etv.

use smash::{
    app::{lua_bind::*, *},
    phx::*,
};

pub const JAB_1_HITSTUN: f32 = 1.7;
pub const JAB_1_HITSTUN_TIPPER: f32 = 2.3;

pub const KEN_COMBO_FAIR_ANGLE: u64 = 67;
pub const FAIR_TIPPER_SIZE: f32 = 5.0;

pub const GRAB_1_SIZE: f32 = 16.0;
pub const GRAB_2_SIZE: f32 = 9.0;

pub const DAIR_TIPPER_ANGLE: u64 = 280;
pub const DAIR_TIPPER_HITSTUN: f32 = 1.75;

pub const UP_THROW_BKB: i32 = 60;
pub const UP_THROW_KBG: i32 = 130;

// Evil angle, may adjust.
pub const DOWN_THROW_ANGLE: u64 = 135;

// Simple struct to pair actions with when we can cancel them.
pub struct CancelFrame {
    pub action: u64,
    pub cancel_frame: f32,
}

impl CancelFrame {
    // Check if we can cancel the given move.
    pub unsafe extern "C" fn is_cancellable(&self, boma: *mut BattleObjectModuleAccessor) -> bool {
        let curr_frame = MotionModule::frame(boma);
        let motion_hash = Hash40::new_raw(MotionModule::motion_kind(boma));
        let cancellable_frame = FighterMotionModuleImpl::get_cancel_frame(boma, motion_hash, false);
        (cancellable_frame - curr_frame) <= self.cancel_frame
    }
}
