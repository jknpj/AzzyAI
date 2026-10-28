-- Offline test harness for AzzyAI.
-- Simulates the Ragnarok Online client AI API (GetV, GetActors, SkillObject, ...) so the
-- real AI code can run without a game client, then checks the casts against expectations.
--
-- Usage (normally through run.py): lua5.1 harness.lua <scenario> [ticks]
-- The current directory must contain AI/USER_AI, because the AI loads its files by relative path.

local HERE = (arg[0]:match("^(.*)[/\\]") or ".") .. "/"
local scenario = arg[1] or "bayeri"
local TICKS = tonumber(arg[2] or "300")
local STEP = 100 -- ms between AI() calls, like the real client

local now = 1000
local OWNER, HOMUN = 2000001, 2001
local world = {}
local casts = {}
local traces = {}

local function ent(id, t) world[id] = t; t.id = id; return t end

local function rangeOf(skill, lvl)
	local si = SkillInfo and SkillInfo[skill]
	if not si then return 1 end
	local r = si[2]
	if type(r) == "table" then return r[lvl or #r] or r[#r] end
	return r or 1
end

local function skillField(skill, idx, lvl)
	local si = SkillInfo and SkillInfo[skill]
	if not si then return 0 end
	local c = si[idx]
	if type(c) == "table" then return c[lvl] or c[#c] or 0 end
	return c or 0
end

-- Client API mocks ----------------------------------------------------------

function GetTick() return now end
function TraceAI(s) traces[#traces + 1] = now .. " " .. tostring(s) end
function GetMsg(id) return { 0 } end
function GetResMsg(id) return { 0 } end
function GetActors()
	local r = {}
	for id, e in pairs(world) do if not e.dead then r[#r + 1] = id end end
	table.sort(r)
	return r
end
function IsMonster(id) local e = world[id]; return (e and e.monster) and 1 or 0 end
function GetDistance(x1, y1, x2, y2) return math.floor(math.sqrt((x1 - x2) ^ 2 + (y1 - y2) ^ 2)) end
function IsInAttackSight(a, b, skill, lvl)
	local A, B = world[a], world[b]
	if not A or not B then return false end
	local range = A.atkrange or 1
	if skill and skill ~= 0 then range = math.max(rangeOf(skill, lvl), 1) end
	return math.max(math.abs(A.x - B.x), math.abs(A.y - B.y)) <= range
end

local skillKnown = {}
function GetV(v, id, extra, extra2)
	local e = world[id]
	if v == V_OWNER then return OWNER end
	if not e then
		if v == V_POSITION then return -1, -1 end
		return 0
	end
	if v == V_POSITION then return e.x, e.y end
	if v == V_TYPE then return e.class or 0 end
	if v == V_MOTION then return e.dead and MOTION_DEAD or (e.motion or MOTION_STAND) end
	if v == V_ATTACKRANGE then return e.atkrange or 1 end
	if v == V_TARGET then return e.target or 0 end
	-- like the client: unknown skills report range 1
	if v == V_SKILLATTACKRANGE then
		if skillKnown[extra] then return math.max(rangeOf(extra), 2) end
		return 1
	end
	if v == V_SKILLATTACKRANGE_LEVEL then
		if skillKnown[extra] then return rangeOf(extra, extra2) end
		return 1
	end
	if v == V_HOMUNTYPE then return e.homuntype or e.class end
	if v == V_HP then return e.hp end
	if v == V_SP then return e.sp end
	if v == V_MAXHP then return e.maxhp end
	if v == V_MAXSP then return e.maxsp end
	if v == V_MERTYPE then return e.mertype end
	return 0
end

function Move(id, x, y) local e = world[id]; if e then e.destx, e.desty = x, y; e.motion = MOTION_MOVE end end
function MoveToOwner(id) local o = world[OWNER]; Move(id, o.x, o.y) end
function Attack(id, target)
	local e = world[id]
	if e then e.motion = MOTION_ATTACK; e.attacks = (e.attacks or 0) + 1 end
end

local function recordCast(kind, id, lvl, skill, a, b)
	local e = world[id]
	local name = (SkillInfo and SkillInfo[skill] and SkillInfo[skill][1]) or tostring(skill)
	local ok = e.sp >= skillField(skill, 3, lvl)
	if ok then
		e.sp = e.sp - skillField(skill, 3, lvl)
		e.motion = MOTION_SKILL
		-- the client stops calling AI() while casting and during after-cast delay
		e.busyUntil = now + skillField(skill, 4, lvl) + skillField(skill, 5, lvl) + skillField(skill, 6, lvl)
	end
	casts[#casts + 1] = { t = now, kind = kind, skill = skill, name = name, lvl = lvl, a = a, b = b, ok = ok, sp = e.sp }
end
function SkillObject(id, lvl, skill, target) recordCast("obj", id, lvl, skill, target) end
function SkillGround(id, lvl, skill, x, y) recordCast("gnd", id, lvl, skill, x, y) end

-- Scenario setup ------------------------------------------------------------

dofile(HERE .. "scenarios.lua")
local sc = SCENARIOS[scenario]
if not sc then
	print("unknown scenario " .. scenario)
	os.exit(2)
end

-- Monster IDs must be outside the summon ID range of the selected server type
local mobBase = sc.private and 110000000 or 60000
ent(OWNER, { x = 100, y = 100, hp = 10000, maxhp = 10000, sp = 500, maxsp = 500, class = 4060 })
ent(HOMUN, { x = 101, y = 100, hp = 20000, maxhp = 20000, sp = sc.sp or 800, maxsp = sc.sp or 800,
	homuntype = sc.homuntype, atkrange = 1 })
for i, m in ipairs(sc.mobs) do
	ent(mobBase + i, { monster = true, x = m[1], y = m[2], hp = 99999, maxhp = 99999, sp = 0, maxsp = 0,
		class = 1002, target = OWNER, motion = MOTION_ATTACK, atkrange = 1 })
end

-- Config overrides go in right after H_Config.lua, before AI_main.lua applies them,
-- exactly as if the user had saved them in the config file.
local realdofile = dofile
dofile = function(path)
	local r = realdofile(path)
	if path:find("H_Config.lua", 1, true) then
		if sc.private then ServerType = SERVER_PRIVATE end
		if sc.set then for k, v in pairs(sc.set) do _G[k] = v end end
	end
	return r
end
dofile("./AI/USER_AI/AI.lua")
dofile = realdofile
for _, name in ipairs(sc.skills) do
	assert(_G[name], "unknown skill constant " .. name)
	skillKnown[_G[name]] = true
end

-- Simulation loop -----------------------------------------------------------

local runtimeError
for _ = 1, TICKS do
	local h = world[HOMUN]
	if not h.busyUntil or now >= h.busyUntil then
		local ok, e = pcall(AI, HOMUN)
		if not ok then runtimeError = e; break end
	end
	if h.destx then
		if h.x < h.destx then h.x = h.x + 1 elseif h.x > h.destx then h.x = h.x - 1 end
		if h.y < h.desty then h.y = h.y + 1 elseif h.y > h.desty then h.y = h.y - 1 end
		if h.x == h.destx and h.y == h.desty then h.destx = nil; h.motion = MOTION_STAND end
	end
	if h.motion == MOTION_SKILL then h.motion = MOTION_STAND end
	if now % 1000 == 0 then h.sp = math.min(h.maxsp, h.sp + (sc.spregen or 10)) end
	now = now + STEP
end

-- Report --------------------------------------------------------------------

local verbose = os.getenv("HARNESS_VERBOSE") == "1"
local counts, lastCast, failures = {}, {}, {}
local function fail(msg) failures[#failures + 1] = msg end

for _, c in ipairs(casts) do
	if verbose then
		print(("  t=%6d %-3s %-24s Lv%-2d target=%s,%s ok=%s sp=%d"):format(
			c.t, c.kind, c.name, c.lvl, tostring(c.a), tostring(c.b), tostring(c.ok), c.sp))
	end
	if c.ok then
		counts[c.name] = (counts[c.name] or 0) + 1
		-- skills tracked by AutoSkillCooldown must never be recast before their reuse delay
		local reuse = skillField(c.skill, 9, c.lvl)
		if AutoSkillCooldown[c.skill] ~= nil and lastCast[c.skill] and c.t - lastCast[c.skill] < reuse then
			fail(("%s recast after %d ms, reuse is %d ms"):format(c.name, c.t - lastCast[c.skill], reuse))
		end
		lastCast[c.skill] = c.t
	else
		fail(("%s Lv%d cast without enough SP at t=%d"):format(c.name, c.lvl, c.t))
	end
end

if runtimeError then fail("runtime error: " .. tostring(runtimeError)) end
for _, name in ipairs(sc.expect or {}) do
	if not counts[name] then fail("expected skill never cast: " .. name) end
end
for _, name in ipairs(sc.forbid or {}) do
	if counts[name] then fail("forbidden skill was cast: " .. name) end
end
local attacks = world[HOMUN].attacks or 0
if attacks < (sc.minAttacks or 1) then
	fail(("only %d normal attacks, expected at least %d"):format(attacks, sc.minAttacks or 1))
end
for desc, check in pairs(sc.checks or {}) do
	local ok, res = pcall(check)
	if not ok or not res then fail("check failed: " .. desc .. (ok and "" or (" (" .. tostring(res) .. ")"))) end
end

local names = {}
for k in pairs(counts) do names[#names + 1] = k end
table.sort(names)
local summary = {}
for _, k in ipairs(names) do summary[#summary + 1] = k .. " x" .. counts[k] end
print(("%-16s %s attacks=%d casts: %s"):format(scenario, #failures == 0 and "PASS" or "FAIL", attacks,
	table.concat(summary, ", ")))
for _, f in ipairs(failures) do print("    - " .. f) end
if os.getenv("HARNESS_TRACE") == "1" then for _, t in ipairs(traces) do print("  TRACE " .. t) end end
os.exit(#failures == 0 and 0 or 1)
