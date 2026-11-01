-- damage.lua: expected damage of a homunculus against a monster, following rAthena Renewal.
--
-- Everything comes from the pinned rAthena commit (RATHENA_COMMIT in fixtures/rathena.lua):
--   status_calc_homunculus_ / status_calc_misc / status_base_atk / status_base_matk_*  (status.cpp)
--   battle_calc_weapon_attack, battle_calc_magic_attack, battle_calc_defense_reduction,
--   battle_calc_element_damage, battle_apply_div_fix, is_attack_hitting                 (battle.cpp)
--   SkillImpl::calculateSkillRatio / modifyDamageData of each skill                      (skills/homunculus/*.cpp)
-- The skill ratios, skill types, elements, hit counts and monster stats are generated into
-- fixtures/rathena.lua (RA_SKILL, RA_RATIO, RA_MOB) by rathena_sync.py.
--
-- It is an expected-value model, not a random simulation: ATK and MATK use their mean, hits
-- always land except for the hit/flee chance, and there are no critical hits (enable_critical
-- does not include homunculi). Not modelled: status effects (bleeding, stun, Tinder Breaker's
-- DEF reduction ...), buffs, the owner's skills and monster-side effects.
Damage = {}

local ELEMENT = { Neutral = 0, Water = 1, Earth = 2, Fire = 3, Wind = 4, Poison = 5, Holy = 6, Dark = 7, Ghost = 8, Undead = 9 }
local MIN_HITRATE, MAX_HITRATE = 5, 100 -- battle.conf defaults
local USHRT_MAX = 65535

-- C integer division: truncates toward zero
local function idiv(a, b)
	local q = a / b
	if q >= 0 then return math.floor(q) end
	return -math.floor(-q)
end
Damage.idiv = idiv

-- mean of rAthena's `rnd() % (hi - lo) + lo`, which is uniform over [lo, hi - 1]
local function meanRange(lo, hi)
	if hi > lo then return lo + (hi - lo - 1) / 2 end
	return lo
end

-- battle.conf hom_setting & HOMSET_SAME_MATK (0x20) makes Min-MATK equal Max-MATK; the
-- default homunc.conf value 0x3D has it set
function Damage.homunStatus(p, opts)
	opts = opts or {}
	local lv = p.level
	local weight = idiv(lv, 10) -- APPLY_HOMUN_LEVEL_STATWEIGHT: every stat counts level / 10 more
	local hstr, hdex, hluk, hint = p.str + weight, p.dex + weight, p.luk + weight, p.int + weight
	local s = {
		level = lv,
		-- sstatus->str ...: what the skill ratios read (without the level weight)
		stats = { str = p.str, agi = p.agi, vit = p.vit, int = p.int, dex = p.dex, luk = p.luk },
		-- status_base_atk (BL_HOM, Renewal), status_base_atk_min/max, status_base_matk_min/max
		batk = math.min(2 * lv + hstr, USHRT_MAX),
		atkMin = idiv(hstr + hdex, 5),
		atkMax = idiv(hluk + hstr + hdex, 3),
		matkMin = hint + lv + idiv(hint + hdex, 5),
		matkMax = hint + lv + idiv(hluk + hint + hdex, 3),
		hit = lv + p.dex + 150, -- status_calc_misc, BL_HOM
	}
	if opts.sameMatk ~= false then s.matkMin = math.max(s.matkMin, s.matkMax) end
	return s
end

-- A monster's defensive status (status_calc_misc for BL_MOB). `over` replaces fields, like
-- setunitdata does on the test server (def, mdef, flee, element, elv, size ...).
function Damage.mobStatus(m, over)
	local t = {}
	for k, v in pairs(m or {}) do t[k] = v end
	for k, v in pairs(over or {}) do t[k] = v end
	t.level = t.level or 1
	t.def = t.def or 0
	t.mdef = t.mdef or 0
	t.res = t.res or 0
	t.mres = t.mres or 0
	t.def2 = t.def2 or idiv(t.level + (t.vit or 1), 2)
	t.mdef2 = t.mdef2 or idiv((t.int or 1) + t.level, 4)
	t.flee = t.flee or (t.level + (t.agi or 1) + 100)
	t.element = t.element or "Neutral"
	t.elv = t.elv or 1
	return t
end

-- battle_attr_fix: percentage the defender's element lets through
local function elementRate(atkElement, mob)
	local byLevel = ElementFix and ElementFix[mob.elv]
	local row = byLevel and byLevel[ELEMENT[atkElement] or 0]
	local rate = row and row[ELEMENT[mob.element] or 0]
	return rate or 100
end

-- is_attack_hitting (Renewal: hitrate = hit - flee, capped)
local function hitChance(hs, mob)
	local rate = math.max(MIN_HITRATE, math.min(MAX_HITRATE, hs.hit - mob.flee))
	return rate / 100
end

-- RES / MRES: damage -= res / (res + 400) * 80% * damage
local function resistance(dmg, res)
	if res <= 0 or dmg == 0 then return dmg end
	return dmg - math.floor(res / (res + 400) * 80 / 100 * dmg)
end

-- skill ratio in % (battle_calc_attack_skill_ratio / battle_calc_magic_attack): 100 plus the
-- skill's calculateSkillRatio addition; skills without one (Tinder Breaker, CBC, EQC) stay at 100
local function skillRatio(hs, skill, lvl)
	local add = RA_RATIO and RA_RATIO[skill]
	return 100 + (add and add(lvl, hs.level, hs.stats) or 0)
end

-- hits per use: HitCount (a negative count only splits the total into that many displayed hits);
-- Sonic Claw and Blazing And Furious set it to the spheres the homunculus has
local SPHERE_HITS = { MH_SONIC_CRAW = true, MH_BLAZING_AND_FURIOUS = true }
local function hitCount(ra, skill, lvl, ctx)
	if SPHERE_HITS[ra.name] then return math.max(ctx and ctx.spheres or 1, 1) end
	return ra.hits[math.min(lvl, #ra.hits)] or 1
end

local function applyHits(dmg, n)
	if n < 0 then n = -n; return idiv(dmg, n) * n end
	return dmg * n
end

-- why Damage.skill can't price this skill, or nil
-- Ground units that hit their targets every Interval (skill_unit_onplace_timer); the other units
-- (Poison Mist, Safety Wall, Volcanic Ash) only apply a status, e.g. skill.cpp UNT_POISON_MIST
local DAMAGE_UNITS = { Lava_Slide = true, Dummyskill = true }
function Damage.unmodelled(skill)
	local ra = RA_SKILL and RA_SKILL[skill]
	if not ra then return "no rAthena data" end
	-- Blazing And Furious is flagged NoDamage but attacks from castendNoDamageId: a calculateSkillRatio
	-- means the skill deals damage
	if ra.nodamage and not (RA_RATIO and RA_RATIO[skill]) then return "deals no damage" end
	if ra.unit and ra.unitid and not DAMAGE_UNITS[ra.unitid] then return "its unit only applies a status" end
	if ra.type ~= "Weapon" and ra.type ~= "Magic" then return "type " .. tostring(ra.type) end
	return nil
end

-- number of times a ground unit hits (Lava Slide, Blast Forge, Poison Mist): Duration1 / Interval,
-- assuming the target stays inside the unit; 1 for everything else
function Damage.ticks(skill, lvl)
	local ra = RA_SKILL and RA_SKILL[skill]
	if not ra or not ra.unit or ra.unitid == "Dummyskill" or ra.interval <= 0 then return 1 end
	return math.max(1, idiv(ra.duration[math.min(lvl, #ra.duration)] or 0, ra.interval))
end

-- flat damage battle_calc_weapon_attack adds after DEF and element (battle.cpp, the switch after
-- battle_calc_element_damage): Tinder Breaker, CBC and EQC are otherwise a normal attack
local FLAT_AFTER_ELEMENT = {
	MH_TINDER_BREAKER = 2500, MH_CBC = 4000, MH_EQC = 6000,
}

-- Expected damage of one use of `skill` on one monster (all hits, one unit tick).
-- ctx.targets = monsters hit together (splash damage that is split among them),
-- ctx.spheres = spirit spheres at the moment of the hit (Eleanor)
function Damage.skill(hs, skill, lvl, mob, ctx)
	local why = Damage.unmodelled(skill)
	if why then return nil, why end
	local ra = RA_SKILL[skill]
	local ratio = skillRatio(hs, skill, lvl)
	local element = ra.element and ra.element[math.min(lvl, #ra.element)] or "Neutral"
	if element == "Weapon" then element = "Neutral" end
	local targets = math.max(ctx and ctx.targets or 1, 1)
	local dmg, hit

	if ra.type == "Magic" then
		-- battle_calc_magic_attack: MATK, split, skill ratio, MRES, MDEF, minimum 1, element
		dmg = meanRange(hs.matkMin, hs.matkMax)
		if ra.splitsplash then dmg = idiv(dmg, targets) end
		dmg = idiv(math.floor(dmg) * ratio, 100)
		dmg = resistance(dmg, mob.mres)
		if not ra.ignoredef then
			dmg = idiv(dmg * (1000 + mob.mdef), 1000 + mob.mdef * 10) - mob.mdef2
		end
		hit = 1 -- magic always hits
	else
		-- battle_calc_weapon_attack: base ATK + batk, split, skill ratio, RES, DEF, minimum 1, element
		dmg = meanRange(hs.atkMin, hs.atkMax) + hs.batk
		if ra.splitsplash then dmg = idiv(math.floor(dmg), targets) end
		dmg = idiv(math.floor(dmg) * ratio, 100)
		dmg = resistance(dmg, mob.res)
		if not ra.ignoredef then
			dmg = idiv(dmg * (4000 + mob.def), 4000 + 10 * mob.def) - mob.def2
		end
		hit = hitChance(hs, mob)
	end
	if dmg < 1 then dmg = 1 end
	dmg = idiv(dmg * elementRate(element, mob), 100)
	local flat = FLAT_AFTER_ELEMENT[ra.name]
	if flat then dmg = dmg + flat * lvl + hs.level end
	dmg = applyHits(dmg, hitCount(ra, skill, lvl, ctx))
	return dmg * hit
end

-- Expected damage of one normal attack: battle_calc_weapon_attack with skill 0 (ratio 100%,
-- neutral weapon element)
function Damage.attack(hs, mob)
	local dmg = meanRange(hs.atkMin, hs.atkMax) + hs.batk
	dmg = resistance(math.floor(dmg), mob.res)
	dmg = idiv(dmg * (4000 + mob.def), 4000 + 10 * mob.def) - mob.def2
	if dmg < 1 then dmg = 1 end
	dmg = idiv(dmg * elementRate("Neutral", mob), 100)
	return dmg * hitChance(hs, mob)
end
