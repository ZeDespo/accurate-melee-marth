// Holds all the miscellaneous items that are useful to this crate. Globals, structs,
// etv.

use smash::{
    app::{lua_bind::*, *},
    phx::*,
};

// TODO: Implement auto-ken combo
pub const FIGHTER_MARTH_INSTANCE_WORK_ID_FAIR_HIT: i32 = 0x2000eb;

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
