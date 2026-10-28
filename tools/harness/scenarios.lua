-- Test scenarios for harness.lua. Each one runs the real AI for ~30 s of game time.
--   homuntype  homunculus S type (EIRA=48, BAYERI=49, SERA=50, DIETER=51, ELEANOR=52)
--   skills     skills the homunculus has learned
--   mobs       monster positions (owner stands at 100,100, all monsters attack the owner)
--   private    true to run with ServerType = SERVER_PRIVATE
--   set        config overrides applied on top of fixtures/H_Config.lua
--   expect     skills that must be cast at least once
--   forbid     skills that must never be cast
--   minAttacks minimum normal attacks (default 1)
--   checks     named functions that must return true after the run

local cluster = { {103,100}, {104,101}, {104,99}, {103,102}, {105,100} }
local single = { {103,100} }

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

SCENARIOS = {
	bayeri = { homuntype = 49, skills = BAYERI_SKILLS, mobs = cluster,
		expect = { "Heilige Stange", "Stahl Horn", "Stein Wand", "Goldene Tone", "Angriffs Modus" } },
	-- AoE must not be wasted on a single target (AutoMobCount = 2)
	bayeri_single = { homuntype = 49, skills = BAYERI_SKILLS, mobs = single,
		expect = { "Glanzen Spies", "Stahl Horn" }, forbid = { "Heilige Stange", "Stein Wand" } },
	bayeri_pferd = { homuntype = 49, skills = BAYERI_SKILLS, mobs = cluster,
		set = { UseBayeriHeiligePferd = 1, UseBayeriHailegeStar = 0 },
		expect = { "Heilige Pferd" }, forbid = { "Heilige Stange" } },
	-- regression: Stein Wand used to be spammed without checking SP (the harness fails any cast without SP)
	bayeri_low_sp = { homuntype = 49, skills = BAYERI_SKILLS, mobs = cluster, sp = 150, spregen = 5,
			minAttacks = 100 },

	-- regression: swapped duration/reuse made Magma Flow recast forever and Dieter never attacked
	dieter = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster, minAttacks = 20,
		expect = { "Lava Slide", "Magma Flow", "Pyroclastic", "Tempering" } },
	dieter_private = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster, private = true, minAttacks = 20,
		expect = { "Lava Slide", "Magma Flow" },
		checks = {
			["private Lava Slide AoE grows with level"] = function()
				return GetSkillAoESize(MH_LAVA_SLIDE, 1) == 3 and GetSkillAoESize(MH_LAVA_SLIDE, 10) == 9
			end,
		} },
	-- regression: Blast Forge has range 1, which the old learned-skill check read as "not learned"
	dieter_forge = { homuntype = 51, skills = DIETER_SKILLS, mobs = cluster,
		set = { UseDieterBlastForge = 1 }, expect = { "Blast Forge" } },

	sera = { homuntype = 50, skills = SERA_SKILLS, mobs = cluster,
		expect = { "Needle of Paralyze", "Poison Mist", "Pain Killer" } },
	sera_s = { homuntype = 50, skills = SERA_SKILLS, mobs = cluster,
		set = { UseSeraToxinOfMandara = 1, UseSeraNeedleStinger = 1, UseSeraParalyze = 0 },
		expect = { "Toxin Of Mandara", "Needle Stinger" },
		checks = {
			["official Toxin is caster-centered"] = function()
				return GetSkillInfo(MH_TOXIN_OF_MANDARA, 7) == 0 and SkillAOEInfo[MH_TOXIN_OF_MANDARA][2] == 1
			end,
			["official Sonic Claw reuse is 500 ms"] = function() return GetSkillInfo(MH_SONIC_CLAW, 9, 5) == 500 end,
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
			["private Sonic Claw reuse is 1000 ms"] = function() return GetSkillInfo(MH_SONIC_CLAW, 9, 5) == 1000 end,
			["private summon ID range"] = function() return MagicNumber == 110000000 end,
		} },

	eira = { homuntype = 48, skills = EIRA_SKILLS, mobs = cluster,
		expect = { "Eraser Cutter", "Xeno Slasher" } },
	eira_s = { homuntype = 48, skills = EIRA_SKILLS, mobs = cluster,
		set = { UseEiraTwisterCutter = 1, UseEiraAbsoluteZephyr = 1 },
		expect = { "Absolute Zephyr" } },

	eleanor = { homuntype = 52, skills = ELEANOR_SKILLS, mobs = single,
		expect = { "Sonic Claw", "Blazing And Furious", "The One Fighter Rises" } },
}
