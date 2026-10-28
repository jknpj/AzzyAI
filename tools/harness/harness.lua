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
-- wall clock follows the simulated time, so "once per hour" logic can be tested
local realtime = os.time
os.time = function(t) if t then return realtime(t) end return 1700000000 + math.floor(now / 1000) end
local keepTraces = os.getenv("HARNESS_TRACE") == "1"
function TraceAI(s) if keepTraces then traces[#traces + 1] = now .. " " .. tostring(s) end end
function GetMsg(id) return { 0 } end
function GetResMsg(id) return { 0 } end
function GetActors()
	local r = {}
	for id, e in pairs(world) do if not e.dead then r[#r + 1] = id end end
	table.sort(r)
	return r
end
local selfMonster = false -- scenario option: the client reports our own homunculus as a monster
function IsMonster(id)
	if selfMonster and id == HOMUN then return 1 end
	local e = world[id]; return (e and e.monster) and 1 or 0
end
function GetDistance(x1, y1, x2, y2) return math.floor(math.sqrt((x1 - x2) ^ 2 + (y1 - y2) ^ 2)) end
function IsInAttackSight(a, b, skill, lvl)
	local A, B = world[a], world[b]
	if not A or not B then return false end
	local range = A.atkrange or 1
	if skill and skill ~= 0 then range = math.max(rangeOf(skill, lvl), 1) end
	return math.max(math.abs(A.x - B.x), math.abs(A.y - B.y)) <= range
end

local skillKnown = {}
local getvCalls = {}
function GetV(v, id, extra, extra2)
	getvCalls[v] = (getvCalls[v] or 0) + 1
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
	-- like the client: unknown skills (or levels above the learned one) report range 1
	if v == V_SKILLATTACKRANGE then
		if skillKnown[extra] then return math.max(rangeOf(extra, skillKnown[extra]), 2) end
		return 1
	end
	if v == V_SKILLATTACKRANGE_LEVEL then
		if skillKnown[extra] and (extra2 or 1) <= skillKnown[extra] then return rangeOf(extra, extra2) end
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
	local e, t = world[id], world[target]
	if not e then return end
	-- only attacks on real monsters count; anything else (self, owner, players) is a bug
	if t and t.monster then
		e.motion = MOTION_ATTACK; e.attacks = (e.attacks or 0) + 1
	else
		e.badAttacks = (e.badAttacks or 0) + 1; e.badTarget = target
	end
end

local function recordCast(kind, id, lvl, skill, a, b)
	local e = world[id]
	local name = (SkillInfo and SkillInfo[skill] and SkillInfo[skill][1]) or tostring(skill)
	-- like the server: a skill or level the homunculus hasn't learned is refused ("Skill failed")
	local learned = id ~= HOMUN or (skillKnown[skill] and lvl <= skillKnown[skill])
	-- rAthena skill_check_condition_mercenary: homunculus/mercenary need MORE SP than the cost
	local ok = learned and e.sp > skillField(skill, 3, lvl)
	if ok then
		e.sp = e.sp - skillField(skill, 3, lvl)
		e.motion = MOTION_SKILL
		-- the client stops calling AI() while casting and during after-cast delay
		e.busyUntil = now + skillField(skill, 4, lvl) + skillField(skill, 5, lvl) + skillField(skill, 6, lvl)
	end
	casts[#casts + 1] = { t = now, kind = kind, skill = skill, name = name, lvl = lvl, a = a, b = b, ok = ok,
		refused = not learned, sp = e.sp }
end
HarnessCasts = casts
-- refused casts at or after game time t (for checks after an AI reload)
function HarnessRefusedSince(t)
	local n = 0
	for _, c in ipairs(casts) do if c.refused and c.t >= t then n = n + 1 end end
	return n
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
if sc.ticks then TICKS = math.max(TICKS, sc.ticks) end
selfMonster = sc.selfMonster == true

-- Monster IDs must be outside the summon ID range of the selected server type
local mobBase = (sc.private or sc.privateIds) and 110000000 or 60000
ent(OWNER, { x = 100, y = 100, hp = 10000, maxhp = 10000, sp = 500, maxsp = 500, class = 4060 })
ent(HOMUN, { x = 101, y = 100, hp = 20000, maxhp = 20000, sp = sc.sp or 800, maxsp = sc.sp or 800,
	homuntype = sc.homuntype, atkrange = 1 })
for i, m in ipairs(sc.mobs) do
	ent(mobBase + i, { monster = true, x = m[1], y = m[2], hp = 99999, maxhp = 99999, sp = 0, maxsp = 0,
		class = 1002, target = sc.passive and 0 or OWNER, motion = sc.passive and MOTION_STAND or MOTION_ATTACK, atkrange = 1 })
end

-- Config overrides go in right after H_Config.lua, before AI_main.lua applies them,
-- exactly as if the user had saved them in the config file.
local realdofile = dofile
-- the client starts every AI load in a fresh Lua state; without this, wrappers such as
-- OldSkillGround=SkillGround would wrap themselves on reload and recurse forever
local cleanG = {}
for k, v in pairs(_G) do cleanG[k] = v end
local function loadAI()
	for k in pairs(_G) do if cleanG[k] == nil then _G[k] = nil end end
	for k, v in pairs(cleanG) do _G[k] = v end
	dofile = function(path)
		local r = realdofile(path)
		if path:find("H_Config.lua", 1, true) then
			if sc.private then ServerType = SERVER_PRIVATE end
			if sc.set then for k, v in pairs(sc.set) do _G[k] = v end end
		end
		return r
	end
	local ok, e = pcall(realdofile, "./AI/USER_AI/AI.lua")
	dofile = realdofile
	if not ok then error(e, 0) end
end
loadAI()
for _, entry in ipairs(sc.skills) do
	-- "MH_TOXIN_OF_MANDARA:3" = learned at level 3; no suffix = every level
	local name, lvl = entry:match("^([%w_]+):?(%d*)$")
	assert(_G[name], "unknown skill constant " .. name)
	skillKnown[_G[name]] = tonumber(lvl) or 99
end

-- Simulation loop -----------------------------------------------------------

-- HARNESS_PROFILE=1: time every AI() call and sample which functions run (every 100 VM instructions)
local profile = os.getenv("HARNESS_PROFILE") == "1"
local samples, aiTime, aiMax, aiCalls = {}, 0, 0, 0
local function sampler()
	local info = debug.getinfo(2, "S")
	if info then
		local key = info.short_src:match("[^/\\]*$") .. ":" .. info.linedefined
		samples[key] = (samples[key] or 0) + 1
	end
end
collectgarbage("collect")
local memStart = collectgarbage("count")

local function globalTableSizes()
	local sizes = {}
	for k, v in pairs(_G) do
		if type(v) == "table" and v ~= _G and type(k) == "string" then
			local n = 0
			for _, vv in pairs(v) do
				n = n + 1
				if type(vv) == "table" then for _ in pairs(vv) do n = n + 1 end end
			end
			sizes[k] = n
		end
	end
	return sizes
end

local runtimeError
local sizesEarly
for tick = 1, TICKS do
	if profile and tick == 20 then sizesEarly = globalTableSizes() end
	-- the client reloads the AI on every map change/teleport; only files in data/ survive
	if sc.reloadAt and tick == sc.reloadAt then
		loadAI()
		if sc.afterReload then sc.afterReload(skillKnown) end
	end
	local h = world[HOMUN]
	if not h.busyUntil or now >= h.busyUntil then
		local t0 = profile and os.clock()
		if profile then debug.sethook(sampler, "", 100) end
		local ok, e = pcall(AI, HOMUN)
		if profile then
			debug.sethook()
			local dt = os.clock() - t0
			aiTime, aiCalls = aiTime + dt, aiCalls + 1
			if dt > aiMax then aiMax = dt end
		end
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
local counts, lastCast, failures, refused = {}, {}, {}, {}
local function fail(msg) failures[#failures + 1] = msg end

for _, c in ipairs(casts) do
	if verbose then
		print(("  t=%6d %-3s %-24s Lv%-2d target=%s,%s ok=%s sp=%d"):format(
			c.t, c.kind, c.name, c.lvl, tostring(c.a), tostring(c.b), tostring(c.ok), c.sp))
	end
	if c.refused then
		refused[#refused + 1] = c.name .. " Lv" .. c.lvl
	elseif c.ok then
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
-- every refused cast is a "Skill failed" message in the client
if #refused > (sc.maxRefused or 0) then
	fail(("%d casts refused by the server (max %d): %s"):format(#refused, sc.maxRefused or 0,
		table.concat(refused, ", ", 1, math.min(#refused, 8))))
end
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
if (world[HOMUN].badAttacks or 0) > 0 then
	fail(("%d attacks on non-monster actor %s"):format(world[HOMUN].badAttacks, tostring(world[HOMUN].badTarget)))
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
if profile and aiCalls > 0 then
	collectgarbage("collect")
	print(("    profile: %d AI calls, avg %.3f ms, max %.3f ms, heap %+.0f KB after GC"):format(
		aiCalls, aiTime / aiCalls * 1000, aiMax * 1000, collectgarbage("count") - memStart))
	local vnames = {}
	for k, val in pairs(_G) do
		if type(k) == "string" and k:sub(1, 2) == "V_" and type(val) == "number" then vnames[val] = k end
	end
	local gv, gtotal = {}, 0
	for v, n in pairs(getvCalls) do gv[#gv + 1] = v; gtotal = gtotal + n end
	table.sort(gv, function(a, b) return getvCalls[a] > getvCalls[b] end)
	local parts = {}
	for i = 1, math.min(6, #gv) do
		parts[#parts + 1] = (vnames[gv[i]] or tostring(gv[i])) .. "=" .. math.floor(getvCalls[gv[i]] / aiCalls)
	end
	print(("    GetV per AI call: %d (%s)"):format(math.floor(gtotal / aiCalls), table.concat(parts, ", ")))
	local keys, total = {}, 0
	for k, n in pairs(samples) do keys[#keys + 1] = k; total = total + n end
	table.sort(keys, function(a, b) return samples[a] > samples[b] end)
	for i = 1, math.min(15, #keys) do
		print(("      %5.1f%%  %s"):format(samples[keys[i]] * 100 / total, keys[i]))
	end
	-- global tables that keep growing during the run are memory leaks
	if sizesEarly then
		for k, n in pairs(globalTableSizes()) do
			if n - (sizesEarly[k] or 0) > 50 then
				print(("    growing table: %s %d -> %d entries"):format(k, sizesEarly[k] or 0, n))
			end
		end
	end
end
os.exit(#failures == 0 and 0 or 1)
