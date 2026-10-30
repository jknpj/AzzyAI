-- Test scenarios for harness.lua. Each one runs the real AI for ~30 s of game time.
--   homuntype  homunculus S type (EIRA=48, BAYERI=49, SERA=50, DIETER=51, ELEANOR=52)
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

local BAYERI_SKILLS = { "MH_STAHL_HORN", "MH_HEILIGE_STANGE", "MH_GOLDENE_FERSE", "MH_ANGRIFFS_MODUS",
	"MH_STEINWAND", "MH_GLANZEN_SPIES", "MH_HEILIGE_PFERD", "MH_GOLDENE_TONE" }
local DIETER_SKILLS = { "MH_MAGMA_FLOW", "MH_GRANITIC_ARMOR", "MH_LAVA_SLIDE", "MH_PYROCLASTIC",
	"MH_VOLCANIC_ASH", "MH_BLAST_FORGE", "MH_TEMPERING" }
local SERA_SKILLS = { "MH_SUMMON_LEGION", "MH_NEEDLE_OF_PARALYZE", "MH_POISON_MIST", "MH_PAIN_KILLER",
	"MH_TOXIN_OF_MANDARA", "MH_NEEDLE_STINGER" }
local EIRA_SKILLS = { "MH_LIGHT_OF_REGENE", "MH_OVERED_BOOST", "MH_ERASER_CUTTER", "MH_XENO_SLASHER",
	"MH_SILENT_BREEZE", "MH_TWISTER_CUTTER", "MH_ABSOLUTE_ZEPHYR" }
local ELEANOR_SKILLS = { "MH_STYLE_CHANGE", "MH_SONIC_CLAW", "MH_SILVERVEIN_RUSH", "MH_MIDNIGHT_FRENZY",
	"MH_TINDER_BREAKER", "MH_CBC", "MH_EQC", "MH_BLAZING_AND_FURIOUS", "MH_THE_ONE_FIGHTER_RISES" }

-- SkillList/SkillInfo against the rAthena skill trees (db/re/homunculus_db.yml): same max levels,
-- every active skill listed and per-level SkillInfo entries for every level
local function checkSkillTrees()
	assert(RA_HOMUN and RA_SKILL, "fixtures/rathena.lua missing: run python tools/harness/rathena_sync.py")
	local problems = {}
	local function name(id) return (RA_SKILL[id] and RA_SKILL[id].name) or tostring(id) end
	for const, tree in pairs(RA_HOMUN) do
		local list = SkillList[_G[const]] or {}
		for id, lvl in pairs(list) do
			local ra, si = tree[id], SkillInfo[id]
			if not ra then
				problems[#problems + 1] = const .. ": " .. name(id) .. " is not in the rAthena skill tree"
			elseif ra.maxlv ~= lvl then
				problems[#problems + 1] = ("%s: %s max level %d, rAthena %d"):format(const, name(id), lvl, ra.maxlv)
			end
			if not si then
				problems[#problems + 1] = const .. ": " .. name(id) .. " has no SkillInfo"
			else
				for idx = 2, 9 do
					if type(si[idx]) == "table" and #si[idx] < lvl then
						problems[#problems + 1] = ("%s: SkillInfo[%s][%d] has %d levels, needs %d"):format(
							const, name(id), idx, #si[idx], lvl)
					end
				end
			end
		end
		for id in pairs(tree) do
			if not list[id] and RA_SKILL[id] and RA_SKILL[id].target ~= "Passive" then
				problems[#problems + 1] = const .. ": active skill " .. name(id) .. " missing from SkillList"
			end
		end
	end
	table.sort(problems)
	if #problems > 0 then error(table.concat(problems, "; "), 0) end
	return true
end

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

SCENARIOS = {
	-- freshly made homunculus: no skills learned, little SP; it must still fight
	amistr_fresh = { homuntype = 2, skills = {}, mobs = cluster, sp = 20, spregen = 1, minAttacks = 100,
		checks = {
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
	bayeri = { homuntype = 49, skills = BAYERI_SKILLS, mobs = cluster,
		expect = { "Heilige Stange", "Stahl Horn", "Stein Wand", "Goldene Tone", "Angriffs Modus" } },
	-- AoE must not be wasted on a single target (AutoMobCount = 2)
	bayeri_single = { homuntype = 49, skills = BAYERI_SKILLS, mobs = single,
		expect = { "Glanzen Spies", "Stahl Horn" }, forbid = { "Heilige Stange", "Stein Wand" } },
	-- both AoEs on and fully learned: the cheap ranged Stange must not starve the caster-centered Pferd
	bayeri_stange_pferd = { homuntype = 49, skills = BAYERI_SKILLS, mobs = cluster,
		set = { UseBayeriHeiligePferd = 1 },
		expect = { "Heilige Stange", "Heilige Pferd" } },
	bayeri_pferd = { homuntype = 49, skills = BAYERI_SKILLS, mobs = cluster,
		set = { UseBayeriHeiligePferd = 1, UseBayeriHailegeStar = 0 },
		expect = { "Heilige Pferd" }, forbid = { "Heilige Stange" } },
	-- regression: Stein Wand used to be spammed without checking SP (the harness fails any cast without SP)
	bayeri_low_sp = { homuntype = 49, skills = BAYERI_SKILLS, mobs = cluster, sp = 150, spregen = 5,
			minAttacks = 100 },
	-- partly levelled homunculus: hard-coded Lv5 casts (Stein Wand) and configured levels above the
	-- learned one must be lowered before casting. Unlearned Pferd has ambiguous range 1,
	-- so it requires the same eight bounded probes as bayeri_pferd_unlearned below.
	-- Heilige Stange is off like in bayeri_pferd: Lv2 casts in 3 s with no cooldown, so the AI would
	-- chain it from range 9 and never get close enough for the caster-centered Pferd (or melee).
	-- Glanzen Spies is off too, or it spends the SP the Pferd probes need.
	bayeri_partial = { homuntype = 49, mobs = cluster, maxRefused = 8, minAttacks = 100, ticks = 500,
		skills = { "MH_STAHL_HORN:3", "MH_HEILIGE_STANGE:2", "MH_GOLDENE_FERSE", "MH_ANGRIFFS_MODUS",
			"MH_STEINWAND:1", "MH_GLANZEN_SPIES", "MH_GOLDENE_TONE" },
		set = { UseBayeriHeiligePferd = 1, UseBayeriHailegeStar = 0, UseBayeriGlanzenSpies = 0 },
		expect = { "Stahl Horn", "Stein Wand" }, forbid = { "Heilige Pferd", "Heilige Stange" },
		checks = {
			["unlearned Pferd probes stop"] = function() return LearnedSkill[MH_HEILIGE_PFERD].cap == 0 end,
			["learned levels detected"] = function()
				return GetSkillLevelCap(MH_STAHL_HORN) == 3 and GetSkillLevelCap(MH_HEILIGE_STANGE) == 2
					and GetSkillLevelCap(MH_STEINWAND) == 1
			end,
			["Stahl Horn lowered to the learned Lv3"] = function()
				for _, c in ipairs(HarnessCasts) do
					if c.skill == MH_STAHL_HORN and c.lvl > 3 then return false end
				end
				return true
			end,
		} },
	-- the cheap Stange chain above, with Stange on: casts stay at the learned Lv2
	bayeri_partial_stange = { homuntype = 49, mobs = cluster, minAttacks = 0,
		skills = { "MH_STAHL_HORN:3", "MH_HEILIGE_STANGE:2", "MH_STEINWAND:1" },
		expect = { "Heilige Stange" },
		checks = {
			["Heilige Stange stays at the learned Lv2"] = function()
				for _, c in ipairs(HarnessCasts) do
					if c.skill == MH_HEILIGE_STANGE and c.lvl ~= 2 then return false end
				end
				return GetSkillLevelCap(MH_HEILIGE_STANGE) == 2
			end,
		} },
	-- Heilige Pferd not learned but enabled: its Lv1-4 have range 1 so the probe can't tell. The AI
	-- steps down after refusals and remembers it in data/; after a map change (AI reload) it doesn't
	-- try again until LEARNED_SKILL_RECHECK_SECONDS have passed.
	bayeri_pferd_unlearned = { homuntype = 49, mobs = cluster, maxRefused = 8, ticks = 500, reloadAt = 400,
		skills = { "MH_STAHL_HORN", "MH_GOLDENE_FERSE", "MH_ANGRIFFS_MODUS", "MH_STEINWAND", "MH_GOLDENE_TONE" },
		set = { UseBayeriHeiligePferd = 1, UseBayeriHailegeStar = 0 },
		forbid = { "Heilige Pferd" },
		checks = {
			["no refused casts after reload"] = function() return HarnessRefusedSince(1000 + 399 * 100) == 0 end,
			["Pferd known as not learned"] = function() return LearnedSkill[MH_HEILIGE_PFERD].cap == 0 end,
		} },

	-- regression: swapped duration/reuse made Magma Flow recast forever and Dieter never attacked
	dieter = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster, minAttacks = 20,
		expect = { "Lava Slide", "Magma Flow", "Pyroclastic", "Tempering" } },
	dieter_private = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster, private = true, minAttacks = 20,
		expect = { "Lava Slide", "Magma Flow" },
		checks = {
			["private Lava Slide AoE stays 7x7 (rAthena Unit Layout 3)"] = function()
				return GetSkillAoESize(MH_LAVA_SLIDE, 1) == 7 and GetSkillAoESize(MH_LAVA_SLIDE, 10) == 7
			end,
		} },
	-- regression: Blast Forge has range 1, which the old learned-skill check read as "not learned"
	dieter_forge = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster,
		set = { UseDieterBlastForge = 1 }, expect = { "Blast Forge" } },

	sera = { homuntype = 50, skills = SERA_SKILLS, mobs = cluster,
		expect = { "Needle of Paralyze", "Poison Mist", "Pain Killer" } },
	-- regression: Toxin Lv1-4 has range 1, so its learned level can't be probed; the old check
	-- reported Lv4 and the AI spammed refused casts ("Skill failed") forever
	sera_toxin_low = { homuntype = 50, mobs = cluster, maxRefused = 4,
		skills = { "MH_SUMMON_LEGION", "MH_NEEDLE_OF_PARALYZE", "MH_POISON_MIST", "MH_PAIN_KILLER",
			"MH_TOXIN_OF_MANDARA:2", "MH_NEEDLE_STINGER" },
		set = { UseSeraToxinOfMandara = 1, SeraToxinOfMandaraLevel = 10, UseSeraParalyze = 0 },
		expect = { "Toxin Of Mandara" },
		checks = {
			["Toxin settles on the learned level"] = function() return LearnedSkill[MH_TOXIN_OF_MANDARA].cap == 2 end,
		} },
	-- after a map change the saved level is used right away: no refused casts
	sera_toxin_reload = { homuntype = 50, mobs = cluster, maxRefused = 4, reloadAt = 200,
		skills = { "MH_SUMMON_LEGION", "MH_POISON_MIST", "MH_PAIN_KILLER", "MH_TOXIN_OF_MANDARA:2", "MH_NEEDLE_STINGER" },
		set = { UseSeraToxinOfMandara = 1, SeraToxinOfMandaraLevel = 10, UseSeraParalyze = 0 },
		expect = { "Toxin Of Mandara" },
		checks = {
			["no refused casts after reload"] = function() return HarnessRefusedSince(1000 + 199 * 100) == 0 end,
			["Toxin still Lv2"] = function() return LearnedSkill[MH_TOXIN_OF_MANDARA].cap == 2 end,
		} },
	-- the player levelled Toxin between sessions: the periodic level-up try picks it up
	sera_toxin_levelup = { homuntype = 50, mobs = cluster, maxRefused = 5, reloadAt = 200,
		skills = { "MH_SUMMON_LEGION", "MH_POISON_MIST", "MH_PAIN_KILLER", "MH_TOXIN_OF_MANDARA:2", "MH_NEEDLE_STINGER" },
		set = { UseSeraToxinOfMandara = 1, SeraToxinOfMandaraLevel = 10, UseSeraParalyze = 0,
			LEARNED_SKILL_RECHECK_SECONDS = 0 },
		afterReload = function(known) known[MH_TOXIN_OF_MANDARA] = 3 end,
		expect = { "Toxin Of Mandara" },
		checks = {
			["Toxin raised to Lv3"] = function() return LearnedSkill[MH_TOXIN_OF_MANDARA].cap == 3 end,
		} },
	sera_s = { homuntype = 50, skills = SERA_SKILLS, mobs = cluster,
		set = { UseSeraToxinOfMandara = 1, UseSeraNeedleStinger = 1, UseSeraParalyze = 0 },
		expect = { "Toxin Of Mandara", "Needle Stinger" },
		checks = {
			["official Toxin is caster-centered"] = function()
				return GetSkillInfo(MH_TOXIN_OF_MANDARA, 7) == 0 and SkillAOEInfo[MH_TOXIN_OF_MANDARA][2] == 1
			end,
			["official Sonic Claw reuse is 1000 ms, range 1"] = function()
				return GetSkillInfo(MH_SONIC_CLAW, 9, 5) == 1000 and GetSkillInfo(MH_SONIC_CLAW, 2, 5) == 1
					and SkillSphereCost[MH_CBC] == 2
			end,
			["official Lava Slide AoE is 7x7"] = function() return GetSkillAoESize(MH_LAVA_SLIDE, 1) == 7 end,
		} },
	sera_s_private = { homuntype = 50, skills = SERA_SKILLS, mobs = cluster, private = true,
		set = { UseSeraToxinOfMandara = 1, UseSeraNeedleStinger = 1, UseSeraParalyze = 0 },
		expect = { "Toxin Of Mandara", "Needle Stinger" },
		checks = {
			["private Toxin is target-centered, range 2"] = function()
				return GetSkillInfo(MH_TOXIN_OF_MANDARA, 7) == 1 and SkillAOEInfo[MH_TOXIN_OF_MANDARA][2] == 0
					and GetSkillInfo(MH_TOXIN_OF_MANDARA, 2, 10) == 2
			end,
			["private Sonic Claw range 3, CBC 1 sphere"] = function()
				return GetSkillInfo(MH_SONIC_CLAW, 2, 5) == 3 and SkillSphereCost[MH_CBC] == 1
			end,
			["private summon ID range"] = function() return MagicNumber == 110000000 end,
		} },

	eira = { homuntype = 48, skills = EIRA_SKILLS, mobs = cluster,
		expect = { "Eraser Cutter", "Xeno Slasher" } },
	eira_s = { homuntype = 48, skills = EIRA_SKILLS, mobs = cluster,
		set = { UseEiraTwisterCutter = 1, UseEiraAbsoluteZephyr = 1 },
		expect = { "Absolute Zephyr" } },

	eleanor = { homuntype = 52, skills = ELEANOR_SKILLS, mobs = single,
		expect = { "Sonic Claw", "Blazing And Furious", "The One Fighter Rises" },
		checks = {
			["grapple skills obey their individual config switches"] = function()
				local previous = {
					mode = EleanorMode,
					spheres = MySpheres,
					comboSCTimeout = ComboSCTimeout,
					comboSVTimeout = ComboSVTimeout,
					cbc = UseEleanorCBC,
					eqc = UseEleanorEQC,
					tinderBreaker = UseEleanorTinderBreaker,
				}
				EleanorMode = GRAPPLING_MODE
				MySpheres = AutoComboSpheres
				ComboSCTimeout = GetTick() + 1000
				ComboSVTimeout = GetTick() + 1000

				UseEleanorCBC = 0
				UseEleanorEQC = 1
				UseEleanorTinderBreaker = 0
				local eqc = GetGrappleSkill(MyID)

				UseEleanorCBC = 1
				UseEleanorEQC = 0
				local cbc = GetGrappleSkill(MyID)

				UseEleanorCBC = 0
				UseEleanorEQC = 0
				UseEleanorTinderBreaker = 1
				local tinderBreaker = GetGrappleSkill(MyID)

				UseEleanorTinderBreaker = 0
				local disabled = GetGrappleSkill(MyID)

				EleanorMode = previous.mode
				MySpheres = previous.spheres
				ComboSCTimeout = previous.comboSCTimeout
				ComboSVTimeout = previous.comboSVTimeout
				UseEleanorCBC = previous.cbc
				UseEleanorEQC = previous.eqc
				UseEleanorTinderBreaker = previous.tinderBreaker

				return eqc == MH_EQC and cbc == MH_CBC and tinderBreaker == MH_TINDER_BREAKER and disabled == 0
			end,
		} },

	-- many monsters on screen: must keep fighting and not error out
	crowd_bayeri = { homuntype = 49, skills = BAYERI_SKILLS, mobs = crowd, expect = { "Heilige Stange" } },
	crowd_sera = { homuntype = 50, skills = SERA_SKILLS, mobs = crowd, expect = { "Poison Mist" } },
	-- GetBestAoECoord / GetBestAoETarget (optimized) must pick exactly what the old brute-force
	-- search picks, on 40 monsters, for ground and target-centered AoEs of several sizes
	crowd_dieter = { homuntype = 51, skills = DIETER_SKILLS, mobs = crowd, minAttacks = 1,
		checks = {
			["optimized AoE aiming matches the brute-force search"] = function()
				ActorPosCache = {} -- the world moved since the last AI() call
				local function bruteCoord(skill, level)
					local range = GetSkillInfo(skill, 2, level)
					local aoe = ((GetSkillAoESize(skill, level) or 1) - 1) / 2
					local mx, my = GetV(V_POSITION, MyID)
					local ox, oy = GetV(V_POSITION, OwnerID)
					local best, bx, by = 0, -1, -1
					for xx = mx - range, mx + range do
						for yy = my - range, my + range do
							local n = 0
							for k in pairs(Targets) do
								local x, y = GetV(V_POSITION, k)
								if GetTact(TACT_BASIC, k) ~= 0 and x < ox + 20 and x > ox - 20 and y < oy + 20 and y > oy - 20
									and math.abs(x - xx) <= aoe and math.abs(y - yy) <= aoe then
									n = n + AoEMobWeight(skill, k)
								end
							end
							if n > best + 1e-6 then best, bx, by = n, xx, yy end
						end
					end
					return bx, by
				end
				local function bruteTarget(skill, level)
					local best, res = 0, -1
					for k in pairs(Targets) do
						if GetTact(TACT_BASIC, k) ~= 0 and IsInAttackSight(MyID, k, skill, level) == true then
							local n = GetMobCount(skill, level, k, 1)
							if best < n then best, res = n, k end
						end
					end
					return res
				end
				local n = 0
				for _ in pairs(Targets) do n = n + 1 end
				if n < 10 then error("only " .. n .. " targets", 0) end
				for _, s in ipairs({ { MH_LAVA_SLIDE, 10 }, { MH_VOLCANIC_ASH, 5 }, { MH_XENO_SLASHER, 10 }, { MH_XENO_SLASHER, 1 } }) do
					local ax, ay = GetBestAoECoord(MyID, s[1], s[2])
					local bx, by = bruteCoord(s[1], s[2])
					if ax ~= bx or ay ~= by then
						error(("coord %d Lv%d: %s,%s vs %s,%s"):format(s[1], s[2], ax, ay, bx, by), 0)
					end
				end
				for _, s in ipairs({ { MH_HEILIGE_STANGE, 10 }, { MH_HEILIGE_STANGE, 1 }, { MH_XENO_SLASHER, 10 } }) do
					local a, b = GetBestAoETarget(MyID, s[1], s[2]), bruteTarget(s[1], s[2])
					if a ~= b then error(("target %d Lv%d: %s vs %s"):format(s[1], s[2], a, b), 0) end
				end
				return true
			end,
		} },

	-- rAthena data (fixtures/rathena.lua, regenerated by rathena_sync.py) --------------------
	rathena_skill_tree = { homuntype = 2, skills = {}, mobs = single, sp = 20, spregen = 1,
		checks = { ["SkillList/SkillInfo match the rAthena skill trees"] = checkSkillTrees } },
	-- Eleanor on rAthena: Sonic Claw needs a sphere, which only normal attacks in fighting style give
	eleanor_private = { homuntype = 52, skills = ELEANOR_SKILLS, mobs = single, private = true,
		expect = { "Sonic Claw" } },
	-- real monsters: plants (Ai 06) next to an aggressive Zombie
	plants_bayeri = { homuntype = 49, skills = BAYERI_SKILLS, mobs = { {103,100,1078}, {103,101,1080}, {104,100,1015} },
		checks = plantChecks() },
	plants_bayeri_private = { homuntype = 49, skills = BAYERI_SKILLS, private = true,
		mobs = { {103,100,1078}, {103,101,1080}, {104,100,1015} }, checks = plantChecks() },
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
			["element table: fire on water 1 = 90%, fire on fire 2 = 0%"] = function()
				return ElementFix[1][3][1] == 90 and ElementFix[2][3][3] == 0
			end,
		} },
	-- the default limit (50%) still lets fire hit a Water 1 Poring (90%)...
	dieter_element_ok = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster, minAttacks = 20,
		expect = { "Lava Slide" } },
	-- ...a stricter limit makes Dieter leave its fire attacks out against it
	dieter_element_skip = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster, minAttacks = 20,
		set = { SkillElementMinRate = 95 },
		forbid = { "Lava Slide" },
		checks = {
			["Lava Slide does 90% to a Poring"] = function()
				local m = HarnessMonsters()[1]
				return SkillElementRate(MH_LAVA_SLIDE, m.id) == 90 and not SkillElementOK(MH_LAVA_SLIDE, m.id)
			end,
		} },
	-- UseMobInfo = 0 brings back the old behavior
	dieter_element_off = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster, minAttacks = 20,
		set = { SkillElementMinRate = 95, UseMobInfo = 0 }, expect = { "Lava Slide" } },
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
	plants_no_tactics = { homuntype = 49, skills = BAYERI_SKILLS, dropTactics = { 1078, 1080 },
		mobs = { {103,100,1078}, {103,101,1080}, {104,100,1015} }, checks = plantChecks() },
	-- Mixed elements: a Fire 2 monster (immune to fire) in front of three Earth 2 ones (175% from fire).
	-- Dieter must still use Lava Slide as an attack (the pack makes it worth it) and lay it so
	-- the Earth pack is inside.
	mixed_element_dieter = { homuntype = 51, skills = DIETER_SKILLS, minAttacks = 1, ticks = 400,
		set = { LavaSlideMode = 0 },
		mobs = { {102,100,1673}, {104,102,1672}, {105,102,1678}, {104,103,1672} },
		checks = {
			["Lava Slide was cast covering the Earth pack"] = function()
				local n = 0
				for _, c in ipairs(HarnessCasts) do
					if c.ok and c.skill == MH_LAVA_SLIDE then
						n = n + 1
						for _, m in ipairs(HarnessMonsters()) do
							if m.class ~= 1673 and math.max(math.abs(c.a - m.x), math.abs(c.b - m.y)) > 3 then
								return false
							end
						end
					end
				end
				return n > 0
			end,
			["the weighted count ignores the immune Dimik"] = function()
				local fire, earth
				for _, m in ipairs(HarnessMonsters()) do
					if m.class == 1673 then fire = m.id elseif not earth then earth = m.id end
				end
				return AoEMobWeight(MH_LAVA_SLIDE, fire) == 0 and AoEMobWeight(MH_LAVA_SLIDE, earth) == 1.75
			end,
		} },
	-- only Fire monsters: no fire skill at all
	fire_immune_dieter = { homuntype = 51, skills = DIETER_SKILLS, minAttacks = 20,
		mobs = { {102,100,1673}, {103,101,1673}, {103,99,1673} }, forbid = { "Lava Slide", "Blast Forge" },
		set = { UseDieterBlastForge = 1, LavaSlideMode = 0 } },
	-- TargetPriority: SelectEnemy on a fixed set of equal-tactics monsters (plant, Fire 2 immune to
	-- Lava Slide, an isolated Earth 2 monster and a pack of three), with TargetPriority 0 and 1
	target_priority = { homuntype = 51, skills = DIETER_SKILLS, minAttacks = 0, dropTactics = { 1078 },
		set = { LavaSlideMode = 0 },
		mobs = { {101,101,1078}, {103,100,1673}, {104,102,1672}, {105,102,1672}, {104,103,1672}, {99,100,1672} },
		checks = (function()
			local spots = { {101,101}, {103,100}, {104,102}, {105,102}, {104,103}, {99,100} }
			-- picks[name] = id the AI chooses among the given monsters (indexes into spots) with TargetPriority tp
			local function pick(tp, indexes, aggroIndex)
				TargetPriority = tp
				AutoSkillCooldown[MH_LAVA_SLIDE] = 0
				TactRowCache, MobInfoCache, ActorClassCache, ActorPosCache = {}, {}, {}, {}
				local ids = {}
				for _, m in ipairs(HarnessMonsters()) do ids[#ids + 1] = m.id end
				table.sort(ids)
				Targets = {}
				for i, id in ipairs(ids) do
					local m = HarnessMob(id)
					m.dead, m.x, m.y = nil, spots[i][1], spots[i][2]
					Targets[id] = { 3, 0 }
				end
				local home = HarnessMob(MyID)
				home.x, home.y = 101, 100
				local enemys = {}
				for _, i in ipairs(indexes) do
					local attacking = i == aggroIndex
					enemys[ids[i]] = { 3, attacking and 1 or 0, TACT_ATTACK_L, 0 }
				end
				Unreachable = {}
				return SelectEnemy(enemys), ids, enemys
			end
			local function expectPick(tp, indexes, wantIndex, aggroIndex)
				local got, ids, enemys = pick(tp, indexes, aggroIndex)
				if got ~= ids[wantIndex] then
					local ctx = TargetPriorityContext()
					local keys = {}
					for k in pairs(enemys) do keys[#keys + 1] = k end
					table.sort(keys)
					local w = ctx.aoe and AoEClusterWeights(ctx.aoe, keys) or {}
					local parts = {}
					for _, k in ipairs(keys) do
						parts[#parts + 1] = ("%d:cluster=%s,immune=%s"):format(k, tostring(w[k]), tostring(TargetImmuneToSkills(ctx, k)))
					end
					error(("TargetPriority=%d picked monster %s, wanted #%d (%s); aoe=%s skills=%d %s"):format(
						tp, tostring(got), wantIndex, tostring(ids[wantIndex]), ctx.aoe and ctx.aoe[1] or "none",
						#ctx.skills, table.concat(parts, " ")), 0)
				end
				return true
			end
			return {
				["off: the closest wins, plant included"] = function() return expectPick(0, { 1, 2, 3 }, 1) end,
				["on: the plant goes last"] = function() return expectPick(1, { 1, 2, 3 }, 3) end,
				["on: a monster immune to the skills goes after one that is not"] = function() return expectPick(1, { 2, 3 }, 3) end,
				["off: the immune monster is closer so it wins"] = function() return expectPick(0, { 2, 3 }, 2) end,
				["on: the one attacking us goes before a closer one"] = function() return expectPick(1, { 3, 4 }, 4, 4) end,
				["off: the closer one wins over the attacker"] = function() return expectPick(0, { 3, 4 }, 3, 4) end,
				["on: the pack is preferred to an isolated monster that is closer"] = function() return expectPick(1, { 6, 3 }, 3) end,
				["off: the isolated monster is closer so it wins"] = function() return expectPick(0, { 6, 3 }, 6) end,
			}
		end)() },
}
