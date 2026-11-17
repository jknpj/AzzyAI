-- Eleanor preset. Loaded after H_Config.lua: only options that differ for Eleanor are listed.
-- Delete a line to use the global H_Config.lua value, or delete this file to disable the preset.
-- Grappling only: without Sonic Claw the fighting chain (Silvervein Rush, Midnight Frenzy) never
-- starts, and GetEleanorPreferredStyle() picks grappling on load and after a revive.
UseEleanorSonicClaw = 0
-- Caprice (Vanilmirth skill inherited by Homunculus S) off for this client.
UseVanilCaprice = 0
-- The One Fighter Rises is the sphere starter: on rAthena it refills all 10 spirit spheres, then the
-- grappling chain (Tinder -> CBC -> EQC) can run. Blazing and Furious stays off: it consumes spheres.
UseEleanorTheOneFighterRises = 1
UseEleanorBlazingAndFurious = 0
-- TOFR must be usable on a single target so it can refill spheres before the chain.
AutoMobCount             = 1
-- The server needs 1 sphere for Tinder Breaker and another for the combo window to stay open.
AutoComboSpheres         = 4
-- The server gives Tinder Breaker 3-7 range, but CBC/EQC are range 1. Fighting at range 1 keeps her in
-- melee (so she takes hits and builds grappling spheres) and leaves her close enough to chain CBC/EQC.
SkillInfo[MH_TINDER_BREAKER][2] = {1,1,1,1,1}
-- The default tactic uses CLASS_S, which only lets Tinder Breaker and CBC be picked. EQC (the last
-- grapple combo) is only selectable with CLASS_GRAPPLE_2.
MyTact[0][TACT_SKILLCLASS] = CLASS_GRAPPLE_2
-- Tinder Breaker has NO cast bar on Hero Saga (instant), but the combo state only opens after the
-- skill animation. The successful log pattern is ~1.1 s after Tinder for CBC and ~0.5 s after CBC
-- for EQC; 250 ms was too early (server refused every CBC).
SkillInfo[MH_TINDER_BREAKER][5] = {0,0,0,0,0}
EleanorComboGap         = 1100
EleanorFollowGap        = 500
-- Keep the AI combo window open long enough for that gap plus a retry (local AzzyUtil patch).
EleanorComboWindow      = 3000
-- The client's learned-level probe is unreliable here; request the minimum level so the server
-- cannot refuse the combo follow-ups for asking for a level above the learned one.
EleanorCBCLevel         = 1
EleanorEQCLevel         = 1
-- This client hides the casting/instant confirmation, so AI_main trusts the expected cast time for
-- Tinder/CBC/EQC (patched locally) instead of marking them failed and clearing the combo.
EleanorTrustCastDelay   = 1
