use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;

#[acmd_script(
    agent = "samusd",
    scripts =  ["sound_squat", "sound_squatrv"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dsamus_crouch_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		
}
#[acmd_script(
    agent = "samusd",
    script =  "expression_catchpull",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_catchpull_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::phx::Hash40::new("catch_pull"),false,0.0);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "samusd",
    script =  "expression_catchwait",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_catchwait_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_catchattack",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_catchattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("havel"), /*Damage*/ 1.3, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 8.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "samusd",
    script =  "effect_catchattack",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_catchattack_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, /*Frames*/ 3.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
		}
}	
#[acmd_script(
    agent = "samusd",
    script =  "expression_catchattack",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_catchattack_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "samusd",
    script =  "expression_catchcut",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_catchcut_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	

pub fn install() {
    smashline::install_acmd_scripts!(
		dsamus_crouch_sound,
        dsamus_catchpull_expr,
        dsamus_catchwait_expr,
        dsamus_catchattack, dsamus_catchattack_eff, dsamus_catchattack_expr,
        dsamus_catchcut_expr
    );
}