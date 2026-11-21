-- Test scenarios for harness.lua. Each one runs the real AI for ~30 s of game time.
--   homuntype  homunculus S type (EIRA=48, BAYERI=49, SERA=50, DIETER=51, ELEANOR=52)
--   merc       { mertype = N } run AI_M.lua instead of AI.lua for a mercenary (21 = SWORDMAN01)
--   skills     skills the homunculus has learned
--   mobs       monster positions (owner stands at 100,100, all monsters attack the owner);
--              { x, y, mobId } behaves like that rAthena monster (fixtures/rathena.lua RA_MOB):
--              aggressive ones attack the owner, passive ones fight back, plants never move
--   private    true to run with ServerType = SERVER_PRIVATE (the simulated server uses rAthena's skill data)
--   privateIds rAthena-style actor IDs without changing ServerType (wrong-config check)
--   passive    monsters stand still and attack nobody
--   selfMonster the client's IsMonster() also returns 1 for our own homunculus
--   sp/spregen homunculus max SP and SP regained per second
--   startTick  client uptime in milliseconds (default 1000)
--   dropTactics monster classes whose H_Tactics.lua row is removed (tests the defaults for unknown monsters)
--   tactics    { [monster class] = { [tactic index] = value } } replaces those entries of the default row
--   probeFiller skill names the simulated client answers with the "not learned" filler (range + 1)
--              even though they are in `skills`, and distrusted (`TrustSkillProbe = -1`): the
--              official-client behavior that used to disable the skill at startup
--   afterLoad  function run after every AI load, to patch the AI's tables or wrap its functions
--   onTick     function(world, now, OWNER, HOMUN) run before every AI() call
--   reloadAt   tick that simulates a map change: the client reloads every AI file
--   afterReload function(skillKnown) run right after that reload
--   retainer   { id, x, y, target, homuntype } the owner's other homunculus/mercenary in the world
--   player     { id, x, y } an actor IsMonster() reports as a player (SKILL_OBJECT_CMD target)
--   set        config overrides applied on top of fixtures/H_Config.lua
--   expect     skills that must be cast at least once
--   forbid     skills that must never be cast
--   minAttacks minimum normal attacks (default 1)
--   checks     named functions that must return true after the run

local cluster = { {103,100}, {104,101}, {104,99}, {103,102}, {105,100} }
local single = { {103,100} }
-- 40 monsters spread around the owner, for profiling the per-tick cost
local crowd = {}
for i = 0, 39 do crowd[#crowd + 1] = { 100 + (i % 8) * 2 - 7, 100 + math.floor(i / 8) * 3 - 6 } end

-- plants never get skills (H_Tactics: SKILL_NEVER) and the aggressive monster still gets attacked
local function plantChecks()
	return {
		["no skills cast on plants"] = function()
			for _, c in ipairs(HarnessCasts) do
				local m = c.kind == "obj" and HarnessMob(c.a)
				if m and m.ra and m.ra.ai == "06" then error(c.name .. " cast on " .. m.ra.name, 0) end
			end
			return true
		end,
		["the aggressive monster is fought"] = function()
			for _, m in ipairs(HarnessMonsters()) do
				if m.ra and m.ra.ai ~= "06" and (m.hitsTaken or 0) > 0 then return true end
			end
			return RA_MOB == nil
		end,
	}
end

-- Kiting by attack range: a passive monster (the Poring class patched to attack range 9) stands 5 cells
-- from the homunculus. KITE_FIRST = { distance, minimum step } of the first kite the AI starts.
local function kiteScenario(byRange, runChecks)
	local checks = {}
	-- KiteThresholdFor against synthetic MobInfo rows (the scenario's monster takes the class)
	local function thresholdFor(info, base, enabled)
		MobInfo[99990] = info
		HarnessMob(60001).class = 99990
		TactRowCache, MobInfoCache, ActorClassCache = {}, {}, {}
		KiteByRange = enabled
		return KiteThresholdFor(60001, base)
	end
	for name, check in pairs(runChecks) do checks[name] = check end
	checks["melee monster keeps the configured threshold"] = function() return thresholdFor({0,0,1,0,0,1}, 3, 1) == 3 end
	checks["ranged monster starts the kite earlier"] = function() return thresholdFor({0,0,1,0,0,9}, 3, 1) == 11 end
	checks["no kite when its range reaches KiteBounds"] = function() return thresholdFor({0,0,1,0,0,10}, 3, 1) == nil end
	checks["immobile monster only inside its range"] = function()
		return thresholdFor({0,0,1,0,8,9}, 3, 1) == 9 and thresholdFor({0,0,1,0,8,1}, 3, 1) == 1
	end
	checks["off: the configured threshold, whatever the range"] = function() return thresholdFor({0,0,1,0,0,9}, 3, 0) == 3 end
	checks["unknown monster or old MobInfo without range: configured threshold"] = function()
		return thresholdFor(nil, 3, 1) == 3 and thresholdFor({0,0,1,0,0}, 3, 1) == 3
	end
	return { homuntype = 2, skills = {}, mobs = { {106,100} }, passive = true, minAttacks = 0,
		set = { KiteMonsters = 1, ForceKite = 1, KiteByRange = byRange },
		tactics = { [1002] = { [3] = 2 } }, -- TACT_KITE = KITE_ALWAYS
		afterLoad = function()
			MobInfo[1002] = { 0, 0, 1, 0, 0, 9 }
			local real = DoKiteAdjust
			DoKiteAdjust = function(myid, enemy, minstep)
				if KITE_FIRST == nil then KITE_FIRST = { GetDistanceA(myid, enemy), minstep } end
				return real(myid, enemy, minstep)
			end
		end,
		checks = checks }
end

SCENARIOS = {
	mercenary_diagnostics = { homuntype = 2, skills = {}, mobs = single, checks = {
		["mercenary class IDs select known profiles before legacy stat fallbacks"] = function()
			local fakeMerc, fakeClass, fakeHP, fakeSP, fakeReportedType = 900001, 6017, 999, 777, 1
			local realGetV, realMercType = GetV, MercType
			local realSkillList, realSkillInfo = SkillList, SkillInfo
			local realTypeString = TypeString
			dofile("./AI/USER_AI/M_SkillList.lua")
			GetV = function(field, id, extra, extra2)
				if id == fakeMerc then
					if field == V_HOMUNTYPE then return nil end
					if field == V_TYPE then return fakeClass end
					if field == V_MERTYPE then return fakeReportedType end
					if field == V_MAXHP then return fakeHP end
					if field == V_MAXSP then return fakeSP end
					if field == V_SKILLATTACKRANGE and extra == MA_DOUBLE then return 9 end
				end
				return realGetV(field, id, extra, extra2)
			end
			MercType = GetMerType(fakeMerc)
			local archerType = MercType == ARCHER01
			local archerLog = GetMercenaryStartupDiagnostics(fakeMerc)
			fakeClass = 6027
			MercType = GetMerType(fakeMerc)
			local lancerType = MercType == LANCER01
			fakeClass = 6037
			local swordmanType = GetMerType(fakeMerc) == SWORDMAN01
			fakeClass = 6046
			local swordmanTenType = GetMerType(fakeMerc) == SWORDMAN10
			fakeClass = 1965
			local wildRoseType = GetMerType(fakeMerc) == WILDROSE
			fakeClass, fakeHP, fakeSP = 2342, 7513, 201
			MercType = GetMerType(fakeMerc)
			local kasaType = MercType == KASAMERC
			local kasaHasKnownSkills = SkillList[KASAMERC][MER_CRASH] == 5
				and SkillList[KASAMERC][MS_MAGNUM] == 5
			local skill, level = GetMobSkill(fakeMerc)
			local kasaSelectsMobSkill = skill == MS_MAGNUM and level == 5
			skill, level = GetDebuffSkill(fakeMerc)
			local kasaSelectsDebuff = skill == MER_CRASH and level == 5
			fakeClass = 2213
			MercType = GetMerType(fakeMerc)
			skill, level = GetGuardSkill(fakeMerc)
			local wanderSelectsGuard = skill == MS_PARRYING and level == 4
			skill, level = GetMobSkill(fakeMerc)
			local wanderSelectsMobSkill = skill == MS_BOWLINGBASH and level == 5
			fakeClass = 2217
			MercType = GetMerType(fakeMerc)
			skill, level = GetGuardSkill(fakeMerc)
			local teddySelectsDefender = skill == ML_DEFENDER and level == 1
			fakeClass = 3569
			MercType = GetMerType(fakeMerc)
			local popeType = MercType == POPEMERC
			local popeHasKnownSkills = SkillList[POPEMERC][MER_BLESSING] == 3
				and SkillList[POPEMERC][MER_DECAGI] == 1
				and SkillList[POPEMERC][MER_MAGNIFICAT] == 1
			skill, level = GetSOffensiveSkill(fakeMerc)
			local popeSelectsBlessing = skill == MER_BLESSING and level == 3
			skill, level = GetDebuffSkill(fakeMerc)
			local popeSelectsDebuff = skill == MER_DECAGI and level == 1
			fakeClass = 3570
			local casualPopeType = GetMerType(fakeMerc) == POPEMERC
			fakeClass = 2344
			local unsupportedType = GetMerType(fakeMerc) == UNKNOWNMER
			fakeClass, fakeHP, fakeSP = 999999, 300, 220
			local fallbackType = GetMerType(fakeMerc) == ARCHER01
			fakeClass, fakeHP, fakeSP, fakeReportedType = 6018, 999, 777, 3
			local reportedTypeIsPreserved = GetMerType(fakeMerc) == 3
			fakeReportedType = 1
			fakeClass, fakeHP, fakeSP = 2344, 7513, 201
			MercType = GetMerType(fakeMerc)
			local unknownLog = GetMercenaryStartupDiagnostics(fakeMerc)
			TypeString = "M"
			local mercenaryLogPath = GetStartupLogPath()
			TypeString = realTypeString
			GetV, MercType, SkillList, SkillInfo = realGetV, realMercType, realSkillList, realSkillInfo
			return archerType
				and archerLog:find("class=6017", 1, true) ~= nil
				and archerLog:find("reportedType=1", 1, true) ~= nil
				and archerLog:find("maxHP=999", 1, true) ~= nil
				and lancerType and swordmanType and swordmanTenType and wildRoseType and kasaType
				and kasaHasKnownSkills and kasaSelectsMobSkill and kasaSelectsDebuff
				and wanderSelectsGuard and wanderSelectsMobSkill and teddySelectsDefender
				and popeType and casualPopeType and popeHasKnownSkills
				and popeSelectsBlessing and popeSelectsDebuff
				and unsupportedType and fallbackType and reportedTypeIsPreserved
				and unknownLog:find("detectedType=100", 1, true) ~= nil
				and unknownLog:find("profileSkills=[]", 1, true) ~= nil
				and unknownLog:find("observedSkillRanges=[Double Strafe range 9]", 1, true) ~= nil
				and mercenaryLogPath == "AAIStartM.txt"
		end,
	} },
	-- freshly made homunculus: no skills learned, little SP; it must still fight
	amistr_fresh = { homuntype = 2, skills = {}, mobs = cluster, sp = 20, spregen = 1, minAttacks = 100,
		checks = {
			["only normal and Eleanor combo dancing remain"] = function()
				return SkillAnimationDance == nil and QueueSkillAnimationDance == nil
					and TrySkillAnimationDance == nil and IsSkillAnimationDanceEnabled == nil
					and UseDanceAttack == 1 and EleanorComboDance == 1
			end,
			["attack range reuses positions until the next tick"] = function()
				local realGetV, realRange = GetV, AttackRange
				local savedCache = ActorPosCache
				local savedSkill, savedLevel = MySkill, MySkillLevel
				local reads, targetX = 0, 103
				ActorPosCache = {}
				GetV = function(field, id, ...)
					if field == V_POSITION and (id == MyID or id == OwnerID) then
						reads = reads + 1
						if id == MyID then return 100, 100 end
						return targetX, 100
					end
					return realGetV(field, id, ...)
				end
				AttackRange = function(_, skill, level)
					assert(skill == 123 and level == 2, "skill arguments changed")
					return 3
				end
				MySkill, MySkillLevel = 123, 2
				local ok, err = pcall(function()
					assert(IsInAttackSight(MyID, OwnerID, 123, 2), "exact range rejected")
					assert(IsInAttackSight(MyID, OwnerID), "default skill rejected")
					assert(reads == 2, "positions read more than once per actor")
					targetX = 104
					ActorPosCache = {}
					assert(not IsInAttackSight(MyID, OwnerID, 123, 2), "out-of-range target accepted")
					assert(reads == 4, "positions not refreshed after cache reset")
					targetX = -1
					ActorPosCache = {}
					assert(not IsInAttackSight(MyID, OwnerID, 123, 2), "invisible target accepted")
				end)
				GetV, AttackRange = realGetV, realRange
				ActorPosCache = savedCache
				MySkill, MySkillLevel = savedSkill, savedLevel
				if not ok then error(err, 0) end
				return true
			end,
			["fallback settings use the names read by the AI"] = function()
				local defaults = setmetatable({}, { __index = _G })
				setfenv(assert(loadfile("./AI/USER_AI/Defaults.lua")), defaults)()
				local expected = {
					AoEFixedLevel = 1, UseBerserkMobbed = 0, UseBerserkSkill = 0,
					UseBerserkAttack = 0, Berserk_SkillAlways = 0, Berserk_Dance = 0,
					Berserk_IgnoreMinSP = 0,
				}
				for name, value in pairs(expected) do
					assert(rawget(defaults, name) == value, "missing or incorrect fallback: " .. name)
				end
				assert(BerserkMode == 0, "BerserkMode was not initialized")
				return true
			end,
			["Lif level zero skips Urgent Escape without casting"] = function()
				local realBase, realDoSkill = GetHomunBaseType, DoSkill
				local realGuard = GetGuardSkill
				local savedLevel, savedBuff, savedTimeout = LifEscapeLevel, UseDefensiveBuff, GuardTimeout
				local savedAutoTimeout = AutoSkillTimeout
				local savedCooldown = AutoSkillCooldown[HLIF_AVOID]
				local casts, guardCalls = 0, 0
				GetHomunBaseType = function() return LIF end
				DoSkill = function(skill)
					if skill == HLIF_AVOID then casts = casts + 1 end
					return 0
				end
				GetGuardSkill = function(id)
					guardCalls = guardCalls + 1
					return realGuard(id)
				end
				LifEscapeLevel, UseDefensiveBuff, GuardTimeout = 0, 1, 0
				AutoSkillTimeout = 0
				AutoSkillCooldown[HLIF_AVOID] = 0
				local ok, err = pcall(function()
					local skill, level = GetGuardSkill(MyID)
					assert(skill == HLIF_AVOID and level == 0, "zero did not disable Urgent Escape")
					guardCalls = 0
					DoAutoBuffs(1)
					assert(guardCalls == 1 and casts == 0, "Urgent Escape level zero was not safely skipped")
					LifEscapeLevel = 5
					skill, level = GetGuardSkill(MyID)
					assert(skill == HLIF_AVOID and level == 5, "valid Urgent Escape level was changed")
				end)
				GetHomunBaseType, DoSkill = realBase, realDoSkill
				GetGuardSkill = realGuard
				LifEscapeLevel, UseDefensiveBuff, GuardTimeout = savedLevel, savedBuff, savedTimeout
				AutoSkillTimeout = savedAutoTimeout
				AutoSkillCooldown[HLIF_AVOID] = savedCooldown
				if not ok then error(err, 0) end
				return true
			end,
			["targeting diagnostics are off by default"] = function()
				local f = io.open("AAI_TARGETING.log", "r")
				if f then f:close(); return false end
				return LastTargetingLogTick == nil
			end,
			["AAIStartH.txt reports a clean install"] = function()
				local f = assert(io.open("AAIStartH.txt", "r"))
				local text = f:read("*a")
				f:close()
				return text:find("started successfully", 1, true) ~= nil and text:find("Logs enabled: AAI_ERROR", 1, true) ~= nil
			end,
			["AAIStartH.txt shows the release number"] = function()
				local f = assert(io.open("AAIStartH.txt", "r"))
				local text = f:read("*a")
				f:close()
				if type(CVersion) ~= "string" or not CVersion:match("^%d+%.%d+%.%d+$") then
					error("CVersion is not MAJOR.MINOR.PATCH: " .. tostring(CVersion), 0)
				end
				return text:find("\nRelease: " .. CVersion .. "\n", 1, true) ~= nil
			end,
			["the file-consistency numbers follow the release"] = function()
				local expected = CVersion:match("^(%d+%.%d+)%.%d+$")
				if not (Version == expected and MainVersion == expected and AUVersion == expected) then
					error(("release %s needs %s, got Version=%s Main=%s AU=%s"):format(
						CVersion, expected, tostring(Version), tostring(MainVersion), tostring(AUVersion)), 0)
				end
				return true
			end,
			["a file from another installation is reported"] = function()
				-- what AI.lua does: read MAJOR.MINOR out of each number (the old 1.56 and the
				-- release 2.0.0 alike) and compare it with Version
				local verspattern = "%d+%.%d+"
				return string.gfind(CVersion, verspattern)() == Version
					and string.gfind(AUVersion, verspattern)() == Version
					and string.gfind("1.56", verspattern)() ~= Version
					and string.gfind("1.30b or earlier", verspattern)() ~= Version
			end,
		} },
	amistr_fresh_alt = { homuntype = 6, skills = {}, mobs = cluster, sp = 20, spregen = 1, minAttacks = 100 },
	amistr_unlearned_bulwark = { homuntype = 2, skills = {}, mobs = single, private = true, passive = true,
		sp = 100, spregen = 1, minAttacks = 100, startTick = 10000000 },
	amistr_bulwark_exact_sp = { homuntype = 2, skills = { "HAMI_DEFENCE:1" }, mobs = single,
		sp = 20, spregen = 0, minAttacks = 100, startTick = 10000000, set = { AmiBulwarkLevel = 1 } },
	amistr_bulwark_learned = { homuntype = 2, skills = { "HAMI_DEFENCE" }, mobs = single,
		sp = 100, spregen = 1, minAttacks = 100, startTick = 10000000, expect = { "Amistr Bulwark" } },
	-- rAthena Bulwark Lv5 lasts 20 s but has a 30 s cooldown; recasting when the buff dropped
	-- (and again after every resurrection/relog) only got "Skill failed". The AI is reloaded
	-- mid-run like after a resurrection; the saved cooldown must still be honoured.
	amistr_bulwark_private_cooldown = { homuntype = 2, skills = { "HAMI_DEFENCE" }, mobs = single,
		private = true, sp = 300, spregen = 5, minAttacks = 100, startTick = 10000000,
		ticks = 1300, reloadAt = 450, expect = { "Amistr Bulwark" },
		checks = {
			["Bulwark is recast as soon as its rAthena cooldown ends"] = function()
				local n = 0
				for _, c in ipairs(HarnessCasts) do
					if c.ok and c.skill == HAMI_DEFENCE then n = n + 1 end
				end
				return n >= 4 and GetSkillInfo(HAMI_DEFENCE, 9, 5) == 30000
			end,
		} },
	amistr_unlearned_smart_bulwark = { homuntype = 2, skills = {}, mobs = single,
		sp = 300, spregen = 1, minAttacks = 100, startTick = 10000000, set = { UseSmartBulwark = 1 } },
	amistr_fresh_single = { homuntype = 2, skills = {}, mobs = single, sp = 20, spregen = 1, minAttacks = 100 },
	amistr_castle_defend = { homuntype = 2, skills = { "HAMI_CASTLE:5" }, mobs = cluster, sp = 100, spregen = 1, minAttacks = 0,
		set = { UseCastleDefend = 1, CastleDefendThreshold = 1 }, maxRefused = 2 },
	-- passive monsters (nobody attacking): the homunculus must still go and attack them
	amistr_fresh_passive = { homuntype = 2, skills = {}, mobs = cluster, passive = true, sp = 20, spregen = 1, minAttacks = 50 },
	amistr_fresh_private = { homuntype = 2, skills = {}, mobs = cluster, private = true, sp = 20, spregen = 1, minAttacks = 100 },
	amistr_fresh_private_passive = { homuntype = 2, skills = {}, mobs = cluster, private = true, passive = true,
		sp = 20, spregen = 1, minAttacks = 50 },
	-- rAthena IDs but the config was left on SERVER_OFFICIAL: monsters looked like players and were never attacked
	amistr_private_ids_official_config = { homuntype = 2, skills = {}, mobs = cluster, privateIds = true, passive = true,
		sp = 20, spregen = 1, minAttacks = 50 },
	-- the client's IsMonster() reports our own homunculus as a monster: it used to target itself
	-- (always the closest "monster") and never attack anything
	amistr_self_reported_monster = { homuntype = 2, skills = {}, mobs = cluster, private = true, passive = true,
		selfMonster = true, sp = 20, spregen = 1, minAttacks = 50 },
	amistr_self_reported_monster_official = { homuntype = 2, skills = {}, mobs = cluster, passive = true,
		selfMonster = true, sp = 20, spregen = 1, minAttacks = 50 },
	amistr_targeting_log = { homuntype = 2, skills = {}, mobs = single, privateIds = true, passive = true,
		sp = 20, spregen = 1, minAttacks = 50,
		set = { LogEnable = { AAI_ERROR = 1, AAI_TARGETING = 1 } },
		checks = {
			["targeting snapshots include classification and are rate limited"] = function()
				local f = assert(io.open("AAI_TARGETING.log", "r"))
				local text = f:read("*a")
				f:close()
				local _, count = text:gsub("Snapshot tick=", "")
				return count == 6 and text:find("Actor 110000001 nativeMonster=1 player=0", 1, true) ~= nil
					and text:find("inTargets=true", 1, true) ~= nil
					and text:find("KS=not KS, not targeted by anything", 1, true) ~= nil
			end,
		} },
	amistr_targeting_passive_log = { homuntype = 2, skills = {}, mobs = single, private = true, passive = true,
		sp = 20, spregen = 1, minAttacks = 0,
		set = { SuperPassive = 1, LogEnable = { AAI_ERROR = 1, AAI_TARGETING = 1 } },
		checks = {
			["passive blocker is visible without changing targeting"] = function()
				local f = assert(io.open("AAI_TARGETING.log", "r"))
				local text = f:read("*a")
				f:close()
				return MyEnemy == 0 and text:find("SuperPassive=1", 1, true) ~= nil
					and text:find("inTargets=true", 1, true) ~= nil
			end,
		} },
	-- damage-aware defence: HP drops inside the 3 s window trigger it only below DamageDangerHP;
	-- heals and old samples do not count
	-- AoE must not be wasted on a single target (AutoMobCount = 2)
	-- low SP compares by damage per SP, where Glanzen Spies beats Stahl Horn, so it may appear late
	-- Glanzen Spies ignores DEF: it only replaces Stahl Horn against a high-DEF target
	-- a pair of monsters only gets the DEF-ignoring area skills when their DEF is high
	-- both AoEs on and fully learned: the cheap ranged Stange must not starve the caster-centered Pferd
	-- rAthena timing (no 1 s after-cast delay): Pferd scores higher, Stange fills its cooldown
	-- regression: Stein Wand used to be spammed without checking SP (the harness fails any cast without SP)
	-- partly levelled homunculus: hard-coded Lv5 casts (Stein Wand) and configured levels above the
	-- learned one must be lowered before casting. Unlearned Pferd has ambiguous range 1,
	-- so it requires the same eight bounded probes as bayeri_pferd_unlearned below.
	-- Heilige Stange is off like in bayeri_pferd: Lv2 casts in 3 s with no cooldown, so the AI would
	-- chain it from range 9 and never get close enough for the caster-centered Pferd (or melee).
	-- Glanzen Spies is off too, or it spends the SP the Pferd probes need.
	-- the cheap Stange chain above, with Stange on: casts stay at the learned Lv2
	-- Heilige Pferd not learned but enabled: its Lv1-4 have range 1 so the probe can't tell. The AI
	-- steps down after refusals and remembers it in data/; after a map change (AI reload) it doesn't
	-- try again until LEARNED_SKILL_RECHECK_SECONDS have passed.

	-- regression: swapped duration/reuse made Magma Flow recast forever and Dieter never attacked
	-- The official client answers some learned skills with the "not learned" filler (see
	-- ProbeAnswerPlausible). On LATAM that came back for Lava Slide: the startup lint disabled
	-- UseDieterLavaSlide and DoSkill skipped it, so the Dieter never cast it. The enabled skill has
	-- to be tried; the server is the only authority on its level. OldHomunType 0 keeps inherited
	-- base-homunculus skills (which this Dieter has not learned either) out of the run.
	-- the same client answer for a skill the homunculus never learned: the ladder ends at 0, so the
	-- AI stops requesting a cast the server refuses instead of resending it forever
	-- regression: GetTick() resets with Windows, so yesterday's saved buff expiry looked hours away
	-- regression: Blast Forge has range 1, which the old learned-skill check read as "not learned"
	-- Lava Slide allows one active instance, so Volcanic Ash (a debuff) is used as an attack skill
	-- stay away from the owner (instances with an ungeared owner)
	-- MVPs: Baphomet (1039) is a passive MVP here; BossAggro 1 alone leaves it alone
	-- per-type preset (Types/Dieter.lua): AutoMobCount 1 lets Dieter use his area skills on one monster

	-- regression: Toxin Lv1-4 has range 1, so its learned level can't be probed; the old check
	-- reported Lv4 and the AI spammed refused casts ("Skill failed") forever
	-- after a map change the saved level is used right away: no refused casts
	-- the player levelled Toxin between sessions: the periodic level-up try picks it up


	-- skill picking by SkillDamage score: Twister Cutter beats Eraser Cutter on a neutral target...
	-- ...but not on Earth 1, which takes 90% from Wind
	-- low SP: compared by damage per SP, so the cheap Eraser Cutter wins

	-- a pack is worth her AoE: SP is saved for The One Fighter Rises instead of being spent on combo chains
	-- a lone target only gets The One Fighter Rises when its DEF makes the combos too weak
	-- Grappling without spheres while nothing hits her (the owner tanks): she can't start Tinder
	-- Breaker and would never gain spheres, so she switches back to fighting style
	-- UseEleanorSonicClaw=0 must also stop the grapple fallback: it used to cast Sonic Claw to
	-- recover spheres and switched her to fighting style against the preset

	-- many monsters on screen: must keep fighting and not error out
	-- GetBestAoECoord / GetBestAoETarget (optimized) must pick exactly what the old brute-force
	-- search picks, on 40 monsters, for ground and target-centered AoEs of several sizes

	-- rAthena data (fixtures/rathena.lua, regenerated by rathena_sync.py) --------------------
	-- the test server's passive benchmark: Eira and a zero-defense Neutral Poring for 60 seconds,
	-- no extra delay between skills. Run with HARNESS_DAMAGE=1 to see the expected damage.
	-- Eleanor on rAthena: Sonic Claw needs a sphere, which only normal attacks in fighting style give
	-- The AI asks for an attack every tick but the estimate must throttle those commands.
	-- real monsters: plants (Ai 06) next to an aggressive Zombie
	-- passive Porings (Ai 02) don't attack anyone until the homunculus hits them
	porings_private = { homuntype = 2, skills = {}, mobs = { {103,100,1002}, {104,101,1002} }, private = true,
		sp = 20, spregen = 1, minAttacks = 50,
		checks = {
			["hit Porings fight back"] = function()
				for _, m in ipairs(HarnessMonsters()) do
					if (m.hitsTaken or 0) > 0 and m.target ~= 2001 and m.ra then return false end
				end
				return true
			end,
		} },

	-- Monster database (USER_AI/MobInfo.lua) ------------------------------------------------
	mobinfo_data = { homuntype = 2, skills = {}, mobs = single, sp = 20, spregen = 1,
		checks = {
			["MobInfo.lua is loaded (run tools/harness/rathena_sync.py)"] = function() return MobInfo ~= nil end,
			["Poring: Water 1, no flags"] = function()
				local p = MobInfo[1002]
				return p[2] == 1 and p[3] == 1 and p[5] == 0
			end,
			["Baphomet is a boss MVP, Red Plant a plant"] = function()
				return HasMobFlag(MobInfo[1039], MOB_FLAG_MVP) and HasMobFlag(MobInfo[1039], MOB_FLAG_BOSS)
					and HasMobFlag(MobInfo[1078], MOB_FLAG_PLANT) and not HasMobFlag(MobInfo[1002], MOB_FLAG_PLANT)
			end,
			["skill elements: Lava Slide fire, Heilige Stange holy"] = function()
				return SkillElement[MH_LAVA_SLIDE] == 3 and SkillElement[MH_HEILIGE_STANGE] == 6
			end,
			["element table: fire on water 1 = 50%, fire on fire 2 = 0%"] = function()
				return ElementFix[1][3][1] == 50 and ElementFix[2][3][3] == 0
			end,
		} },
	-- the default limit (50%) still lets fire hit a Water 1 Poring (90%)...
	-- ...a stricter limit makes Dieter leave its fire attacks out against it
	-- UseMobInfo = 0 brings back the old behavior
	-- an MVP standing next to a Poring: with BossAggro = 1 (default) the homunculus leaves it alone
	boss_mvp_ignored = { homuntype = 2, skills = {}, mobs = { {103,100,1039}, {98,100,1002} }, passive = true,
		sp = 20, spregen = 1, minAttacks = 20,
		checks = {
			["the MVP is never attacked, the Poring is"] = function()
				local boss, poring = 0, 0
				for _, m in ipairs(HarnessMonsters()) do
					if m.class == 1039 then boss = m.hitsTaken or 0 else poring = poring + (m.hitsTaken or 0) end
				end
				return boss == 0 and poring > 0
			end,
		} },
	-- ...but defends the owner when the MVP attacks
	boss_mvp_defend = { homuntype = 2, skills = {}, mobs = { {103,100,1039} }, sp = 20, spregen = 1, minAttacks = 20 },
	-- BossAggro = 0: tactics decide, the default tactic attacks it
	boss_mvp_tactics = { homuntype = 2, skills = {}, mobs = { {103,100,1039} }, passive = true,
		sp = 20, spregen = 1, minAttacks = 20, set = { BossAggro = 0 } },
	-- plants without a tactics row (they normally have one): no skills on them (PlantSkills = 0)
	-- Mixed elements: a Fire 2 monster (immune to fire) in front of three Earth 2 ones (175% from fire).
	-- Dieter must still use Lava Slide as an attack (the pack makes it worth it) and lay it so
	-- the Earth pack is inside.
	-- only Fire monsters: no fire skill at all
	-- KiteByRange: the monster (range 9, not attacking anybody) uses KiteParanoidThreshold 2, so the
	-- kite starts at 2 + 9 - 1 = 10 cells with a step that takes us out of its range; off, it only
	-- starts within KiteParanoidThreshold
	kite_by_range = kiteScenario(1, {
		["kites right away, with a step that leaves its range"] = function()
			local k = KITE_FIRST
			if k == nil then error("the AI never kited", 0) end
			if not (k[1] > 2 and k[2] ~= nil and k[1] + k[2] > 9) then
				error(("first kite at distance %s with step %s"):format(tostring(k[1]), tostring(k[2])), 0)
			end
			return true
		end }),
	-- KiteParanoid: kite a monster that is not on us yet, with KiteParanoidThreshold instead of
	-- KiteThreshold. The monster is KITE_REACT (by tactic) and passive, so only the option can
	-- start the kite.
	kite_paranoid = { homuntype = 2, skills = {}, mobs = { {103,100} }, passive = true, minAttacks = 0,
		set = { KiteMonsters = 1, ForceKite = 1, KiteParanoid = 1, KiteByRange = 0, UseIdleWalk = 0 },
		tactics = { [1002] = { [1] = 0, [3] = 1 } }, -- TACT_BASIC = TACT_IGNORE, TACT_KITE = KITE_REACT
		afterLoad = function()
			local real = DoKiteAdjust
			DoKiteAdjust = function(myid, enemy, minstep)
				if KITE_FIRST == nil then KITE_FIRST = { GetDistanceA(myid, enemy), minstep } end
				return real(myid, enemy, minstep)
			end
		end,
		checks = {
			["kites a monster that is not attacking anyone yet"] = function()
				local k = KITE_FIRST
				return k ~= nil and k[1] <= 2
			end,
		} },

	kite_paranoid_off = { homuntype = 2, skills = {}, mobs = { {103,100} }, passive = true, minAttacks = 0,
		set = { KiteMonsters = 1, ForceKite = 1, KiteByRange = 0, UseIdleWalk = 0 },
		tactics = { [1002] = { [1] = 0, [3] = 1 } }, -- TACT_BASIC = TACT_IGNORE, TACT_KITE = KITE_REACT
		afterLoad = function()
			local real = DoKiteAdjust
			DoKiteAdjust = function(myid, enemy, minstep)
				if KITE_FIRST == nil then KITE_FIRST = { GetDistanceA(myid, enemy), minstep } end
				return real(myid, enemy, minstep)
			end
		end,
		checks = {
			["without the option it does not kite a monster that ignores us"] = function()
				return KITE_FIRST == nil
			end,
		} },

	kite_range_off = kiteScenario(0, {
		["does not kite until the monster is within KiteParanoidThreshold"] = function()
			local k = KITE_FIRST
			return k == nil or (k[1] <= 2 and k[2] == nil)
		end }),
	-- TargetPriority: SelectEnemy on a fixed set of equal-tactics monsters (plant, Fire 2 immune to
	-- Lava Slide, an isolated Earth 2 monster and a pack of three), with TargetPriority 0 and 1

	-- ------------------------------------------------------------------
	-- Unit-style regressions for paths the timed scenarios above never take.
	-- ------------------------------------------------------------------

	-- TargetPriority=1 ranks targets by the AoE cluster they would be in. The cluster table is
	-- built from Targets, but SelectEnemy also gets an opportunistic "current enemy" that may have
	-- left the list (the monster being left for a better one). It must still get its weight
	-- computed on demand instead of being ranked as cluster 0.

	-- Losing the target has to clear the per-target skill counter: it gates tactics with a
	-- finite cast count (MyTact rows with -1/-3/-4, e.g. Orc Zombie, Zenorc).
	skill_counter_reset = { homuntype = 2, skills = {}, mobs = single, passive = true,
		minAttacks = 0, ticks = 3,
		afterLoad = function()
			local realAI = AI
			local done = false
			AI = function(id)
				realAI(id)
				if done then return end
				done = true
				local realIdle, realFollow = OnIDLE_ST, OnFOLLOW_ST
				OnIDLE_ST, OnFOLLOW_ST = function() end, function() end
				MyState, MyEnemy, MySkillUsedCount = ATTACK_ST, 0, 7
				OnATTACK_ST() -- takes the "target gone" branch
				OnIDLE_ST, OnFOLLOW_ST = realIdle, realFollow
				PROBE_SKILL_COUNT = MySkillUsedCount
			end
		end,
		checks = {
			["losing the target resets MySkillUsedCount"] = function()
				return PROBE_SKILL_COUNT == 0
			end,
		} },

	-- GetBestBrandishTarget must return the actor id; Targets[id] is the {motion, class} row.
	brandish_target_id = { homuntype = 2, skills = {}, mobs = { {102,100} }, passive = true,
		minAttacks = 0, ticks = 3,
		afterLoad = function()
			local realAI = AI
			local done = false
			AI = function(id)
				realAI(id)
				if done then return end
				done = true
				PROBE_BRANDISH = GetBestBrandishTarget(MyID)
			end
		end,
		checks = {
			["GetBestBrandishTarget returns an actor id"] = function()
				return type(PROBE_BRANDISH) == "number" and Targets[PROBE_BRANDISH] ~= nil
			end,
		} },

	-- convpriority(): a react-low monster that is attacking us must rank like attack-low,
	-- not below every other monster (SelectEnemy compares these numbers).
	convpriority_values = { homuntype = 2, skills = {}, mobs = single, passive = true,
		minAttacks = 0, ticks = 2,
		checks = {
			["react-low while attacked ranks like attack-low"] = function()
				return convpriority(TACT_REACT_L, 1) == 2
			end,
			["react-low while ignored stays at the bottom"] = function()
				return convpriority(TACT_REACT_L, 0) == 0
			end,
			["attack-low and attack-mid keep their order"] = function()
				return convpriority(TACT_ATTACK_L, 1) == 1 and convpriority(TACT_ATTACK_M, 1) == 4
			end,
			["react-mid and react-high keep their values"] = function()
				return convpriority(TACT_REACT_M, 1) == TACT_REACT_M
					and convpriority(TACT_REACT_H, 1) == TACT_REACT_H
			end,
			["ignore stays at the bottom"] = function()
				return convpriority(TACT_IGNORE, 1) == 0
			end,
		} },

	-- Owner sits down: the AI must rest at RestXOff/RestYOff from the owner and leave REST_ST
	-- when the owner stands up.
	rest_owner_sits = { homuntype = 2, skills = {}, mobs = {}, passive = true,
		minAttacks = 0, ticks = 70,
		onTick = function(world, now, owner)
			restTick = (restTick or 0) + 1
			if restTick == 10 then world[owner].motion = MOTION_SIT end
			if restTick == 40 then world[owner].motion = MOTION_STAND end
		end,
		afterLoad = function()
			restTick = 0
			local realAI = AI
			AI = function(id)
				realAI(id)
				if restTick >= 12 and restTick <= 38 and MyState == REST_ST then SAW_REST = true end
				if restTick >= 45 and MyState ~= REST_ST then LEFT_REST = true end
			end
		end,
		checks = {
			["owner sitting puts the homunculus in REST_ST"] = function() return SAW_REST == true end,
			["owner standing leaves REST_ST"] = function() return LEFT_REST == true end,
		} },

	-- Patrol command: sets the state and destination, and the homunculus walks towards it.
	patrol_command = { homuntype = 2, skills = {}, mobs = {}, passive = true,
		minAttacks = 0, ticks = 30,
		afterLoad = function()
			local realAI = AI
			local issued = false
			AI = function(id)
				realAI(id)
				if not issued then
					issued = true
					OnPATROL_CMD(110, 100)
					PATROL_STATE, PATROL_DEST, PATROL_FROM = MyState, MyDestX, MyPatrolX
				elseif GetV(V_POSITION, MyID) ~= 101 then
					MOVED_TO_PATROL = true
				end
			end
		end,
		checks = {
			["patrol command sets PATROL_CMD_ST and the destination"] = function()
				return PATROL_STATE == PATROL_CMD_ST and PATROL_DEST == 110 and PATROL_FROM == 101
			end,
			["the homunculus moves towards the patrol point"] = function() return MOVED_TO_PATROL == true end,
		} },

	-- Tank tactic: a passive monster marked TACT_TANK must be picked by the tank list (normal
	-- targeting excludes -1). The owner is not attacked, so only the tank path can engage it.
	tank_tactic = { homuntype = 2, skills = {}, mobs = { {110,100} }, passive = true,
		minAttacks = 0, ticks = 40,
		-- -1 = TACT_TANK (this file is parsed before Const_.lua defines the constants)
		tactics = { [1002] = { [1] = -1 } },
		checks = {
			["a passive monster with a tank tactic is engaged"] = function()
				return MyEnemy == 60001 and (MyState == TANKCHASE_ST or MyState == TANK_ST)
			end,
		} },

	-- A monster attacked by the owner's retainer (another homunculus/mercenary) must be reachable
	-- as a friend target. The monster is REACT_L and passive, so the normal enemy list never
	-- includes it: only the FriendTargets built from the actor loop can pick it.
	friend_target_retainer = { homuntype = 2, skills = {}, mobs = { {120,100} }, passive = true,
		minAttacks = 0, ticks = 80,
		-- 5 = TACT_REACT_L (this file is parsed before Const_.lua defines the constants)
		tactics = { [1002] = { [1] = 5 } },
		retainer = { id = 3001, x = 118, y = 100, target = 1 },
		afterLoad = function()
			MyFriends[3001] = RETAINER
		end,
		checks = {
			["a monster attacked by a retainer becomes the homunculus target"] = function()
				return MyEnemy == 60001
			end,
		} },

	-- Legacy HOLD command (OnHOLD_CMD -> HOLD_CMD_ST): the homunculus stands its ground and hits
	-- an attacker that is already inside its attack range.
	hold_command = { homuntype = 2, skills = {}, mobs = { {102,100} },
		minAttacks = 1, ticks = 60,
		afterLoad = function()
			OnHOLD_CMD()
			HOLD_STATE, HOLD_DESTX, HOLD_DESTY = MyState, MyDestX, MyDestY
			local realAI = AI
			AI = function(id)
				realAI(id)
				local x, y = GetV(V_POSITION, MyID)
				if MyState ~= HOLD_CMD_ST then HOLD_LEFT = true end
				if x ~= 101 or y ~= 100 then HOLD_MOVED = true end
			end
		end,
		checks = {
			["the hold command sets HOLD_CMD_ST and clears the destination"] = function()
				return HOLD_STATE == HOLD_CMD_ST and HOLD_DESTX == 0 and HOLD_DESTY == 0
			end,
			["the homunculus never leaves HOLD_CMD_ST"] = function() return HOLD_LEFT ~= true end,
			["an attacker in reach is attacked"] = function() return (HarnessMob(60001).hitsTaken or 0) > 0 end,
		} },

	-- HOLD has no chase: an attacker out of reach is dropped instead of followed. The AI is put in
	-- HOLD_CMD_ST before the first tick, so the idle targeting never runs.
	hold_command_range = { homuntype = 2, skills = {}, mobs = { {103,100} },
		minAttacks = 0, ticks = 60,
		afterLoad = function()
			OnHOLD_CMD()
			local realAI = AI
			AI = function(id)
				realAI(id)
				if MyState ~= HOLD_CMD_ST then HOLD_LEFT = true end
			end
		end,
		checks = {
			["an attacker out of reach is neither chased nor attacked"] = function()
				local h = HarnessMob(2001)
				return HOLD_LEFT ~= true and (h.attacks or 0) == 0 and h.x == 101 and h.y == 100
			end,
		} },

	-- Legacy ground-skill command (OnSKILL_AREA_CMD -> SKILL_AREA_CMD_ST): out of range of the
	-- commanded cell the homunculus walks into range first, then casts there and returns to IDLE.

	-- Map change: the client reloads every AI file and the party reappears somewhere else. The
	-- reloaded AI must start from a fresh state, apply the per-type preset again and fight the
	-- monsters of the new map without a single cast the server refuses.

	-- Legacy MOVE command (OnMOVE_CMD -> MOVE_CMD_ST): walks to the ordered cell, goes back to
	-- IDLE_ST on arrival, and a bogus destination (off the owner's screen) is disregarded without
	-- touching the state or the current order.
	move_command = { homuntype = 2, skills = {}, mobs = {}, passive = true,
		minAttacks = 0, ticks = 60, set = { UseIdleWalk = 0 },
		afterLoad = function()
			local realAI = AI
			local phase = 1
			AI = function(id)
				realAI(id)
				local x, y = GetV(V_POSITION, MyID)
				if phase == 1 then
					OnMOVE_CMD(106, 100)
					MOVE_STATE = MyState
					phase = 2
				elseif phase == 2 and x >= 106 and MyState ~= MOVE_CMD_ST then
					MOVE_ARRIVED_STATE = MyState
					MOVE_ARRIVED_X = x
					local stateBefore, orderBefore = MyState, MyMoveX
					-- 30 cells from the owner: the client would never accept this
					OnMOVE_CMD(130, 100)
					MOVE_BAD_SAME = MyState == stateBefore and MyMoveX == orderBefore
					phase = 3
				end
			end
		end,
		checks = {
			["the move command sets MOVE_CMD_ST"] = function() return MOVE_STATE == MOVE_CMD_ST end,
			["the homunculus walks to the ordered cell and leaves MOVE_CMD_ST"] = function()
				return MOVE_ARRIVED_X == 106 and MOVE_ARRIVED_STATE ~= MOVE_CMD_ST
			end,
			["a destination outside the owner's screen is disregarded"] = function()
				return MOVE_BAD_SAME == true and HarnessMob(2001).x < 110
			end,
		} },

	-- Legacy STOP command (OnSTOP_CMD): drops the chase and the pending skill and goes back to
	-- IDLE_ST, so the next tick targets again from scratch.
	stop_command = { homuntype = 2, skills = {}, mobs = { {106,100} },
		minAttacks = 0, ticks = 40,
		afterLoad = function()
			local realAI = AI
			local n = 0
			AI = function(id)
				realAI(id)
				n = n + 1
				if n == 25 then
					STOP_BEFORE_STATE, STOP_BEFORE_ENEMY = MyState, MyEnemy
					OnSTOP_CMD()
					STOP_STATE, STOP_ENEMY = MyState, MyEnemy
					STOP_DESTX, STOP_DESTY, STOP_SKILL = MyDestX, MyDestY, MySkill
				end
			end
		end,
		checks = {
			["the homunculus was chasing before the stop"] = function()
				return (STOP_BEFORE_STATE == CHASE_ST or STOP_BEFORE_STATE == ATTACK_ST)
					and STOP_BEFORE_ENEMY == 60001
			end,
			["the stop command returns to IDLE_ST and clears the order"] = function()
				return STOP_STATE == IDLE_ST and STOP_ENEMY == 0 and STOP_DESTX == 0 and STOP_DESTY == 0
					and STOP_SKILL == 0
			end,
		} },

	-- Legacy ATTACK_AREA command (OnATTACK_AREA_CMD -> ATTACK_AREA_CMD_ST): an enemy of the owner
	-- inside the area becomes the target, so the area order turns into a chase.
	attack_area_command = { homuntype = 2, skills = {}, mobs = { {103,100} },
		minAttacks = 1, ticks = 60,
		afterLoad = function()
			local realAI = AI
			local issued = false
			AI = function(id)
				realAI(id)
				if not issued then
					issued = true
					OnATTACK_AREA_CMD(108, 105)
					AREA_STATE, AREA_DESTX = MyState, MyDestX
					AREA_ENEMY = MyEnemy
				end
			end
		end,
		checks = {
			["the area command sets ATTACK_AREA_CMD_ST with no target yet"] = function()
				return AREA_STATE == ATTACK_AREA_CMD_ST and AREA_DESTX == 108 and AREA_ENEMY == 0
			end,
			["an enemy of the owner inside the area is chased"] = function()
				return MyEnemy == 60001 and (MyState == CHASE_ST or MyState == ATTACK_ST)
			end,
		} },

	-- Same command with nothing to fight: the homunculus walks to the area and then goes back to
	-- IDLE_ST (the arrival branch).
	attack_area_arrival = { homuntype = 2, skills = {}, mobs = {}, passive = true,
		minAttacks = 0, ticks = 60, set = { UseIdleWalk = 0 },
		afterLoad = function()
			local realAI = AI
			local issued = false
			AI = function(id)
				realAI(id)
				local x, y = GetV(V_POSITION, MyID)
				if not issued then
					issued = true
					OnATTACK_AREA_CMD(104, 100)
					AREA_SEEN = MyState == ATTACK_AREA_CMD_ST
				elseif x == 104 and y == 100 and MyState == IDLE_ST then
					AREA_ARRIVED_STATE = MyState
					AREA_ARRIVED_X = x
				end
			end
		end,
		checks = {
			["the area command is entered and then left at the destination"] = function()
				return AREA_SEEN == true and AREA_ARRIVED_X == 104 and AREA_ARRIVED_STATE == IDLE_ST
			end,
		} },

	-- Legacy FOLLOW command (OnFOLLOW_CMD): the first call follows the owner at FollowStayBack,
	-- a second call while following goes back to IDLE_ST and the homunculus stops following.
	follow_command = { homuntype = 2, skills = {}, mobs = {}, passive = true,
		minAttacks = 0, ticks = 120,
		set = { FollowStayBack = 2, UseIdleWalk = 0 },
		afterLoad = function()
			local realAI = AI
			local n = 0
			AI = function(id)
				realAI(id)
				n = n + 1
				if n == 1 then
					OnFOLLOW_CMD()
					FOLLOW_STATE = MyState
				elseif n == 40 then
					FOLLOW_TOGGLE_STATE = MyState
					OnFOLLOW_CMD()
					FOLLOW_OFF_STATE = MyState
				end
				if MyState == FOLLOW_CMD_ST and FOLLOW_OFF_STATE ~= nil then FOLLOW_REENTERED = true end
			end
		end,
		onTick = function(world, now, owner, homun)
			-- the owner walks away while the homunculus follows, then keeps walking after the toggle
			if now > 2000 and now <= 3400 then world[owner].x = world[owner].x + 1 end
			if now > 5000 and now <= 7300 then world[owner].x = world[owner].x + 1 end
			local gap = math.abs(world[owner].x - world[homun].x)
			if FOLLOW_OFF_STATE == nil then
				if gap > (FOLLOW_MAX_GAP or 0) then FOLLOW_MAX_GAP = gap; FOLLOW_MAX_GAP_AT = now end
			elseif gap > (IDLE_MAX_GAP or 0) then
				IDLE_MAX_GAP = gap
			end
		end,
		checks = {
			["the follow command sets FOLLOW_CMD_ST"] = function() return FOLLOW_STATE == FOLLOW_CMD_ST end,
			["the homunculus follows the walking owner"] = function()
				return FOLLOW_TOGGLE_STATE == FOLLOW_CMD_ST and HarnessMob(2001).x >= 108
			end,
			["while following it stays within FollowStayBack"] = function()
				return (FOLLOW_MAX_GAP or 99) <= 3
			end,
			["a second follow command goes back to IDLE_ST"] = function()
				return FOLLOW_OFF_STATE == IDLE_ST and FOLLOW_REENTERED ~= true
			end,
			["only the idle distance rule applies again after the toggle"] = function()
				return (IDLE_MAX_GAP or 0) >= 4
			end,
		} },

	-- SKILL_OBJECT_CMD on a player: not a monster, so the command uses SKILL_OBJECT_CMD_ST, walks
	-- into the skill range and casts on that actor.

	-- ATTACK_OBJECT_CMD: an explicit order to fight a monster the idle targeting would leave alone
	-- (passive, not attacking the owner). It also sets CommandedEnemy and bypasses KS protection.
	attack_object_command = { homuntype = 2, skills = {}, mobs = { {106,100} }, passive = true,
		minAttacks = 1, ticks = 60,
		afterLoad = function()
			local realAI = AI
			local issued = false
			AI = function(id)
				realAI(id)
				if not issued then
					issued = true
					OnATTACK_OBJECT_CMD(60001)
					OBJ_STATE, OBJ_ENEMY = MyState, MyEnemy
					OBJ_COMMANDED, OBJ_BYPASS = CommandedEnemy, BypassKSProtect
				end
			end
		end,
		checks = {
			["the ordered monster becomes the target"] = function()
				return OBJ_STATE == CHASE_ST and OBJ_ENEMY == 60001 and OBJ_COMMANDED == 60001
					and OBJ_BYPASS == 1
			end,
			["the homunculus reaches and attacks it"] = function()
				return (HarnessMob(60001).hitsTaken or 0) > 0
			end,
		} },

	-- CommandedFightStay: an ordered monster is fought until it dies even when the owner walks
	-- away. The owner leaves during the fight, so the AI() "too far from the owner" guard would
	-- pull the homunculus into FOLLOW_ST; the option exempts MyEnemy == CommandedEnemy.
	commanded_fight_stay = { homuntype = 2, skills = {}, mobs = { {103,100} }, passive = true,
		minAttacks = 1, ticks = 140, set = { CommandedFightStay = 1 },
		afterLoad = function()
			local realAI = AI
			local issued = false
			AI = function(id)
				realAI(id)
				if not issued then
					issued = true
					OnATTACK_OBJECT_CMD(60001)
				end
			end
		end,
		onTick = function(world, now, owner, homun)
			if now == 3000 then COMMANDED_ATTACKS = world[homun].attacks or 0 end
			-- the owner walks away while the ordered fight goes on
			if now > 3000 and now <= 7000 then world[owner].x = world[owner].x + 1 end
		end,
		checks = {
			["it was fighting the ordered monster before the owner left"] = function()
				return (COMMANDED_ATTACKS or 0) > 0
			end,
			["it keeps fighting with the owner far away"] = function()
				local h, o = HarnessMob(2001), HarnessMob(2000001)
				return (h.attacks or 0) > COMMANDED_ATTACKS + 4 and math.abs(o.x - h.x) > 14
					and MyEnemy == 60001
			end,
		} },

	-- The same order on a monster that is already far outside the move bounds. The stand-point
	-- calculation used to veto it ("Cannot attack this target"), so the command was useless: the
	-- owner can be weak and send the homunculus to fight away from him.
	commanded_fight_far = { homuntype = 2, skills = {}, mobs = { {119,100} }, passive = true,
		minAttacks = 1, ticks = 90, set = { CommandedFightStay = 1 },
		afterLoad = function()
			local realAI = AI
			local issued = false
			AI = function(id)
				realAI(id)
				if not issued then
					issued = true
					OnATTACK_OBJECT_CMD(60001)
				end
			end
		end,
		checks = {
			["the ordered monster far outside the bounds is reached and attacked"] = function()
				local h, o = HarnessMob(2001), HarnessMob(2000001)
				return (HarnessMob(60001).hitsTaken or 0) > 0 and MyEnemy == 60001
					and math.abs(o.x - h.x) > 14
			end,
		} },

	-- With CommandedFightStay = 0 the far order is refused, as before.
	commanded_fight_far_giveup = { homuntype = 2, skills = {}, mobs = { {119,100} }, passive = true,
		minAttacks = 0, ticks = 60, set = { CommandedFightStay = 0 },
		afterLoad = function()
			local realAI = AI
			local issued = false
			AI = function(id)
				realAI(id)
				if not issued then
					issued = true
					OnATTACK_OBJECT_CMD(60001)
				end
			end
		end,
		checks = {
			["without the option the far order is dropped"] = function()
				local h = HarnessMob(2001)
				return (HarnessMob(60001).hitsTaken or 0) == 0 and math.abs(h.x - 101) <= 2
			end,
		} },

	-- Same fight with CommandedFightStay = 0: when the owner gets out of the move bounds the
	-- fight is dropped and the homunculus follows the owner again.
	commanded_fight_giveup = { homuntype = 2, skills = {}, mobs = { {103,100} }, passive = true,
		minAttacks = 1, ticks = 140, set = { CommandedFightStay = 0 },
		afterLoad = function()
			local realAI = AI
			local issued = false
			AI = function(id)
				realAI(id)
				if not issued then
					issued = true
					OnATTACK_OBJECT_CMD(60001)
				end
			end
		end,
		onTick = function(world, now, owner, homun)
			if now == 3000 then COMMANDED_ATTACKS = world[homun].attacks or 0 end
			if now == 5000 then COMMANDED_ATTACKS_LATE = world[homun].attacks or 0 end
			if now > 3000 and now <= 7000 then world[owner].x = world[owner].x + 1 end
		end,
		checks = {
			["it fought while the owner was close"] = function() return (COMMANDED_ATTACKS or 0) > 0 end,
			["the fight is dropped and it follows the owner"] = function()
				local h, o = HarnessMob(2001), HarnessMob(2000001)
				return (h.attacks or 0) == COMMANDED_ATTACKS_LATE and math.abs(o.x - h.x) <= 4
			end,
		} },

	-- MoveSticky: the move command pins the homunculus around the cell where it arrived. With
	-- MoveStickyFight = 0 it stays out of the fight even with a monster attacking the owner next
	-- to it, and no idle walk drags it back to the owner.
	move_sticky_hold = { homuntype = 2, skills = {}, mobs = { {112,100} },
		minAttacks = 0, ticks = 100,
		set = { MoveSticky = 1, MoveStickyFight = 0, UseIdleWalk = 4 },
		afterLoad = function()
			local realAI = AI
			local moved = false
			AI = function(id)
				realAI(id)
				if not moved then
					moved = true
					OnMOVE_CMD(105, 100)
				end
				if ReturnToMoveHold == 1 and StickyX == 105 then STICKY_PINNED = true end
				local x, y = GetV(V_POSITION, MyID)
				if STICKY_PINNED and (x ~= 105 or y ~= 100) then STICKY_MOVED = true end
			end
		end,
		checks = {
			["the move command pins the homunculus on the cell"] = function() return STICKY_PINNED == true end,
			["MoveStickyFight = 0 keeps it out of the fight and on the cell"] = function()
				return (HarnessMob(2001).attacks or 0) == 0 and STICKY_MOVED ~= true
					and MyState == IDLE_ST
			end,
		} },

	-- MoveStickyFight = 1: it engages the monster from the pin, walks back to the cell when the
	-- fight ends, and a second move command releases the pin.
	move_sticky_fight = { homuntype = 2, skills = {}, mobs = { {112,100} },
		minAttacks = 1, ticks = 150,
		set = { MoveSticky = 1, MoveStickyFight = 1, UseIdleWalk = 4 },
		afterLoad = function()
			local realAI = AI
			AI = function(id)
				realAI(id)
				local now = GetTick()
				if now == 1000 then
					OnMOVE_CMD(105, 100)
				elseif now == 5000 then
					STICKY_ATTACKS = HarnessMob(2001).attacks or 0
					-- the monster walks out of sight: the fight ends and the pin takes over again
					local m = HarnessMob(60001)
					m.x, m.y = 140, 100
				elseif now == 9000 then
					OnMOVE_CMD(107, 100)
				end
				local x = GetV(V_POSITION, MyID)
				if STICKY_ATTACKS and now > 5000 and now < 9000 and MyState == IDLE_ST
					and math.abs(x - 105) <= 3 then
					STICKY_BACK = true
				end
				if STICKY_ATTACKS and ReturnToMoveHold == 0 and StickyX == 0 then UNPINNED = true end
			end
		end,
		checks = {
			["with MoveStickyFight = 1 it fights from the pin"] = function()
				return (STICKY_ATTACKS or 0) > 0
			end,
			["it walks back to the sticky cell after the fight"] = function()
				return STICKY_BACK == true
			end,
			["a second move command releases the pin"] = function() return UNPINNED == true end,
		} },

	-- FOLLOW_CMD_ST friending (StandbyFriending): while the owner sits, a player one cell to the
	-- right joins MyFriends (FRIEND_CIRCLE_ST) and one to the left leaves it (FRIEND_CROSS_ST).
	-- The command state is set before the first tick, so idle targeting never runs.
	follow_friending = { homuntype = 2, skills = {}, mobs = {}, passive = true,
		minAttacks = 0, ticks = 140, set = { UseIdleWalk = 0, StandbyFriending = 1, FollowStayBack = 2 },
		player = { id = 3003, x = 101, y = 100 },
		afterLoad = function()
			OnFOLLOW_CMD()
			local realAI = AI
			local followAgain = false
			AI = function(id)
				realAI(id)
				if MyFriends[3003] == 1 then FRIEND_ADDED = true end
				if MyState == FRIEND_CIRCLE_ST then FRIEND_CIRCLE = true end
				if FRIEND_ADDED and MyFriends[3003] == nil then FRIEND_REMOVED = true end
				if MyState == FRIEND_CROSS_ST then FRIEND_CROSS = true end
				-- the circle walk ends in IDLE_ST; follow again so the removal can be scanned
				if GetTick() >= 6400 and not followAgain then
					followAgain = true
					OnFOLLOW_CMD()
				end
			end
		end,
		onTick = function(world, now, owner)
			-- the homunculus got out of the way first (OnMOVE_CMD in the first tick)
			if now == 2000 then world[owner].motion = MOTION_SIT end
			if now == 6400 then world[3003].x, world[3003].y = 99, 100 end
		end,
		checks = {
			["a player to the right of the sitting owner is friended"] = function()
				return FRIEND_ADDED == true and FRIEND_CIRCLE == true
			end,
			["a player to the left is removed from the friend list"] = function()
				return FRIEND_REMOVED == true and FRIEND_CROSS == true
			end,
		} },

	-- Spear mercenary: AttackRange() reports 2 for LANCER01..10, so it must attack from two cells
	-- without closing to one, and use its spear skill from there.
	mercenary_lancer = { merc = { mertype = 11 }, skills = { "ML_PIERCE:1" }, mobs = { {105,100} },
		minAttacks = 1, ticks = 60, expect = { "Pierce" },
		checks = {
			["the spear type reports attack range 2"] = function() return AttackRange(MyID, 0, nil) == 2 end,
			["it attacks from two cells"] = function()
				local h, m = HarnessMob(2001), HarnessMob(60001)
				return (m.hitsTaken or 0) > 0 and math.abs(h.x - m.x) == 2
			end,
		} },

	-- Mercenary buffs (the path had no coverage): an ARCHER03 has Weapon Quicken, so with
	-- UseOffensiveBuff = 1 (Idle) it must cast it on itself while idling.
	mercenary_quicken = { merc = { mertype = 3 }, skills = { "MER_QUICKEN:1" }, mobs = {},
		passive = true, minAttacks = 0, ticks = 40,
		set = { UseOffensiveBuff = 1, UseDefensiveBuff = 0 },
		expect = { "Weapon Quicken" } },

	-- Mercenary defensive buff: LANCER05 has Guard (Autoguard); with UseDefensiveBuff = 1 (Idle)
	-- it must cast it on itself.
	mercenary_guard = { merc = { mertype = 15 }, skills = { "ML_AUTOGUARD:3" }, mobs = {},
		passive = true, minAttacks = 0, ticks = 40,
		set = { UseDefensiveBuff = 1, UseOffensiveBuff = 0 },
		expect = { "Guard" } },

	-- Mercenary Provoke on the owner: SWORDMAN02 has it; UseProvokeOwner = 1 (Idle) makes the AI
	-- walk into range (the owner is adjacent) and cast it on the owner, not on itself.
	mercenary_provoke = { merc = { mertype = 22 }, skills = { "MER_PROVOKE:5" }, mobs = {},
		passive = true, minAttacks = 0, ticks = 40,
		set = { UseProvokeOwner = 1, UseOffensiveBuff = 0, UseDefensiveBuff = 0 },
		expect = { "Provoke" },
		checks = {
			["Provoke is cast on the owner"] = function()
				for _, c in ipairs(HarnessCasts) do
					if c.name == "Provoke" and c.ok and c.a == 2000001 then return true end
				end
				return false
			end,
		} },

	-- Mercenary Sacrifice: LANCER03 has it; UseSacrificeOwner = 1 casts it on the owner.
	mercenary_sacrifice = { merc = { mertype = 13 }, skills = { "ML_DEVOTION:1" }, mobs = {},
		passive = true, minAttacks = 0, ticks = 40,
		set = { UseSacrificeOwner = 1, UseOffensiveBuff = 0, UseDefensiveBuff = 0 },
		expect = { "Sacrifice" },
		checks = {
			["Sacrifice is cast on the owner"] = function()
				for _, c in ipairs(HarnessCasts) do
					if c.name == "Sacrifice" and c.ok and c.a == 2000001 then return true end
				end
				return false
			end,
		} },

	-- AutoSkillLimit: a global cap on the offensive skills used on one target (the tactic still
	-- applies on top). The mercenary keeps normal-attacking but stops spending SP on skills.
	mercenary_skill_limit = { merc = { mertype = 21 }, skills = { "MS_BASH:1" }, mobs = single,
		minAttacks = 1, ticks = 80, set = { AutoSkillLimit = 1 },
		checks = {
			["one skill on the target, then normal attacks"] = function()
				local n = 0
				for _, c in ipairs(HarnessCasts) do if c.name == "Bash" then n = n + 1 end end
				return n == 1
			end,
		} },

	mercenary_skill_unlimited = { merc = { mertype = 21 }, skills = { "MS_BASH:1" }, mobs = single,
		minAttacks = 1, ticks = 80, expect = { "Bash" },
		checks = {
			["with the default (0) the skill is repeated"] = function()
				local n = 0
				for _, c in ipairs(HarnessCasts) do if c.name == "Bash" then n = n + 1 end end
				return n > 1
			end,
		} },

	-- Mercenaries run a separate AI state (AI_M.lua, M_Config.lua, M_Tactics.lua) that had no
	-- behavioral coverage at all: at least chase and hit a monster attacking the owner.
	mercenary_attacks = { merc = { mertype = 21 }, skills = { "MS_BASH:1", "MER_DECAGI:1" },
		mobs = single, minAttacks = 1, ticks = 60, expect = { "Bash" },
		checks = {
			["the mercenary type is detected at init"] = function() return MercType == SWORDMAN01 end,
			["the mercenary chases and attacks the owner's attacker"] = function()
				return (HarnessMob(60001).hitsTaken or 0) > 0 and MyEnemy == 60001
			end,
			["it is not mistaken for a homunculus"] = function() return IsHomun(MyID) == 0 end,
		} },
}


-- Benchmark of a Homunculus S against a zero-defense Neutral dummy (or a pack of five), for
-- comparing configurations: tools/harness/homun_bench.py. Everything comes from the environment,
-- so the scenario table stays fixed:
--   HBENCH_SET "AutoMobCount=2,UseEiraTwisterCutter=0"   config overrides
--   HBENCH_MOBS 1 to 5 monsters      HBENCH_SECONDS 180      HBENCH_SPREGEN SP per second      HBENCH_CAST_RATIO
local HOMUN_BENCH = {
	["eira"] = { 48, EIRA_SKILLS, 6334 },
	["bayeri"] = { 49, BAYERI_SKILLS, 9221 },
	["sera"] = { 50, SERA_SKILLS, 9707 },
	["dieter"] = { 51, DIETER_SKILLS, 14814 },
	-- the Eleanor extras are the calibration of eleanor_bench_* above (swing rate, SP regeneration)
	["eleanor"] = { 52, ELEANOR_SKILLS, 3381, spregen = 22, swingMs = 129, castPauseMs = 550, attackMotion = 100,
		set = { SphereTrackInterval = 200 } },
}
for name, h in pairs(HOMUN_BENCH) do
	local set = {}
	for key, value in pairs(h.set or {}) do set[key] = value end
	for key, value in (os.getenv("HBENCH_SET") or ""):gmatch("(%w+)=([%w%.%-]+)") do set[key] = tonumber(value) end
	-- HBENCH_DEF / HBENCH_MDEF: hard DEF and MDEF of the dummies (default 0)
	local over = { def = tonumber(os.getenv("HBENCH_DEF") or ""), mdef = tonumber(os.getenv("HBENCH_MDEF") or "") }
	local benchMobs = {}
	for i, m in ipairs({ unpack(cluster, 1, math.min(math.max(tonumber(os.getenv("HBENCH_MOBS") or "") or 1, 1), #cluster)) }) do
		benchMobs[i] = { m[1], m[2], nil, over }
	end
	SCENARIOS["hbench_" .. name] = { homuntype = h[1], skills = h[2],
		mobs = benchMobs,
		private = true, passive = true, sp = h[3], spregen = tonumber(os.getenv("HBENCH_SPREGEN") or "") or h.spregen or 60,
		swingMs = h.swingMs, castPauseMs = h.castPauseMs, attackMotion = h.attackMotion,
		ticks = (tonumber(os.getenv("HBENCH_SECONDS") or "") or 180) * 10,
		castRatio = tonumber(os.getenv("HBENCH_CAST_RATIO") or "") or 1,
		-- the AI knows the dummy as a Poring (Water): show it the Neutral element the damage model uses,
		-- or fire skills would count the monsters at half weight
		afterLoad = function()
			if MobInfo and MobInfo[1002] then
				local info = {}
				for i, v in ipairs(MobInfo[1002]) do info[i] = v end
				info[2] = 0
				info[7], info[8] = over.def or 0, over.mdef or 0
				MobInfo[1002] = info
			end
		end,
		minAttacks = 0, maxRefused = 1000, set = set }
end
