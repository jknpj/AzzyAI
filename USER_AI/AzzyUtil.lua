-------------------------------
-- This file is part of AzzyAI 1.55
-- If you want to use these functions in your own AI
-- it is reccomended that you use the seperately available
-- version, which does not utilize AAI-specific systems. 
-- Written by Dr. Azzy of iRO Chaos
AUVersion="1.56"
-------------------------------




--[[
USAGE NOTES


HPPercent(id)
SPPercent(id)
Returns the current hp or sp of id as a percentage. This only works on things which the player can see the hp/sp of, obviously.

Closer(id,x,y)
Returns coordinants of a location 1 cell closer to actor (id) 

ClosestR(id1,id2,range)
Returns coordinants of the cell within <range> cells of <id2> that is closest to <id1>.
Intended for targeting of ranged skills.

BetterMoveToOwner(mercid,range)
Moves to the closest sell withing range cells of the owner. 

IsNotKS(id)
Returns 1 if attacking monster (id) would be a KS, 0 otherwise. 

IsFriend(id)
Returns 1 if (id) is the owner or a friend. 

@@@@@@@@@@@@@@@@@@@@@@@@@
@@@ Monster Functions @@@
@@@@@@@@@@@@@@@@@@@@@@@@@

GetMobCount(id,range,[id2])
Returns the number of monsters within range cells of the actor id, and the number of said monsters which are currently targeting id. If id2 is specified, monsters which are targeting id2 will also be included in the second value returned. 
Useful for deciding whether to use anti-mob skills.
ex:
local mobcount,aggrocount = GetMobCount(MyID,1,OwnerID)

GetBestBrandishTarget(id)
Returns the id of the best monster adjacent to the mercenary to use brandish spear on (that is, the target that will result in the most monsters being hit). 
Assumes level 1 brandish area. 

ex:
SkillObject(MyID,MySkillLevel,ML_BRANDISH,GetBestBrandishTarget(MyID))


@@@@@@@@@@@@@@@@@@@@@@@
@@@ Skill Functions @@@
@@@@@@@@@@@@@@@@@@@@@@@

GetAtkSkill(id)
Returns a list containing skill id, skill level of the mercenary's single target offencive skill. Crash is not returned in this list, because the damage output spamming crash is lower than the damage output from using normal attacks. 

ex: 
local skill,level= GetAtkSkill(MyID)

GetMobSkill(merctype)
As GetAtkSkill() only for antimob skills

ex
local skill,level = GetMobSkill(MyID)

GetQuickenSkill(id)
Returns a list containing the ID of the weapon quicken skill posessed by the mercenary and the skill level

ex
local skill,level = GetQuickenSkill(MyID)

GetGuardSkill(id)
As GetQuickenSkill() only returns the ID of the guard or parry skill posessed by the mercenary. 

GetSacrificeSkill(id)
As GetQuickenSkill() only returns the ID and other perameters for the Sacrifice (aka Devotion) skill posessed by the mercenary.

GetProvokeSkill(id)
As GetQuiceknSkill() only returns the ID and other perameters for the Provoke skill posessed by the mercenary. 

GetPushbackSkill(id)
As GetMobSkill() only returns the ID and other perameters for the pushback skill (arrow repel or skid trap) posessed by the mercenary

--]]--


--#########################
--### Tactics Functions ###
--######################### 

function 	GetTact(t,m)
	local x
	if m==nil then
		TraceAI("GetTact: Error - nil target")
		logappend("AAI_ERROR","GetTact: Error - nil target")
		return 0
	end
	if t==nil then
		TraceAI("GetTact: Error - nil tactic target:"..m)
		logappend("AAI_ERROR","request for tactic type nil target:"..m)
	end
	if (IsPlayer(m)==1) then
		if (PVPmode~=0) then
			TraceAI("GetTact: Returning pvp tactic")
			return GetPVPTact(t,m)
		else
			return 0
		end
	end
	-- The tactic row only depends on the actor's class, which can't change within one AI() call.
	-- AI() empties TactRowCache at the start of every tick.
	local temp = TactRowCache and TactRowCache[m]
	local e = 0
	if temp == nil then
		if (IsHomun(MyID)==0) then 
			if (MobID[m]==nil) then
				e=GetClass(m)
			else
				e=MobID[m]
			end
		else
			e=GetActorClass(m)
		end
		temp=GetMyTact(e)
		if (temp==nil) then
			if (e >= 1324 and e <= 1363) or (e >= 1938 and e <=1946) then
				temp=GetMyTact(13) -- treasure chest
			end
			if temp==nil then
				temp=GetMyTact(0)
			end
		end
		if TactRowCache then
			TactRowCache[m] = temp
		end
	end
	if (temp[t]==nil) then
		temp=GetMyTact(0)
		TraceAI("GetTact: Undefined tactic "..t.." for actor: "..m.." using default instead")
	end
	x=temp[t]
	if (x==nil) then
		logappend("AAI_ERROR","Default tactic "..t.." is undefined - Please review and correct tactics file or restore default tactics file")
		return 0
	else
		x = MobInfoTact(t, x, m)
		if t==TACT_SP and x==-1 then
			x = AttackSkillReserveSP
		elseif t==TACT_CHASE then
			--TraceAI("TactChase called"..GetV(V_SP,MyID).." "..ChaseSPPauseSP.." "..GetTick().." "..math.max(LastMovedTime,LastSPTime).." "..(5000-ChaseSPPauseTime))
			if GetV(V_SP,MyID) < ChaseSPPauseSP and (GetTick() - math.max(LastMovedTime,LastSPTime)) > (5000-ChaseSPPauseTime) then 
				if ((ChaseSPPause==1 and x~=CHASE_ALWAYS) or x==CHASE_CLEVER) then
					TraceAI("Cleverchase activated: "..m.." ("..e..") ".." LastTick "..math.max(LastMovedTime,LastSPTime))
					return 1
				end
			end
			if x==CHASE_ALWAYS then
				return 0
			elseif x==CHASE_NEVER then
				return 1
			else
				return DoNotChase
			end
		end
		return x
	end
end

function GetMyTact(m)
	return MyTact[m]
end

--########################
--### Monster database ###
--########################
-- MobInfo.lua (generated from rAthena's mob_db by tools/harness/rathena_sync.py):
--   MobInfo[class] = {race, element, element level, size, flags}
--   ElementFix[element level][attack element][defense element] = damage %
--   SkillElement[skill] = element of a damaging skill
-- Homunculi read a monster's class with GetV(V_HOMUNTYPE); mercenaries only know it from MobID files.
MOB_FLAG_BOSS	= 1	-- boss protocol (MVPs and mini-bosses)
MOB_FLAG_MVP	= 2
MOB_FLAG_PLANT	= 4	-- takes 1 damage from everything (plants, mushrooms)
MobInfoCache = {}
ActorClassCache = {}

-- A homunculus' view of an actor's class (monster ID), read once per AI() call and shared by
-- GetTact and GetMobInfo; AI() empties ActorClassCache at the start of every tick.
function GetActorClass(id)
	local class = ActorClassCache[id]
	if class == nil then
		class = GetV(V_HOMUNTYPE, id)
		ActorClassCache[id] = class
	end
	return class
end

function GetMobInfo(id)
	if MobInfo == nil or UseMobInfo == 0 or id == nil or id == 0 then
		return nil
	end
	local info = MobInfoCache[id]
	if info == nil then
		local class
		if IsHomun(MyID) == 1 then
			class = GetActorClass(id)
		else
			class = MobID[id]
		end
		info = (class and MobInfo[class]) or false
		MobInfoCache[id] = info
	end
	return info or nil
end

function HasMobFlag(info, flag)
	return info ~= nil and math.floor(info[5] / flag) % 2 == 1
end

function IsPlantMob(id)
	return HasMobFlag(GetMobInfo(id), MOB_FLAG_PLANT)
end

-- 2 = MVP, 1 = other boss-protocol monster, 0 = neither or unknown
function GetBossLevel(id)
	local info = GetMobInfo(id)
	if HasMobFlag(info, MOB_FLAG_MVP) then
		return 2
	elseif HasMobFlag(info, MOB_FLAG_BOSS) then
		return 1
	end
	return 0
end

-- % of a skill's damage that the target takes because of its element (100 when unknown)
function SkillElementRate(skill, target)
	local element = SkillElement and SkillElement[skill]
	local info = GetMobInfo(target)
	if element == nil or info == nil or ElementFix == nil then
		return 100
	end
	local byLevel = ElementFix[info[3]]
	local row = byLevel and byLevel[element]
	return (row and row[info[2]]) or 100
end

-- False when the target's element shrugs off the skill (less than SkillElementMinRate % of the
-- damage; immune or healed counts as 0), so the AI attacks normally or picks another skill.
function SkillElementOK(skill, target)
	if UseMobInfo == 0 then
		return true
	end
	local rate = SkillElementRate(skill, target)
	return rate > 0 and rate >= (SkillElementMinRate or 0)
end

-- How much monster k counts toward an area skill: its tactics weight times the share of damage
-- its element lets through (Fire 2 vs fire = 0, Earth 2 = 1.75). Monsters below
-- SkillElementMinRate count 0, so packs of mixed elements are judged by who actually gets hurt.
function AoEMobWeight(skill, k)
	local weight = GetTact(TACT_WEIGHT, k)
	if skill == nil or UseMobInfo == 0 or SkillElement == nil or SkillElement[skill] == nil then
		return weight
	end
	if not SkillElementOK(skill, k) then
		return 0
	end
	return weight * SkillElementRate(skill, k) / 100
end

-- Element check of an attack skill: single-target skills look at the target; area skills pass
-- when the area around the target would be hurt enough, even if the target itself is immune.
function SkillElementOKFor(skill, target)
	if SkillElementOK(skill, target) then
		return true
	end
	if UseMobInfo == 0 or GetSkillAoESize(skill, 1) == nil then
		return false
	end
	local level = SkillList[MyHomunType] and SkillList[MyHomunType][skill] or 1
	return GetMobCount(skill, level, target, 1) >= (AutoMobCount or 1)
end

-- Tactic changes from the monster database, applied on top of the tactics file:
--   bosses: attacking tactics become "react" ones (BossAggro: 1 = MVPs, 2 = all bosses)
--   plants: no skills (PlantSkills = 0) and no weight in AoE counts, since they take 1 damage
REACT_FOR_ATTACK = {}
function MobInfoTact(t, x, m)
	if UseMobInfo == 0 or MobInfo == nil then
		return x
	end
	if t == TACT_BASIC then
		if BossAggro ~= nil and BossAggro > 0 and x ~= TACT_IGNORE and GetBossLevel(m) >= 3 - BossAggro then
			return REACT_FOR_ATTACK[x] or x
		end
	elseif (t == TACT_SKILL or t == TACT_WEIGHT) and PlantSkills == 0 and IsPlantMob(m) then
		return 0
	end
	return x
end
REACT_FOR_ATTACK[TACT_ATTACK_L] = TACT_REACT_L
REACT_FOR_ATTACK[TACT_ATTACK_M] = TACT_REACT_M
REACT_FOR_ATTACK[TACT_ATTACK_H] = TACT_REACT_H
REACT_FOR_ATTACK[TACT_SNIPE_L] = TACT_REACT_L
REACT_FOR_ATTACK[TACT_SNIPE_M] = TACT_REACT_M
REACT_FOR_ATTACK[TACT_SNIPE_H] = TACT_REACT_H
REACT_FOR_ATTACK[TACT_ATK_L_REACT_M] = TACT_REACT_M
REACT_FOR_ATTACK[TACT_ATTACK_LAST] = TACT_REACT_L
REACT_FOR_ATTACK[TACT_ATTACK_TOP] = TACT_REACT_H

function	GetPVPTact(t,m)
	local x
	local class
	local e = ENEMY
	if IsMonster(m)==0 then
		TraceAI("pvp tact lookup on ally")
		e= ALLY
	end
	if (MyPVPTact[m]~=nil) then
		e = m
	elseif (MyFriends[m]~=nil) then
		e = MyFriends[m]
	end
	if IsHomun(MyID)==1 then
		class=GetV(V_HOMUNTYPE,m)
	end
	if MyPVPTact[m]~=nil then
		x=MyPVPTact[m][t]
	elseif e~=ALLY and class~=nil then 
		if MyPVPTact[class] ~=nil then
			x=MyPVPTact[class][t]
		end
	elseif MyFriends[e]==nil then
		x=MyPVPTact[e][t]
	end
	if x==nil then
		x=MyPVPTact[0][t]
	end
	if (x==nil) then
		TraceAI("ERROR: Default PVP tactic "..t.." is undefined!")
		logappend("AAI_ERROR","Default PVP tactic "..t.." is undefined! Please correct your PVP tactics file")
		return 0
	else	
	TraceAI("pvp TL on P "..m.."IsMonster "..IsMonster(m).." type "..e.." for tactic "..t.." returning "..x)
		return x
	end
end

-- Extra lines for AAIStartH.txt / AAIStartM.txt, so a user can see which code and logs are active
function StartupDiagnostics()
	local logs = {}
	for name, on in pairs(LogEnable or {}) do
		if on == 1 then logs[#logs + 1] = name end
	end
	table.sort(logs)
	return "\nBuild: LATAM-HG (self-target fix, targeting log)"
		.."\nServerType: "..tostring(ServerType).." (auto-detect "..tostring(AutoDetectServerType)..")"
		.."\nLogs enabled: "..(#logs > 0 and table.concat(logs, ", ") or "none")
		.."\nMonster database (MobInfo.lua): "..(MobInfo ~= nil and "loaded" or "missing")
			.." (UseMobInfo "..tostring(UseMobInfo)..", BossAggro "..tostring(BossAggro)..")"
		.."\nLogs are written to: the RO client folder (next to this file)"
end

function LogTargetingSnapshot(actors)
	if LogEnable["AAI_TARGETING"] ~= 1 then
		return
	end
	local now = GetTick()
	if LastTargetingLogTick and now >= LastTargetingLogTick and now - LastTargetingLogTick < 5000 then
		return
	end
	LastTargetingLogTick = now
	local monsters, targets = 0, 0
	for _ in pairs(Monsters) do monsters = monsters + 1 end
	for _ in pairs(Targets) do targets = targets + 1 end
	local lines = {
		"Snapshot tick="..now.." homun="..tostring(MyID).." owner="..tostring(OwnerID)
			.." state="..tostring(MyState).." enemy="..tostring(MyEnemy)
			.." ServerType="..tostring(ServerType).." actors="..#actors.." monsters="..monsters.." targets="..targets,
		"Thresholds MagicNumber="..tostring(MagicNumber).." MagicNumber2="..tostring(MagicNumber2)
			.." MagicNumber3="..tostring(MagicNumber3),
		"Settings SuperPassive="..tostring(SuperPassive).." ShouldStandby="..tostring(ShouldStandby)
			.." StickyStandby="..tostring(StickyStandby).." HP="..HPPercent(MyID).." AggroHP="..tostring(AggroHP)
			.." SP="..SPPercent(MyID).." AggroSP="..tostring(AggroSP)
			.." aggroDistance="..GetAggroDist().." moveBounds="..GetMoveBounds()
			.." DoNotAttackMoving="..tostring(DoNotAttackMoving).." UseSkillOnly="..tostring(UseSkillOnly),
	}
	local logged = 0
	for _, id in ipairs(actors) do
		if id ~= MyID and id ~= OwnerID then
			if logged == 20 then
				lines[#lines + 1] = "Remaining actors omitted (20-actor limit)"
				break
			end
			local monster = IsMonster(id)
			local x, y = GetV(V_POSITION, id)
			local detail = "Actor "..id.." nativeMonster="..tostring(monster).." player="..IsPlayer(id)
				.." class="..tostring(GetV(V_HOMUNTYPE, id)).." pos="..x..","..y
				.." motion="..tostring(GetV(V_MOTION, id)).." target="..tostring(GetV(V_TARGET, id))
				.." inTargets="..tostring(Targets[id] ~= nil).." ownerDistance="..GetDistanceA(OwnerID, id)
				.." unreachable="..tostring(Unreachable[id])
			if monster == 1 then
				detail = detail.." tactic="..GetTact(TACT_BASIC, id).." KS="..GetKSReason(MyID, id)
			end
			lines[#lines + 1] = detail
			logged = logged + 1
		end
	end
	logappend("AAI_TARGETING", table.concat(lines, "\n"))
end

function ApplyServerType()
	if ServerType ~= SERVER_PRIVATE then
		ServerType = SERVER_OFFICIAL
	end
	MagicNumbersFromConfig = MagicNumber ~= nil or MagicNumber2 ~= nil or MagicNumber3 ~= nil
	if ServerType == SERVER_PRIVATE then
		-- rAthena/Hercules: accounts 2000000-100000000, npcs/mobs/summons from 110000000
		MagicNumber  = MagicNumber  or 110000000
		MagicNumber2 = MagicNumber2 or 2000000
		MagicNumber3 = MagicNumber3 or 100000000
	else
		MagicNumber  = MagicNumber  or 42000
		MagicNumber2 = MagicNumber2 or 100000
	end
	ApplyServerSkillOverrides()
end

-- Called for every actor in sight while ServerType is official (see AI()).
function DetectPrivateServer(id)
	if AutoDetectServerType ~= 1 or MagicNumbersFromConfig or id < 110000000 or IsMonster(id) ~= 1 then
		return
	end
	logappend("AAI_ERROR", "Monster ID "..id.." is from an rAthena/Hercules server: switching to ServerType = SERVER_PRIVATE. Set it in H_Config.lua to skip this check.")
	TraceAI("Private server detected from monster ID "..id)
	ServerType = SERVER_PRIVATE
	MagicNumber, MagicNumber2, MagicNumber3 = nil, nil, nil
	ApplyServerType()
end

-- Swaps per-server skill data (range, cooldown, targeting) defined in the SkillList files
function ApplyServerSkillOverrides()
	if ServerSkillOverrides==nil or ServerSkillOverrides[ServerType]==nil or SkillInfo==nil then
		return
	end
	for skill,o in pairs(ServerSkillOverrides[ServerType]) do
		if SkillInfo[skill]~=nil and o.info~=nil then
			for idx,value in pairs(o.info) do
				SkillInfo[skill][idx]=value
			end
		end
		if o.aoe~=nil and SkillAOEInfo~=nil then
			SkillAOEInfo[skill]=o.aoe
		end
		if o.spheres~=nil and SkillSphereCost~=nil then
			SkillSphereCost[skill]=o.spheres
		end
	end
end

-- Low actor IDs are summons/retainers on official servers only; private servers
-- share one ID range for monsters and summons, so they can't be told apart by ID.
function IsSummonID(id)
	if ServerType == SERVER_PRIVATE then
		return 0
	elseif id < MagicNumber then
		return 1
	end
	return 0
end

function	GetClass(m)
	if (IsSummonID(m)==1) then
		return 10
	elseif (IsActive[m]==0 and AutoDetectPlant==1) then
		return 11
	else
		return 0
	end
end

function IsRescueTarget(id)
	local tactrescue=GetTact(TACT_RESCUE,id)
	local target=GetV(V_TARGET,id)
	local owner=OwnerID
	if target==owner then
		local hp = HPPercent(owner)
		if RescueOwnerLowHP > hp then
			TraceAI("RescueOwnerLowHP -> rescue owner from "..id)
			return 1
		elseif RescueOwnerLowHP < 0 and -1*RescueOwnerLowHP < hp then
			TraceAI("RescueOwnerLowHP -> rescue of owner canceled "..id)
			return 0
		end
	end
	if tactrescue==RESCUE_NEVER then
		return 0
	else	
		TraceAI("tactrescue not never "..tactrescue.." id "..id)
		if tactrescue==RESCUE_RETAINER then
			if MyFriends[target]==2 then
				return 1
			end
		elseif tactrescue==RESCUE_FRIEND then
			if MyFriends[target]==1 then
				return 1
			end
		elseif tactrescue==RESCUE_ALL then
			if IsFriendOrSelf(target)==1 and target ~=MyID then
				return 1
			end
		elseif tactrescue==RESCUE_OWNER then
			if target==OwnerID then
				return 1
			end
		elseif tactrescue==RESCUE_SELF then
			if target==MyID then
				return 1
			end
		end
	end
	return 0
end

function GetTargetClass(id)
-- 0 = no target, 1 = self 2= friend, -1 = other monster -2 = other player
	if id == MyID then
		return 1
	elseif id == 0 then
		return 0
	elseif (IsPlayer(id)==1) then
		if IsFriendOrSelf(id)==1 then
			return 2
		else
			return -2
		end
	elseif IsMonster(id)==1 then
		return -1
	else
		return 0
	end
end

--########################
--### Friend Functions ###
--########################


function IsFriend(id)
	if (id==OwnerID) then
		return 1
	elseif (id~= MyID) then
		local friendclass=MyFriends[id]
		if (friendclass==FRIEND or friendclass==RETAINER) then
			return 1
		end
	end
	return 0
end

function IsPVPFriend(id)
	if (id==OwnerID) then
		return 1
	elseif (id~= MyID) then
		local friendclass=MyFriends[id]
		if (friendclass==FRIEND or friendclass==RETAINER or friendclass==NEUTRAL or friendclass==ALLY) then
			return 1
		end
	end
	return 0
end

function IsFriendOrSelf(id)
	if (id==OwnerID or id == MyID) then
		return 1
	else
		if (MyFriends[id]~=nil) then
			return 1	
		end
	end
	return 0
end

function IsPlayer(id)
	if ServerType == SERVER_PRIVATE then
		if (id >= MagicNumber2 and id <= MagicNumber3) then
			return 1
		end
	elseif (id>MagicNumber2) then
		return 1
	end
	return 0
end

function UpdateFriends() 
	FriendsFile = io.open(ConfigPath.."A_Friends.lua", "w")
	if FriendsFile~=nil then
		FriendsFile:write (STRING_A_FRIENDS_HEAD)
		for k,v in pairs(MyFriends) do
			if (v==FRIEND or (v==PKFRIEND and PainkillerFriendsSave == 1)) then
				FriendsFile:write ("MyFriends["..k.."]="..v.." -- \n")
			end
		end
		FriendsFile:close()
	else
		TraceAI("Failed to open A_Friends.lua for writing!")
		logappend("AAI_ERROR","Failed to open A_Friends.lua for writing!")
	end
end

--########################
--### Monster functions###
--########################


function GetAggroCount(target)
	local aggrocount=0
	if target ==nil then
		for k,v in pairs(Targets) do
			if v[2] > 0 then -- it's aggressive
				local distance = GetDistanceA(OwnerID,k)
				if (distance <= GetMoveBounds()) then
					aggrocount=aggrocount+1*GetTact(TACT_WEIGHT,k)
				end
			end
		end
	else
		for k,v in pairs(Targets) do
			if GetV(V_TARGET,k)==target then
				aggrocount=aggrocount+1*GetTact(TACT_WEIGHT,k)
			end
		end
	end
	return aggrocount
end

function GetMobCount(skill,level,target,aggro)
	local mobcount=0
	if skill==0 or level==0 then 
		return 0
	end
	local skillaoe=GetSkillAoESize(skill,level)
	local x,y=GetPos(MyID)
	if skillaoe==nil then
		if skill == ML_BRANDISH then
			for k,v in pairs (Targets) do
				if GetV(V_MOTION,k)~=MOTION_DEAD then
					if (GetDistanceA(MyID,k)<=1 and GetDistanceA(target,k) <=1 and (v[2]>0 or aggro~=0)) then
						mobcount=mobcount+1*GetTact(TACT_WEIGHT,k)
					elseif (GetDistanceA(MyID,k) ==2 and GetDistance3(MyID,k) < 2.5 and GetDistance3(target,k) < 2.5 and (v[2]>0 or aggro~=0)) then -- sides, 2 cells to either side of target/merc
						mobcount=mobcount+1*GetTact(TACT_WEIGHT,k)
					end
				end
			end
		elseif skill== MA_SHARPSHOOTING then
			local mx,my=GetV(V_POSITION,target)
			local dx=math.abs(x-mx)
			local dy=math.abs(y-my)
			if ((dx==0) or (dx==1 and dy>1) or (dx==2 and dy>6) or (dx==3 and dy>9) or (dx==4 and dy>11)) then 
				if my>y then
					mobcount=GetFASTargetCount(MyID,0,1,aggro)
				elseif my<y then
					mobcount=GetFASTargetCount(MyID,0,-1,aggro)
				end
			elseif ((dy==0) or (dy==1 and dx>1) or (dy==2 and dx>6) or (dy==3 and dx>9) or (dy==4 and dx>11)) then
				if mx>x then
					mobcount=GetFASTargetCount(MyID,1,0,aggro)
				elseif mx<x then
					mobcount=GetFASTargetCount(MyID,-1,0,aggro)
				end
			-- diagonals
			elseif mx>x then
				if my>y then
					mobcount=GetFASTargetCount(MyID,1,1,aggro)
				else
					mobcount=GetFASTargetCount(MyID,-1,1,aggro)
				end
			elseif my>y then
				mobcount=GetFASTargetCount(MyID,1,-1,aggro)
			else
				mobcount=GetFASTargetCount(MyID,-1,-1,aggro)
			end
		end
	else -- we have a wellbehaved skill
		if IsCasterCenteredAoE(skill) then
			target=MyID
		end
		local range=(skillaoe-1)/2
		local logging = LogEnable and LogEnable["AAI_MOBCOUNT"] == 1
		local logstring=""
		-- the center's position is read once, not once per monster
		local tx,ty=GetPos(target)
		for k,v in pairs(Targets) do
			local tbas=GetTact(TACT_BASIC,k)
			local dist = GetDistanceAPR(k,tx,ty)
			if logging then
				logstring=logstring.."|"..k.."v[2]"..v[2].."t"..GetV(V_TARGET,k).."m"..GetV(V_MOTION,k).."aggro"..aggro
			end
			if dist <= range and (v[2]>0 or aggro~=0) and tbas~=0 then
				local w = AoEMobWeight(skill,k)
				mobcount=mobcount+w
				if logging then
					logstring=logstring.."add:"..w.."new:"..mobcount
				end
			end			
		end
		if logging then
			logappend("AAI_MOBCOUNT","Mobcount on well-behaved skill "..FormatSkill(skill,level).." range "..range.." logstring: "..logstring.." result: "..mobcount)
		end
	end
	return mobcount
end

-- Monster to aim a target-centered AoE at: the one whose area holds the most AoEMobWeight
-- (same count as GetMobCount(skill,level,k,1)). Positions and weights are read once instead of
-- once per candidate, so a screen of n monsters costs n GetV calls rather than n^2.
function GetBestAoETarget(myid,skill,level)
	local size=GetSkillAoESize(skill,level)
	if size==nil or IsCasterCenteredAoE(skill) then
		local mcount=0
		local result=-1
		for k,v in pairs(Targets) do
			if GetTact(TACT_BASIC,k)~=0 and IsInAttackSight(myid,k,skill,level)==true then
				local count=GetMobCount(skill,level,k,1)
				if mcount < count then
					result=k
					mcount=count
				end
			end
		end
		return result
	end
	local range=(size-1)/2
	local mobs={}
	local pos={}
	for k,v in pairs(Targets) do
		if GetTact(TACT_BASIC,k)~=0 then
			local x,y=GetPos(k)
			pos[k]={x,y}
			mobs[#mobs+1]={k,x,y,AoEMobWeight(skill,k)}
		end
	end
	local mcount=0
	local result=-1
	for k,v in pairs(Targets) do
		if pos[k]~=nil and IsInAttackSight(myid,k,skill,level)==true then
			local tx,ty=pos[k][1],pos[k][2]
			local count=0
			if tx ~= -1 then
				for i=1,#mobs do
					local m=mobs[i]
					if m[2] ~= -1 and math.abs(m[2]-tx) <= range and math.abs(m[3]-ty) <= range then
						count=count+m[4]
					end
				end
			end
			if mcount < count then
				result=k
				mcount=count
			end
		end
	end
	return result
end

-- Best cell to center a ground AoE on, within range of myid. Each monster within 20 cells of the
-- owner adds its AoEMobWeight; ties keep the first cell found (lowest x, then lowest y).
-- Only cells covered by some monster's area can beat 0, so just those are scored, each with a
-- 2D prefix sum lookup: O(monsters * area) instead of O(range^2 * area^2) per call.
function GetBestAoECoord(myid,skill,level)
	local range=GetSkillInfo(skill,2,level)
	local aoesize=((GetSkillAoESize(skill,level) or 1)-1)/2
	local myx,myy=GetPos(myid)
	local ox,oy=GetPos(OwnerID)
	-- weights summed per cell, and the bounding box of the occupied cells
	local cells, minx, maxx, miny, maxy = {}, nil, nil, nil, nil
	for k,v in pairs(Targets) do
		if GetTact(TACT_BASIC,k)~=0 then
			local x,y=GetPos(k)
			if x < ox+20 and x > ox-20 and y < oy+20 and y > oy-20 then
				local w = AoEMobWeight(skill,k)
				if w ~= 0 then
					local col = cells[x]
					if col == nil then col = {}; cells[x] = col end
					col[y] = (col[y] or 0) + w
					if minx == nil or x < minx then minx = x end
					if maxx == nil or x > maxx then maxx = x end
					if miny == nil or y < miny then miny = y end
					if maxy == nil or y > maxy then maxy = y end
				end
			end
		end
	end
	if minx == nil then
		return -1,-1
	end
	-- prefix sums over the box: P[x][y] = weight in [minx..x] x [miny..y]
	local P = {}
	local zero = {}
	for x=minx-1,maxx do P[x] = {} end
	for y=miny-1,maxy do P[minx-1][y] = 0 end
	for x=minx,maxx do
		local px, ppx, col = P[x], P[x-1], cells[x] or zero
		px[miny-1] = 0
		for y=miny,maxy do
			px[y] = (col[y] or 0) + px[y-1] + ppx[y] - ppx[y-1]
		end
	end
	local function boxsum(x1,y1,x2,y2)
		if x1 < minx then x1 = minx end
		if y1 < miny then y1 = miny end
		if x2 > maxx then x2 = maxx end
		if y2 > maxy then y2 = maxy end
		if x1 > x2 or y1 > y2 then return 0 end
		return P[x2][y2] - P[x1-1][y2] - P[x2][y1-1] + P[x1-1][y1-1]
	end
	-- candidate centers: in range of myid and within aoesize of some monster
	local cx1, cx2 = math.max(myx-range, minx-aoesize), math.min(myx+range, maxx+aoesize)
	local cy1, cy2 = math.max(myy-range, miny-aoesize), math.min(myy+range, maxy+aoesize)
	local mcount, resultx, resulty = 0, -1, -1
	for xx=cx1,cx2 do
		for yy=cy1,cy2 do
			local thiscount = boxsum(xx-aoesize,yy-aoesize,xx+aoesize,yy+aoesize)
			-- weights like 0.9 or 1.75 leave rounding noise in the sums: a cell must win by more than that
			if thiscount > mcount + 1e-6 then
				mcount = thiscount
				resultx,resulty = xx,yy
			end
		end
	end
	return resultx,resulty
end

function GetBestBrandishTarget(myid)
	local mobs =0
	local maxmobs = 0
	local besttarget= -1
	for k,v in pairs(Targets) do
		if (GetDistanceA(myid,k) <=1) then --Is a target in range to brandish
			mobs = 0
			for kk,vv in pairs (Targets) do
				local tbas=GetTact(TACT_BASIC,kk)
				if tbas ~= 0 then
					if (GetDistanceA(myid,kk)<=1 and GetDistanceA(k,kk) <=1) then
						mobs=mobs+1
					elseif (GetDistanceA(myid,kk) ==2 and GetDistance3(myid,kk) < 2.5 and GetDistance3(k,kk) < 2.5)then -- sides, 2 cells to either side of target/merc
						mobs=mobs+1
					end
				end
			end
			if (mobs > maxmobs) then
				maxmobs = mobs
				besttarget = v
			end
		end
	end
	return besttarget
end

function GetBestFASTarget(myid)
	local options = GetFASOptions(myid)
	local mincount=0
	local target=-1
	for x,v in pairs(options) do
		for y,v2 in pairs(v) do
			--TraceAI("GetBestFAST "..x.." "..y.." "..v2)
			if v2~=0 then
				local count=GetFASTargetCount(myid,x,y)
				if count > mincount then
					mincount = count
					target = v2
				end
			end
		end
	end
	return target,mincount			
end

function GetFASDirection(myid,target)
	local x,y=GetV(V_POSITION,myid)
	if GetDistanceA(myid,target) < 15 then
		local mx,my=GetV(V_POSITION,target)
		local dx=math.abs(x-mx)
		local dy=math.abs(y-my)
		if ((dx==0) or (dx==1 and dy>1) or (dx==2 and dy>6) or (dx==3 and dy>9) or (dx==4 and dy>11)) then 
			if my>=y then
				return 0,1 --n
			elseif my<y then
				return 0,-1 --s
			end
		elseif ((dy==0) or (dy==1 and dx>1) or (dy==2 and dx>6) or (dy==3 and dx>9) or (dy==4 and dx>11)) then
			if mx>x then
				return 1,0 --e
			elseif mx<x then
				return-1,0 --w
			end
		-- diagonals
		elseif mx>x then
			if my>=y then
				return 1,1 --ne
			else
				return 1,-1 --se
			end
		elseif my>=y then
			return -1,1 --nw
		else
			return -1,-1 --sw
		end
	end
end

function GetFASOptions(myid)
	local x,y=GetV(V_POSITION,myid)
	local noption,eoption,soption,woption=0,0,0,0
	local neoption,nwoption,seoption,swoption=0,0,0,0
	for k,v in pairs(Monsters) do
		TraceAI("FAS loop id: "..k)
		if GetDistanceA(myid,k) < 15 then
			local mx,my=GetV(V_POSITION,k)
			local dx=math.abs(x-mx)
			local dy=math.abs(y-my)
			if ((dx==0) or (dx==1 and dy>1) or (dx==2 and dy>6) or (dx==3 and dy>9) or (dx==4 and dy>11)) then 
				if my>y then
					noption=k
				elseif my<y then
					soption=k
				end
			elseif ((dy==0) or (dy==1 and dx>1) or (dy==2 and dx>6) or (dy==3 and dx>9) or (dy==4 and dx>11)) then
				if mx>x then
					eoption=k
				elseif mx<x then
					woption=k
				end
			-- diagonals
			elseif mx>x then
				if my>=y then
					neoption=k
				else
					seoption=k
				end
			elseif my>=y then
				nwoption=k
			else
				swoption=k
			end
		end
		if (noption~=0 and soption~=0 and eoption~=0 and woption~=0 and neoption~=0 and seoption~=0 and swoption~=0 and nwoption~=0) then
			TraceAI("All Options Avail - break")
			break
		end
	end
	local result={[-1]={[-1]=swoption,[0]=woption,[1]=nwoption},[0]={[-1]=soption,[0]=0,[1]=noption},[1]={[-1]=seoption,[0]=eoption,[1]=neoption}}
	TraceAI("Returning FAS Options: "..swoption.."-"..woption.."-"..nwoption.." "..soption.."--"..noption.." "..seoption.."-"..eoption.."-"..neoption)
	return result
end

function GetFASTargetCount(myid,directionx,directiony,aggro)
	local x,y=GetV(V_POSITION,myid)
	local s=directionx+directiony
	TraceAI("GFASTC: "..s)
	local count=0
	local xmin,xmax,ymin,ymax,miny,maxy
	if s==1 or s==-1 then -- straight
		if directiony==0 then --Horizontal
			ymin=y-1
			ymax=y+1
			if directionx==1 then --East
				xmin=x
				xmax=x+14
			else
				xmin=x-14
				xmax=x
			end
		else	--Vertical
			xmin=x-1
			xmax=x+1
			if directiony==1 then -- north
				ymin=y
				ymax=y+14
			else
				ymin=y-14
				ymax=y
			end
		end
		for k,v in pairs(Targets) do
			local mx,my=GetV(V_POSITION,k)
			--logappend("AAI_mobcount","straight monster:"..xmin.."-"..xmax..","..ymin.."-"..ymax.." "..mx..","..my)
			if GetTact(TACT_BASIC,k)~=0 and mx>=xmin and xmax>=mx and my>=ymin and ymax>=my then
				count=count+1
			end
		end
		--logappend("AAI_mobcount","straight:"..xmin.."-"..xmax..","..ymin.."-"..ymax.." count"..count)
		return count
	else	--Diagonal
		for k,v in pairs(Targets) do
			if GetTact(TACT_BASIC,k)~=0 and IsNotKS(MyID,k)==1 then
				local mx,my=GetV(V_POSITION,k)
				if directionx==1 then --east
					if (mx+1>=x and x+14>=mx) then -- has prayer of hitting
						if mx==x-1 then
							if my==y+directiony then
								count=count+1
							end
						elseif mx==x then
							if (my==y or my==y+directiony or my==y+2*directiony) then
								count=count+1
							end
						elseif mx==x+14 then
							if my==y+12*directiony then
								count=count+1
							end
						elseif mx==x+13 then
							if (my==y+11*directiony or my==y+10*directiony or my==y+12*directiony) then
								count=count+1
							end
						else	--not a special case
							if directiony==1 then
								miny=y-2+(mx-x)
								maxy=miny+5
							else
								maxy=y+2-(mx-x)
								miny=maxy-5
							end
							if my>=miny and maxy>=my then
								count=count+1
							end
						end
					end
				else --west
					if mx-1<=x and x-14<=mx then -- has prayer of hitting
						if mx==x+1 then
							if my==y+directiony then
								count=count+1
							end
						elseif mx==x then
							if (my==y or my==y+directiony or my==y+2*directiony) then
								count=count+1
							end
						elseif mx==x-14 then
							if my==y+12*directiony then
								count=count+1
							end
						elseif mx==x-13 then
							if (my==y+11*directiony or my==y+10*directiony or my==y+12*directiony) then
								count=count+1
							end
						else	--not a special case
							if directiony==1 then
								miny=y-2+(x-mx)
								maxy=miny+5
							else
								maxy=y+2-(x-mx)
								miny=maxy-5
							end
							if my>=miny and maxy>=my then
								count=count+1
							end
						end
					end
				end
			end
		end
		--logappend("AAI_mobcount","diagonal:"..count)
	end
	return count
end

function IsHiddenOnScreen(myid) --Currently disabled - assume there is never hidden on screen.
	return 1
end

function IsHomun(myid)
	-- AI.lua and AI_M.lua run in separate Lua states, so our own answer never changes
	if myid==MyID and myid~=nil then
		if IsHomunSelf==nil then
			IsHomunSelf = (GetV(V_HOMUNTYPE,myid)~=nil) and 1 or 0
		end
		return IsHomunSelf
	end
	if (GetV(V_HOMUNTYPE,myid)~=nil) then
		return 1
	end
	return 0

end

function GetMerType(myid)
	local merctype=GetV(V_MERTYPE,myid)
	TraceAI("Merc is homun?"..IsHomun(myid))
	if (merctype~=1) then
		return merctype
	elseif merctype > 30 then
		logappend("AAI_ERROR","Invalid merctype reported"..merctype)
		return merctype
	else
		mhp=GetV(V_MAXHP,myid)
		msp=GetV(V_MAXSP,myid)
		if (mhp > 250 and mhp < 333 and msp >= 200 and 250 >= msp ) then -- it's an archer 1
			return ARCHER01
		elseif (mhp == 7513 and msp == 201) then 
			TraceAI("Newly summoned Pengineer")
			return PENGINEER
		elseif (mhp >= 4000 and 5000 >= mhp and msp >= 50 and msp < 65 ) then -- looks like a kitty!
			TraceAI("Wild Rose")
			return WILDROSE
		elseif (mhp >= 7513 and 8500 >= mhp and msp >= 201 and msp < 250 ) then 
			TraceAI("Pengineer")
			return PENGINEER
		elseif (mhp >= 7200 and 7512 >= mhp and msp >= 200 and msp < 250 ) then -- looks like a dopple!
			TraceAI("Dopple Merc")
			return DOPPLEMERC
		elseif (mhp >= 10000 and 13000 >= mhp and msp >= 220 and msp < 300 ) then -- looks like alice!
			TraceAI("Alice")
			return ALICE
		elseif	(mhp >= 6100 and 7200 >= mhp and msp >= 180 and msp < 250 ) then -- looks like a mimic!
			TraceAI("Mimic")
			return MIMIC
		elseif (mhp >= 7500 and 9500 >= mhp and msp >= 180 and msp < 220 ) then -- looks like a disguise!
			TraceAI("Disguise")
			return DISGUISE
		elseif (mhp >= 7000 and 8500 >= mhp and msp >= 250 and msp < 320 ) then -- looks like a male GM mercenary!
			TraceAI("GM Mercenary")
			return GMMALE
		elseif (mhp > 12300 and 14500 >= mhp and msp >= 450 and msp < 600 ) then -- Nice stats, must be one of them overpowerd isis mercs
			TraceAI("ISIS Mercenary")
			return ISIS
		else
			TraceAI("Unknown merc type. Contact the developer ASAP.") 
			return UNKNOWNMER
		end
	end
end

function IsNotKS(myid,target)
	--TraceAI("Checking for KS:"..target)
	local targettarget=GetV(V_TARGET,target)
	local motion=GetV(V_MOTION,target)
	local tactks=GetTact(TACT_KS,target)
	if (target==MyEnemy and BypassKSProtect==1) then --If owner has told homun to attack explicity, let it.
		return 1
	end
	if target==nil then
		TraceAI("IsNotKS")
	end
	if (IsPlayer(target)==1) then
		--TraceAI("PVP - not KS")
		return 1
	elseif (IsFriend(targettarget)==1 or targettarget==myid) then
		--TraceAI("Not KS - "..target.." fighting friend: "..targettarget)
		return 1
	elseif ((tactks==KS_POLITE or DoNotAttackMoving ==1) and motion==MOTION_MOVE) then
		return 0
	elseif (tactks==KS_ALWAYS) then
		--TraceAI("It's an FFA monster, not a KS")
		return 1
	elseif targettarget > 0 and IsMonster(targettarget)~=1 and Actors[targettarget]==1 then
		--TraceAI("Is KS - "..target.." attacking player "..targettarget.." motion "..motion)
		return 0
	else
		--TraceAI("Not Targeted - seeing if anyone is targeting it")
		local actors = GetActors()
		for i,v in ipairs(actors) do
			if (IsMonster(v)~=1 and IsFriendOrSelf(v)==0) then
				if (GetV(V_TARGET,v) == target and (v > 100000 or KSMercHomun ~=1)) then
					--TraceAI("Is KS - "..target.." is targeted by "..v)
					return 0
				end
			end
		end
	--TraceAI("Not KS - "..target.." is not targeted by any other player.")
	return 1
	end
end
function GetKSReason(myid,target)
	--TraceAI("Checking for KS:"..target)
	local targettarget=GetV(V_TARGET,target)
	local motion=GetV(V_MOTION,target)
	local tactks=GetTact(TACT_KS,target)
	if (target==MyEnemy and BypassKSProtect==1) then --If owner has told homun to attack explicity, let it.
		return "KS Protect bypassed"
	end
	if target==nil then
		TraceAI("IsNotKS")
	end
	if (IsPlayer(target)==1) then
		--TraceAI("PVP - not KS")
		return "PVP, not KS"
	elseif ((tactks==KS_POLITE or DoNotAttackMoving ==1) and motion==MOTION_MOVE) then
		return "KS polite aka DoNotAttackMoving"
	elseif (IsFriend(targettarget)==1 or targettarget==myid) then
		--TraceAI("Not KS - "..target.." fighting friend: "..targettarget)
		return "Not KS - enemy attacking "..targettarget
	elseif (tactks==KS_ALWAYS) then
		--TraceAI("It's an FFA monster, not a KS")
		return "FFA monster"
	elseif targettarget > 0 and IsMonster(targettarget)~=1 and Actors[targettarget]==1 then
		--TraceAI("Is KS - "..target.." attacking player "..targettarget.." motion "..motion)
		if MyFriends[targettarget]~=nil then
			return "KS - enemy is attacking "..targettarget.." MyFriends: "..MyFriends[targettarget]
		else
			return "KS - enemy is attacking "..targettarget
		end
	else
		--TraceAI("Not Targeted - seeing if anyone is targeting it")
		local actors = GetActors()
		for i,v in ipairs(actors) do
			if (IsMonster(v)~=1 and IsFriendOrSelf(v)==0) then
				if (GetV(V_TARGET,v) == target and (v > 100000 or KSMercHomun ~=1)) then
					--TraceAI("Is KS - "..target.." is targeted by "..v)
					if MyFriends[v]~=nil then
						return "KS - enemy attacked by "..v.." MyFriends: "..MyFriends[v]
					else
						return "KS - enemy attacked by "..v
					end
				end
			end
		end
	--TraceAI("Not KS - "..target.." is not targeted by any other player.")
	return "not KS, not targeted by anything"
	end
end
--########################
--### HP/SP % functions###
--########################

function HPPercent(id)
	local maxHP=GetV(V_MAXHP,id)
	local curHP=GetV(V_HP,id)
	local percHP=100*curHP/maxHP
	return percHP
end

function SPPercent(id)
	local maxSP=GetV(V_MAXSP,id)
	local curSP=GetV(V_SP,id)
	local percSP=100*curSP/maxSP
	return percSP
end

--########################
--### Spacial functions###
--########################
OldMove=Move

function Move(myid,x,y)
	local ox,oy = GetV(V_POSITION,myid)
	local dis = GetDistance(ox,oy,x,y)
	local dis2owner=GetDistanceAPR(OwnerID,x,y)
	local newx,newy=x,y
	if dis2owner > 14 then 
		logappend("AAI_ERROR","Attempt to move to location "..x..","..y.." which is "..dis2owner.." cells from owner, call disregarded")
		return 
	elseif dis > 15  then
		--factor = 14/dis
		local factor=0.5+((math.random(3)-2)*0.1)
		if dis > 25 then 
			factor =0.4+((math.random(3)-2)*0.1)
		end
		local dx,dy = x-ox,y-oy
		if math.random(2)==1 then
			dx=math.ceil(factor*dx)
		else 
			dx=math.floor(factor*dx)
		end
		if math.random(2)==1 then
			dy=math.ceil(factor*dy)
		else 
			dy=math.floor(factor*dy)
		end
		if _VERSION=="Lua 5.1" then 
			while TakenCells[(x+dx).."_"..(y+dy)] do
				factor=factor-0.08
				if math.random(2)==1 then
					dx=math.ceil(factor*(x-ox))
				else 
					dx=math.floor(factor*(x-ox))
				end
				if math.random(2)==1 then
					dy=math.ceil(factor*(y-oy))
				else 
					dy=math.floor(factor*(y-oy))
				end
			end
		end
		if x==MyDestX and y==MyDestY then
			newx=ox+dx
			newy=oy+dy
			MyDestX,MyDestY=newx,newy
			TraceAI("MOVE: Attempt to move more than 15 cells, destination + MyDest adjusted: "..x..","..y.." "..dis.." "..factor.."new: "..newx..","..newy)
		else
			newx=ox+dx
			newy=oy+dy
			TraceAI("MOVE: Attempt to move more than 15 cells, destination adjusted: "..x..","..y.." "..dis.." "..factor.."new: "..newx..","..newy)
		end
	end
	if (LagReduction and LagReduction ~=0) then
		modtwROMoveX,modtwROMoveY=x,y
		return
	else 
		return OldMove(myid,newx,newy)
	end
end


OldSkillObject=SkillObject
function SkillObject(myid,lvl,skill,target)
	if skill==8041 or skill==8043 or skill==8020 or skill==8025 then
		logappend("AAI_ERROR","Attempted to use skill "..SkillInfo[skill][1].." improperly. Check for corrupt H_SkillInfo or badly behaved addon")
	else
		if (LagReduction and LagReduction ~=0) then
			modtwROSkillObjectID=skill
			modtwROSkillObjectLV=lvl
			modtwROSkillObjectTarg=target
		else
			return OldSkillObject(myid,lvl,skill,target)
		end
	end
end

OldAttack=Attack
function Attack(myid,target)
	if SuperPassive==1 then
		TraceAI("Notice: Attack() called while in SuperPassive. MyEnemy "..MyEnemy.." MyState "..MyState)
	end
	if (LagReduction and LagReduction ~=0) then
		modtwROAttackTarget=target
	else
		return OldAttack(myid,target)
	end
end

OldSkillGround=SkillGround
function SkillGround(myid,lvl,skill,x,y)
	if (LagReduction and LagReduction ~=0) then
		modtwROSkillGroundX=x
		modtwROSkillGroundY=y
		modtwROSkillGroundID=skill
		modtwROSkillGroundLV=lvl
	else
		return OldSkillGround(myid,lvl,skill,x,y)
	end
end

function	GetDistance (x1,y1,x2,y2)
	return math.floor(math.sqrt((x1-x2)^2+(y1-y2)^2))
end

function	GetDistance2 (id1, id2)
	local x1, y1 = GetV (V_POSITION,id1)
	local x2, y2 = GetV (V_POSITION,id2)
	if (x1 == -1 or x2 == -1) then
		return -1
	end
	return GetDistance (x1,y1,x2,y2)
end


function GetDistanceRect(id1,id2)
	local x1, y1 = GetV (V_POSITION,id1)
	local x2, y2 = GetV (V_POSITION,id2)
	if (x1 == -1 or x2 == -1) then
		return 999
	end
	local x=x1-x2
	local y=y1-y2
	return math.max(math.abs(x),math.abs(y))
end

function GetDistance3(id1,id2) 
	local x1, y1 = GetV (V_POSITION,id1)
	local x2, y2 = GetV (V_POSITION,id2)
	if (x1 == -1 or x2 == -1) then
		return -1
	end
	return math.sqrt((x1-x2)^2+(y1-y2)^2)
end

function GetAggroDist()
	if OwnerPosX[10]==OwnerPosX[1] and OwnerPosY[10]==OwnerPosY[1] then
		return StationaryAggroDist
	else
		return MobileAggroDist
	end
end

function GetMoveBounds()
	if OwnerPosX[10]==OwnerPosX[1] and OwnerPosY[10]==OwnerPosY[1] then
		return StationaryMoveBounds
	else
		return MobileMoveBounds
	end
end
function IsToRight(id1,id2)
	local x1,y1=GetV(V_POSITION,id1)
	local x2,y2=GetV(V_POSITION,id2)
	if (x1+1==x2 and y1==y2) then
		return 1
	else
		return 0
	end
end

function	GetDistanceP (x1,y1,x2,y2)
	if (x1 == -1 or x2 == -1) then
		return 999
	end
	return math.floor(math.sqrt((x1-x2)^2+(y1-y2)^2))
end

-- GetV(V_POSITION, id), read once per actor per AI() call: actors only move between ticks.
-- AI() empties ActorPosCache at the start of every tick.
ActorPosCache = {}
function GetPos(id)
	local p = ActorPosCache[id]
	if p == nil then
		local x, y = GetV(V_POSITION, id)
		p = {x, y}
		ActorPosCache[id] = p
	end
	return p[1], p[2]
end

function	GetDistanceA (id1, id2)
	local x1, y1 = GetPos(id1)
	local x2, y2 = GetPos(id2)
	if (x1 == -1 or x2 == -1) then
		return 999
	end
	return GetDistance (x1,y1,x2,y2)
end

function	GetDistanceAP (id1, x2,y2)
	local x1, y1 = GetPos(id1)
	if (x1 == -1 or x2 == -1) then
		return 999
	end
	return GetDistance (x1,y1,x2,y2)
end

function	GetDistancePR (x1,y1,x2,y2)
	if (x1 == -1 or x2 == -1) then
		return 999
	end
	return math.max(math.abs(x1-x2),math.abs(y1-y2))
end

function GetDistanceAR(id1,id2)
	local x1, y1 = GetPos(id1)
	local x2, y2 = GetPos(id2)
	if (x1 == -1 or x2 == -1) then
		return 999
	end
	return math.max(math.abs(x1-x2),math.abs(y1-y2))
end

function	GetDistanceAPR (id1, x2,y2)
	local x1, y1 = GetPos(id1)
	if (x1 == -1 or x2 == -1) then
		return 999
	end
	return math.max(math.abs(x1-x2),math.abs(y1-y2))
end






function BetterMoveToOwner(myid,range)
	if (range==nil) then
		range=1
	end
	local x,y = GetV(V_POSITION,myid)
	local ox,oy = GetV(V_POSITION,OwnerID)
	local destx,desty=0,0
	if (x > ox+range) then
		destx=ox+range
	elseif (x < ox - range) then
		destx=ox-range
	else
		destx=x
	end
	if (y > oy+range) then
		desty=oy+range
	elseif (y < oy - range) then
		desty=oy-range
	else
		desty=y
	end
	MyDestX,MyDestY=destx,desty
	Move(myid,MyDestX,MyDestY)
	return
end
function BetterMoveToOwnerXY(myid,range)
	if (range==nil) then
		range=1
	end
	local x,y = GetV(V_POSITION,myid)
	local ox,oy = GetV(V_POSITION,OwnerID)
	local destx,desty=0,0
	if (x > ox+range) then
		destx=ox+range
	elseif (x < ox - range) then
		destx=ox-range
	else
		destx=x
	end
	if (y > oy+range) then
		desty=oy+range
	elseif (y < oy - range) then
		desty=oy-range
	else
		desty=y
	end
	return destx,desty
end
function BetterMoveToLoc(myid,range,ox,oy)
	if (range==nil) then
		range=1
	end
	local x,y = GetV(V_POSITION,myid)
	--local ox,oy = GetV(V_POSITION,OwnerID)
	local destx,desty=0,0
	if (x > ox+range) then
		destx=ox+range
	elseif (x < ox - range) then
		destx=ox-range
	else
		destx=x
	end
	if (y > oy+range) then
		desty=oy+range
	elseif (y < oy - range) then
		desty=oy-range
	else
		desty=y
	end
	MyDestX,MyDestY=destx,desty
	Move(myid,MyDestX,MyDestY)
	return
end
function BetterMoveToLocXY(myid,range,ox,oy)
	if (range==nil) then
		range=1
	end
	local x,y = GetV(V_POSITION,myid)
	--local ox,oy = GetV(V_POSITION,OwnerID)
	local destx,desty=0,0
	if (x > ox+range) then
		destx=ox+range
	elseif (x < ox - range) then
		destx=ox-range
	else
		destx=x
	end
	if (y > oy+range) then
		desty=oy+range
	elseif (y < oy - range) then
		desty=oy-range
	else
		desty=y
	end
	
	return destx,desty
end

function	GetOwnerPosition (id)
	return GetV (V_POSITION,GetV(V_OWNER,id))
end

function	GetDistanceFromOwner (id)
	local x1, y1 = GetOwnerPosition (id)
	local x2, y2 = GetV (V_POSITION,id)
	if (x1 == -1 or x2 == -1) then
		return -1
	end
	return GetDistance (x1,y1,x2,y2)
end

function	IsOutOfSight (id1,id2)
	local x1,y1 = GetV (V_POSITION,id1)
	local x2,y2 = GetV (V_POSITION,id2)
	if (x1 == -1 or x2 == -1 or id1==-1 or id2==-1) then
		return true
	end
	local d = GetDistancePR(x1,y1,x2,y2)
	if d > 20 then
		return true
	else
		return false
	end
end


function	IsInAttackSight (myid,target,skill,level)
	if (skill==nil) then
		skill=MySkill
		level=MySkillLevel
	elseif (level==nil) then
		logappend("AAI_ERROR","IsInAttackSight called with nil level but skill is not nil. State:"..MyState.." skill "..skill)
	end
	local x1,y1 = GetV (V_POSITION,myid)
	local x2,y2 = GetV (V_POSITION,target)
	if (x1 == -1 or x2 == -1) then
		return false
	end
	local d		= GetDistance (x1,y1,x2,y2)
	local a     = AttackRange(myid,skill,level)
	if a >= d then
		return true;
	else
		return false;
	end
end



function AttackRange(myid,skill,level)
	if (skill==nil or level== nil) then
		if skill==nil or level==nil then
			logappend("AAI_ERROR","AttackRange called with invalid arguments State:"..MyState.." MySkill "..MySkill.." MySkillLevel"..MySkillLevel.." skill: "..formatval(skill).." level: "..formatval(level))
		end
		skill=MySkill
		level=MySkillLevel
	end
	local a     = 0
	if (skill == 0) then
		mertype=GetMerType(myid)
		if (mertype==nil) then
			mertype=-1
		end
		if (mertype == -1) then
			a     = 1
		elseif (mertype > 10 and mertype < 21) then
			a     = 2 --spear mercenary
		else
			a     = GetV(V_ATTACKRANGE,myid)
		end
	--elseif (skill==MS_BOWLINGBASH) then
	--	a	= 1
	else
		a = GetSkillInfo(skill,2,level)
		if a==nil then
			TraceAI("AttackRange() cannot get range for skill "..formatval(skill).." level "..formatval(level))
		end
	end
	if level == nil then
		logappend("AAI_ERROR","AttackRange called with nil level, skill = "..skill.." "..MyState)
	end
	TraceAI("Attack Sight - skill:"..skill.."level: "..level.." Range:"..a) 
	return a
end

function Closer(id,ox,oy)
	local x,y=GetV(V_POSITION,id)
	local newx,newy=0,0
	if (ox==x) then
		newx=x
	elseif (ox > x) then
		newx=ox-1
	else
		newx=ox+1
	end
	if (oy==y) then
		newy=y
	elseif (oy > y) then
		newy=oy-1
	else
		newy=oy+1
	end
	return newx,newy
end




function GetStandPoint(myid,target,skill,level,alt)
	local r=AttackRange(myid,skill,level)
	if GetDistanceA(myid,target) <= r then
		logappend("AAI_ERROR","GetStandPoint called while within range")
		return GetV(V_POSITION,MyID)
	end
	local tx,ty=GetV(V_POSITION,target)
	local x,y=-1,-1
	if _VERSION=="Lua 5.1" and myid==MyID and (alt==nil or alt==0) then
		x,y=GetV(V_POSITION_APPLY_SKILLATTACKRANGE,target,skill,level)
	end
	if skill==0 or skill==nil or (tx==x and ty==y) or (tx < 1 or ty < 1) or math.floor(GetDistanceAP(target,x,y)) > r then -- Use normal closest call, since V_POSITION_APPLY_SKILLATTACKRANGE doesn't work for non-skills, or has given us clearly bogus standpoint for skill.
		x,y=Closest(myid,tx,ty,r,alt)
	end
	return AdjustStandPoint(x,y,tx,ty,r,alt)
end

function AdjustStandPoint (x,y,tx,ty,r,alt)
	if _VERSION=="Lua 5.1" then
		-- Now we need to make sure x and y are not on occupied cells, because of the changes to the client
		local bounds=GetMoveBounds()
		local logstring=""
		if (alt==nil) then
			logstring=logstring.."x,y"..x..","..y.." tx,ty"..tx..","..ty.." r "..r.."alt nil bounds "..bounds.." | "
		else 
			logstring=logstring.."x,y"..x..","..y.." tx,ty"..tx..","..ty.." r "..r.."alt "..alt.." bounds "..bounds.." | "
		end
		local i=0
		
		while (TakenCells[x.."_"..y] or GetDistanceAPR(OwnerID,x,y) > bounds) do
			i=i+1
			if (TakenCells[x.."_"..y]) then
				logstring=logstring.."i="..i.."tc="..TakenCells[x.."_"..y].."d ".. GetDistanceAPR(OwnerID,x,y).." :"
			else 
				logstring=logstring.."i="..i.."tc= nil d ".. GetDistanceAPR(OwnerID,x,y).." :"
			end
			if r==1 then
				if alt==nil or alt==0 then
					x,y=AdjustCW(x,y,tx,ty)
					logstring=logstring.."CW "..x..","..y
				elseif alt==1 then
					x,y=AdjustCCW(x,y,tx,ty)
					logstring=logstring.."CCW "..x..","..y
				else --alt=2
					x,y=AdjustOpp(x,y,tx,ty)
					logstring=logstring.."OPP "..x..","..y
					alt=0
				end
				if i > 7 then
					logappend("AAI_CLOSEST","AdjustStandPoint() FAILED  logstring="..logstring)

		TraceAI("AdjustStandPoint() FAILED logstring="..logstring)
					return -1,-1
				end
			else
				if alt==nil or alt==0 then
					x,y=AdjustCWr(x,y,tx,ty,r)
					logstring=logstring.."CWr "..x..","..y
				elseif alt==1 then
					x,y=AdjustCCWr(x,y,tx,ty,r)
					logstring=logstring.."CCWr "..x..","..y
				else --alt=2
					x,y=x*-1,y*-1
					logstring=logstring.."Opp R "..x..","..y
					alt=0
				end
				if i > 7 then
					logappend("AAI_CLOSEST","AdjustStandPoint() FAILED  logstring="..logstring)
					return -1,-1
				end
			end
		end
		logappend("AAI_CLOSEST","AdjustStandPoint() returned "..x.." , "..y.." logstring="..logstring)
	end
	return x,y
end

function Closest(myid,ox,oy,range,alt)
	local x,y=GetV(V_POSITION,myid)
	local dx,dy=ox-x,oy-y
	local newx,newy=0,0
	local logstring="x,y"..x..","..y.." ox,oy"..ox..","..oy
	if range > 1 then
		logstring=logstring.."range > 1 "
		local dist=math.sqrt(dx^2+dy^2)
		if (dist < range) then
			newx,newy=x,y
			logstring=logstring.."dist < range new"..newx..","..newy.." "
		else
			local factor=range/dist
			local xoff=(dx-absceil(dx*factor))
			local yoff=(dy-absceil(dy*factor))
			newx,newy=x+xoff,y+yoff
			logstring=logstring.."dist > range new" ..newx..","..newy.." factor:"..factor.." "
		end
		-- This routine tries to find an adjacent cell to the closest cell that will be close enough - to address unwalkable cells near where retainer wants to walk (think: pebbles in thors)
		
		if (alt ~= nil) then
			logstring=logstring.."alt not nil"..alt.." "
			if (alt==1) then
				if (x > ox) then
					newx=newx-1
				elseif (x < ox) then
					newx=newx+1
				else
					alt=2
				end
			end
			if (alt==2) then
				if (y > oy) then
					newy=newy-1
				elseif (y < oy) then
					newy=newy+1
				else
					alt=3
				end
			end
		
			--if (alt==4) then -- This seems to cause more problems than it solves, set to 4 to prevent from running
			--	if (x > ox) then
			--		newx=newx-1
			--		newy=newy+1
			--	elseif (x < ox) then
			--		newx=newx+1
			--		newy=newy+1
			--	elseif (y > oy) then
			--		newy=newy-1
			--		newx=newx+1
			--	else 	
			--		newy=newy+1
			--		newx=newx+1
			--	end
			--end
			logstring=logstring.."end of alt sequence new"..newx..","..newy.." alt "..alt.." "
		end
		--xxx,yyy=GetV(V_POSITION,target)
		--TraceAI("alt is: "..GetDistance(xxx,yyy,newx,newy).." "..xxx..","..yyy.." "..newx..","..newy.."MyPos: "..x..","..y) 	
	else -- simplified, better closest point for melee. 
		logstring=logstring.."range = 1 d"..dx..","..dy.." o"..ox..","..oy.." "
		if dx > 0 then
			newx=ox-1
		elseif dx < 0 then
			newx=ox+1
		else -- dx = 0
			newx=x
		end
		if dy > 0 then
			newy=oy-1
		elseif dy < 0 then
			newy=oy+1
		else -- dy = 0
			newy=y
		end
		logstring=logstring.."new"..newx..","..newy.." "
		if alt~=nil then
			if alt==1 then
				newx,newy=AdjustCW(newx,newy,ox,oy)
				logstring=logstring.."adjust cw"..newx..","..newy.." "
			else
				newx,newy=AdjustCCW(newx,newy,ox,oy)
				logstring=logstring.."adjust ccw"..newx..","..newy.." "
			end
		end
	end
	local logalt="nil"
	if alt~=nil then
		logalt=alt
	end
	if newx<=0 or newy<=0 then 
		logappend("AAI_ERROR","Closest() Failure!"..newx.." , "..newy.." alt ="..logalt.." logstring="..logstring)
	end
	logappend("AAI_CLOSEST","Closest() returned "..newx.." , "..newy.." alt ="..logalt.." logstring="..logstring)
	return newx,newy
end

function AdjustCW(x,y,ox,oy)
	local newx,newy,dy,dx
	if x==ox then
		if y==oy then --stacked!? Unstack!
			newx,newy=ox+1,oy
		else
			dy=y-oy
			newx=x+absunit(dy)
			newy=y
		end
	elseif y==oy then
		dx=x-ox
		newy=y-absunit(dx)
		newx=x
	elseif y>oy then
		if x>ox then
			newy=y-1
			newx=x
		else
			newy=y
			newx=x+1
		end
	else -- y<oy
		if x>ox then
			newx=x-1
			newy=y
		else
			newx=x
			newy=y+1
		end
	end
	return newx,newy
end
function AdjustCCW(x,y,ox,oy)
	local newx,newy,dx,dy
	if x==ox then
		if y==oy then --stacked!? Unstack!
			newx,newy=ox-1,oy
		else
			dy=y-oy
			newx=x-absunit(dy)
			newy=y
		end
	elseif y==oy then
		dx=x-ox
		newy=y+absunit(dx)
		newx=x
	elseif y>oy then
		if x>ox then
			newy=y
			newx=x-1
		else
			newy=y-1
			newx=x
		end
	else -- y<oy
		if x>ox then
			newx=x
			newy=y+1
		else
			newx=x+1
			newy=y
		end
	end
	return newx,newy
end

function AdjustCWr(x,y,ox,oy,r)
	if r==1 then
		return AdjustCW(x,y,ox,oy)
	end
	local newx,newy,dy,dx
	dx,dy=x-ox,y-oy
	if math.abs(dx) == math.abs(dy)  then --we're on a diagonal, this could be a problem, so let's get off it!
		if dx < 0 then
			if dy < 0 then
				newx=x+1
				newy=y
			else
				newx=x
				newy=y-1
			end
		elseif dx > 0 then
			if dy < 0 then
				newy=y+1
				newx=x
			else
				newx=x-1
				newy=y
			end
		else --stacked, which should NEVER HAPPEN but has somehow happened anyway
			newx=x+1
			newy=y+1
			logappend("AAI_ERROR","AdjustCWr called with x,y=ox,oy - this can't happen!")
		end
	else
		if math.abs(dx) < math.abs(dy) then --north or south quadrant
			if dy < 0 then -- south quadrant
				newx=x+1
				newy=y+1
			else --north
				newx=x-1
				newy=y-1
			end
		else --east or west quadrant
			if dx < 0 then --west
				newy=y-1
				newx=x+1
			else
				newx=x-1
				newy=y+1
			end
		end
	end

	if (newx-ox==0 and math.abs(newy-oy)==1) or (newy-oy==0 and math.abs(newx-ox)==1) then
		newx,newy=newx*2,newy*2
	end
	return newx,newy
end

function AdjustCCWr(x,y,ox,oy,r)
	if r==1 then
		return AdjustCCW(x,y,ox,oy)
	end
	local newx,newy,dy,dx
	dx,dy=x-ox,y-oy
	if math.abs(dx) == math.abs(dy)  then --we're on a diagonal, this could be a problem, so let's get off it!
		if dx < 0 then
			if dy < 0 then
				newx=x
				newy=y+1
			else
				newx=x+1
				newy=y
			end
		elseif dx > 0 then
			if dy < 0 then
				newy=y
				newx=x-1
			else
				newx=x
				newy=y-1
			end
		else --stacked, which should NEVER HAPPEN but has somehow happened anyway
			newx=x-1
			newy=y-1
			logappend("AAI_ERROR","AdjustCWr called with x,y=ox,oy - this can't happen!")
		end
	else
		if math.abs(dx) < math.abs(dy) then --north or south quadrant
			if dy < 0 then -- south quadrant
				newx=x-1
				newy=y+1
			else --north
				newx=x+1
				newy=y-1
			end
		else --east or west quadrant
			if dx < 0 then --west
				newy=y+1
				newx=x+1
			else
				newx=x-1
				newy=y-1
			end
		end
	end
	if (newx-ox==0 and math.abs(newy-oy)==1) or (newy-oy==0 and math.abs(newx-ox)==1) then
		newx,newy=newx*2,newy*2
	end
	return newx,newy
end

function AdjustOpp(x,y,ox,oy)
	local dx=ox-x
	local dy=oy-y
	return x+2*dx,y+2*dy
end
function ClosestR(myid,target,range,alt)
	local ox,oy=GetV(V_POSITION,target)
	return Closest(myid,ox,oy,range,alt)
end

--###############################   ##   ##
--### LUA 5.1 feature support ###   ##   ##
--###############################   ##   ##


function DiagonalDist(distance)
	return math.floor(math.sqrt(distance*distance))
end

function GetDanceCell(x,y,enemy)
	local enemyx,enemyy=GetV(V_POSITION,enemy)
	if IsInAttackSight(MyID,enemy) == false then --Why is this being called?! 
		logappend("AAI_ERROR","GetDanceCell() called when enemy out of range!" )
		x,y=Closest(MyID,enemyx,enemyy,1)
		return x,y
	end
	local t = math.random(2)
	if t == 1 then
		return AdjustCW(x,y,enemyx,enemyy)
	elseif t == 2 then
		return AdjustCCW(x,y,enemyx,enemyy)
	else 
		return AdjustOpp(x,y,enemyx,enemyy)
	end
end

--[[ --disabled for a simpler dance alog, because you can't dance any ranged attacks at the moment, so optimize for range=1

function GetDanceCell(x,y,enemy)
	distance = GetDistance2(MyID,enemy)
	ex,ey=GetV(V_POSITION,enemy)
	s=(((math.random(2)-1)*2)-1)
	t=(((math.random(2)-1)*2)-1)
	if (GetDistance(x-s,y+t,ex,ey) == distance) then
		return x-s,y+t
	elseif (GetDistance(x,y+t,ex,ey) == distance) then
		return x,y+t
	elseif (GetDistance(x+s,y+t,ex,ey) == distance) then
		return x+s,y+t
	elseif (GetDistance(x-s,y,ex,ey) == distance) then
		return x-s,y
	elseif (GetDistance(x+s,y,ex,ey) == distance) then
		return x+s,y
	elseif (GetDistance(x-s,y-t,ex,ey) == distance) then
		return x-s,y-t
	elseif (GetDistance(x,y-t,ex,ey) == distance) then
		return x,y-t
	else
		return x+s,y-t
	end
	
end
]]--]]
--#########################
--### GetSkill functions###
--#########################

function GetSAtkSkill(myid)
	local skill = 0
	local level = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if htype > 47 then -- it's a Homun S
			if htype==EIRA and UseEiraEraseCutter==1 then
				skill=MH_ERASER_CUTTER
				if EiraEraseCutterLevel==nil then
					level=SkillList[EIRA][MH_ERASER_CUTTER]
				else
					level=EiraEraseCutterLevel
				end
			elseif htype==BAYERI and UseBayeriStahlHorn==1 then
				skill=MH_STAHL_HORN
				if BayeriStahlHornLevel==nil then
					level=SkillList[BAYERI][MH_STAHL_HORN]
				else
					level=BayeriStahlHornLevel
				end
			elseif htype==SERA and UseSeraParalyze==1 then
				skill=MH_NEEDLE_OF_PARALYZE
				if SeraParalyzeLevel==nil then
					level=SkillList[SERA][MH_NEEDLE_OF_PARALYZE]
				else
					level=SeraParalyzeLevel
				end
			elseif htype==ELEANOR and UseEleanorSonicClaw==1 and EleanorMode==FIGHTING_MODE then
				skill=MH_SONIC_CRAW
				if EleanorSonicClawLevel==nil then
					level=SkillList[ELEANOR][MH_SONIC_CRAW]
				else
					level=EleanorSonicClawLevel
				end
			elseif htype==ELEANOR and UseEleanorTinderBreaker==1 and EleanorMode==GRAPPLING_MODE then
				skill=MH_TINDER_BREAKER
				if EleanorTinderBreakerLevel==nil then
					level=SkillList[ELEANOR][MH_TINDER_BREAKER]
				else
					level=EleanorTinderBreakerLevel
				end
			end
		end
		if level ~=0 and IsSkillReady(skill) then
			return skill,level
		end
	end
	return 0,0
end

function GetComboSkill(myid)
	local skill = 0
	local level = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if htype==ELEANOR then
			if EleanorMode==FIGHTING_MODE then
				-- Sonic Claw already used one sphere, so the follow-up needs one less than the start
				if ComboSCTimeout > GetTick() and MySpheres >= AutoComboSpheres - 1 and IsSkillReady(MH_SILVERVEIN_RUSH) then
					skill=MH_SILVERVEIN_RUSH
					if EleanorSilverveinLevel==nil then
						level=SkillList[ELEANOR][MH_SILVERVEIN_RUSH]
					else
						level=EleanorSilverveinLevel
					end
				elseif ComboSVTimeout > GetTick() and IsSkillReady(MH_MIDNIGHT_FRENZY) then
					skill=MH_MIDNIGHT_FRENZY
					if EleanorMidnightLevel==nil then
						level=SkillList[ELEANOR][MH_MIDNIGHT_FRENZY]
					else
						level=EleanorMidnightLevel
					end
				end
			end
		end
		if level ~=0 then
			return skill,level
		end
	end
	return 0,0
end

function GetGrappleSkill(myid)
	local skill = 0
	local level = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if htype==ELEANOR and EleanorMode==GRAPPLING_MODE then
			-- AutoComboSpheres decides when to start the chain (Tinder Breaker); a started chain only
			-- needs the spheres the next skill costs (CBC 1, EQC 2), or the server refuses it
			if ComboSCTimeout > GetTick() and UseEleanorCBC == 1 and IsSkillReady(MH_CBC) then
				skill=MH_CBC
				if EleanorCBCLevel==nil then
					level=SkillList[ELEANOR][MH_CBC]
				else
					level=EleanorCBCLevel
				end
			elseif ComboSVTimeout > GetTick() and UseEleanorEQC == 1 and IsSkillReady(MH_EQC) then
				skill=MH_EQC
				if EleanorEQCLevel==nil then
					level=SkillList[ELEANOR][MH_EQC]
				else
					level=EleanorEQCLevel
				end
			elseif MySpheres >= AutoComboSpheres and UseEleanorTinderBreaker == 1 and IsSkillReady(MH_TINDER_BREAKER) then
				skill=MH_TINDER_BREAKER
				if EleanorTinderBreakerLevel==nil then
					level=SkillList[ELEANOR][MH_TINDER_BREAKER]
				else
					level=EleanorTinderBreakerLevel
				end
			end
		end
		if level ~=0 then
			return skill,level
		end
	end
	return 0,0
end

function GetAtkSkill(myid)
	local skill = 0
	local level = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if htype < 17 then
			homuntype=modulo(MyHomunType,4)
		else
			homuntype=modulo(DetectedOldHomunType,4)
		end
		if (homuntype==0) then -- It's a vanil!
			skill=HVAN_CAPRICE
			if GetTick() < AutoSkillCooldown[skill] then
				level=0
				skill=0
			elseif (VanCapriceLevel==nil) then
				level=SkillList[VANILMIRTH][HVAN_CAPRICE]
			else
				level=VanCapriceLevel
			end
		elseif	(homuntype==3) then -- It's a filir!
			skill=HFLI_MOON
			if GetTick() < AutoSkillCooldown[skill] then
				level=0
				skill=0
			elseif (FilirMoonlightLevel~=nil) then
				level=FilirMoonlightLevel
			elseif (FilerMoonlightLevel==nil) then
				level=SkillList[FILIR][HFLI_MOON]
			else
				level=FilerMoonlightLevel
			end
		end		
		if level ~=0 then
			return skill,level
		end
	else
		for i,v in ipairs(AtkSkillList) do
			level = SkillList[MercType][v]
			if level ~= nil then
				skill=v
				return skill,level
			end
		end
	end
	return 0,0
end

function GetPushbackSkill(myid)
	if (IsHomun(myid)==1) then
		return 0,0
	else
		for i,v in ipairs(PushSkillList) do
			level = SkillList[MercType][v]
			if level ~= nil then
				skill=v
				return skill,level
			end
		end
	end
	return 0,0
end


function GetDebuffSkill(myid)
	if (IsHomun(myid)==1) then
		if MyHomunType==EIRA and UseEiraSilentBreeze==1 and IsSkillReady(MH_SILENT_BREEZE) then
			skill=MH_SILENT_BREEZE
			if EiraSilentBreezeLevel==nil then
				level=SkillList[EIRA][MH_SILENT_BREEZE]
			else
				level=EiraSilentBreezeLevel
			end
			return skill,level
		elseif MyHomunType==DIETER and UseDieterVolcanicAsh==1 then
			skill=MH_VOLCANIC_ASH
			level=SkillList[DIETER][MH_VOLCANIC_ASH]
			local t = GetTick()
			if (AshTimeout[1] < t or AshTimeout[2] < t or AshTimeout[3] < t) then
				return skill,level
			else 
				return 0,0
			end
		end
	else
		for i,v in ipairs(DebuffSkillList) do
			level = SkillList[MercType][v]
			if level ~= nil then
				skill=v
				return skill,level
			end
		end
	end
	return 0,0
end

function GetMinionSkill(myid)
	local level,skill=0,0
	if (IsHomun(myid)==1) then
		if MyHomunType==SERA and UseSeraCallLegion==1 then
			skill=MH_SUMMON_LEGION
			level=SeraCallLegionLevel or SkillList[SERA][MH_SUMMON_LEGION]
			TraceAI("GetMinionSkill "..skill..level)
			if AutoSkillCooldown[skill]~=nil then
				if GetTick() < AutoSkillCooldown[skill] then -- in cooldown
					level=0
					skill=0
				end
			end
			return skill,level
		end
	end
	return 0,0
end

function GetProvokeSkill(myid)
	if (IsHomun(myid)==1) then
		return 0,0
	else
		level=SkillList[MercType][MER_PROVOKE]
		if level ~=nil then
			return MER_PROVOKE,level
		end
	end
	return 0,0
end

function GetSacrificeSkill(myid)
	level=SkillList[MercType][ML_DEVOTION]
	if level ~=nil then
		return ML_DEVOTION,level
	end
	return 0,0
end

function GetMobSkill(myid)
	local skill = 0
	local level = 0
	if (IsHomun(myid)==1) then
	
		htype=MyHomunType
		if htype <17 then
			skill=0
		else -- it's a homun s
			if htype==EIRA and UseEiraXenoSlasher==1 then
				skill=MH_XENO_SLASHER
				if EiraXenoSlasherLevel==nil then
					level=SkillList[EIRA][MH_XENO_SLASHER]
				else
					level=EiraXenoSlasherLevel
				end
			elseif htype==BAYERI and UseBayeriHailegeStar==1 then
				skill=MH_HEILIGE_STANGE
				if BayeriHailegeStarLevel==nil then
					level=SkillList[BAYERI][MH_HEILIGE_STANGE]
				else
					level=BayeriHailegeStarLevel
				end
			elseif htype==SERA and UseSeraPoisonMist==1 and PoisonMistMode==0 then
				skill=MH_POISON_MIST
				if SeraPoisonMistLevel==nil then
					level=SkillList[SERA][MH_POISON_MIST]
				else
					level=SeraPoisonMistLevel
				end
			elseif htype==DIETER and UseDieterLavaSlide==1 and LavaSlideMode==0 then
				skill=MH_LAVA_SLIDE
				if DieterLavaSlideLevel==nil then
					level=SkillList[DIETER][MH_LAVA_SLIDE]
				else
					level=DieterLavaSlideLevel
				end
			end 
			if AutoSkillCooldown[skill]~=nil then
				if GetTick() < AutoSkillCooldown[skill] then -- in cooldown
					level=0
					skill=0
				end
			end
		end
		return skill,level
	else -- SO MUCH EASIER WHEN LEVEL ISN'T SELECTABLE!!!!
		for i,v in ipairs(MobSkillList) do
			level = SkillList[MercType][v]
			if level ~= nil then
				skill=v
				return skill,level
			end
		end
	end
	return 0,0
end


function	GetQuickenSkill(myid)
	local level = 0
	local skill = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if htype < 17 then
			homuntype=modulo(MyHomunType,4)
		else
			homuntype=modulo(DetectedOldHomunType,4)
		end
		if (homuntype==1) then -- It's a lif!
			skill=HLIF_CHANGE
			level=SkillList[LIF][HLIF_CHANGE]
		elseif	(homuntype==3) then -- It's a filer!
			skill=HFLI_FLEET
			if (FilirFlitLevel==nil) then
				level=SkillList[FILIR][HFLI_FLEET]
			else
				level=FilirFlitLevel
			end
		elseif  (homuntype==2) and AllowBloodlust == 1 then --it's an amistr
			skill=HAMI_BLOODLUST
			level=SkillList[AMISTR][HAMI_BLOODLUST]
		end
	else
		level=SkillList[MercType][MER_QUICKEN]
		if level ~=nil then
			skill=MER_QUICKEN
		end
	end
	-- keep the skill id: callers read skill==0 as "no buff available" and stop trying for good
	if AutoSkillCooldown[skill]~=nil then
		if GetTick() < AutoSkillCooldown[skill] then -- in cooldown
			level=0
		end
	end
	return skill,level
end

function	GetSOffensiveSkill(myid)
	local level = 0
	local skill = 0
	local skillopt = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if (htype==BAYERI and UseBayeriAngriffModus~=0) then
			skill=MH_ANGRIFFS_MODUS
			level = SkillList[BAYERI][MH_ANGRIFFS_MODUS]
			skillopt=UseBayeriAngriffModus
			-- the server refuses Angriffs Modus while Goldene Ferse is up (and vice versa)
			if UseBayeriGoldenPherze~=0 and SDefensiveTimeout > GetTick() then
				level = 0
			end
		elseif	(htype==DIETER and UseDieterMagmaFlow~=0) then
			skill=MH_MAGMA_FLOW
			level = SkillList[DIETER][MH_MAGMA_FLOW]
			skillopt=UseDieterMagmaFlow
		end
		return skill,level,skillopt
	else
		level=SkillList[MercType][MER_BLESSING]
		if level~=nil then
			skill=MER_BLESSING
		else
			level=0
		end
		return skill,level,UseBlessingSelf
	end
end

function	GetSDefensiveSkill(myid)
	local level = 0
	local skill = 0
	local skillopt=0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if (htype==BAYERI and UseBayeriGoldenPherze~=0) then
			skill=MH_GOLDENE_FERSE
			level = SkillList[BAYERI][MH_GOLDENE_FERSE]
			skillopt=UseBayeriGoldenPherze
			if UseBayeriAngriffModus~=0 and SOffensiveTimeout > GetTick() then
				level = 0
			end
		elseif	(htype==DIETER and UseDieterGraniticArmor~=0) then
			skill=MH_GRANITIC_ARMOR
			level = SkillList[DIETER][MH_GRANITIC_ARMOR]
			skillopt=UseDieterGraniticArmor
		end
		return skill,level,skillopt
	else
		level=SkillList[MercType][MER_KYRIE]
		if level~=nil then
			skill=MER_KYRIE
		else
			level=0
		end
		return skill,level,UseKyrieSelf
	end
	return 0,0,0
end

function	GetSOwnerBuffSkill(myid)
	local level = 0
	local skill = 0
	local skillopt = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if (htype==EIRA and UseEiraOveredBoost~=0) then
			skill=MH_OVERED_BOOST
			level = SkillList[EIRA][MH_OVERED_BOOST]
			skillopt=UseEiraOveredBoost
		elseif	(htype==DIETER and UseDieterPyroclastic~=0) then
			skill=MH_PYROCLASTIC
			if DieterPyroclasticLevel==nil then
				level = SkillList[DIETER][MH_PYROCLASTIC]
			else
				level=DieterPyroclasticLevel
			end
			skillopt=UseDieterPyroclastic
		end
		return skill,level,skillopt
	else
		level=SkillList[MercType][MER_INCAGI]
		if level~=nil then
			skill=MER_INCAGI
		else
			level=0
		end
		return skill,level,UseIncAgiSelf
	end
	return 0,0,0
end

function GetSightOrAoE(myid)
	local level = 0
	local skill = 0
	local skillopt = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if	(htype==DIETER and UseDieterLavaSlide==1 and LavaSlideMode~=0) then
			skill=MH_LAVA_SLIDE
			level = DieterLavaSlideLevel or SkillList[DIETER][MH_LAVA_SLIDE]
			skillopt=LavaSlideMode
		elseif (htype==SERA and PoisonMistMode~=0 and UseSeraPoisonMist==1) then
			skill=MH_POISON_MIST
			level = SeraPoisonMistLevel or SkillList[SERA][MH_POISON_MIST]
			skillopt=PoisonMistMode
		end
	else
		if MercType==2 then
			skill=MER_SIGHT
			level=1
			skillopt=UseAutoSight
		end
	end
	if AutoSkillCooldown[skill]~=nil then
		if GetTick() < AutoSkillCooldown[skill] then -- in cooldown
			level=0
			skill=skill
		end
	end
	-- the AoE is laid at the owner's feet: skip it while the monsters there shrug off its element
	if level ~= 0 and IsHomun(myid)==1 and not AoEElementOK(skill) then
		level=0
	end
	return skill,level,skillopt
end

-- False when every monster near the owner would take less than SkillElementMinRate % from the
-- skill's element (no monster around, or no monster database: true)
function AoEElementOK(skill)
	if UseMobInfo == 0 or MobInfo == nil or SkillElement == nil or SkillElement[skill] == nil then
		return true
	end
	local seen = false
	for k,v in pairs(Targets) do
		if GetDistanceA(OwnerID,k) <= 7 then
			seen = true
			if SkillElementOK(skill,k) then
				return true
			end
		end
	end
	return not seen
end
function	GetGuardSkill(myid)
	local level = 0
	local skill = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if htype < 17 then
			homuntype=modulo(MyHomunType,4)
		else
			homuntype=modulo(DetectedOldHomunType,4)
		end
		if (homuntype==1) then -- It's a lif!
			skill=HLIF_AVOID
			if (LifEscapeLevel==nil) then
				level=SkillList[LIF][HLIF_AVOID]
			else
				level=LifEscapeLevel
			end
		elseif (homuntype==2) then -- It's an amistr!
			skill=HAMI_DEFENCE
			if (AmiBulwarkLevel==nil) then
				level=SkillList[AMISTR][HAMI_DEFENCE]
			else
				level=AmiBulwarkLevel
			end
		elseif (homuntype==3) then -- It's an filir!
			skill=HFLI_SPEED
			if (FilirAccelLevel==nil) then
				level=SkillList[FILIR][HFLI_SPEED]
			else
				level=FilirAccelLevel
			end
		end
		if AutoSkillCooldown[skill]~=nil and GetTick() < AutoSkillCooldown[skill] then
			level=0 -- in cooldown
		end
		return skill,level
	else
		for i,v in ipairs(GuardSkillList) do
			level = SkillList[MercType][v]
			if level ~= nil then
				skill=v
				return skill,level
			end
		end
	end
	return 0,0
end

function	GetOffensiveOwnerSkill(myid)
	local level = 0
	local skill = 0
	local skillopt = 0
	if (IsHomun(myid)==1) then
		return 0,0,0
	else
		level=SkillList[MercType][MER_BLESSING]
		if level~=nil then
			skill=MER_BLESSING
		else
			level=0
		end
		return skill,level,UseBlessingOwner
	end
end

function	GetDefensiveOwnerSkill(myid)
	local level = 0
	local skill = 0
	local skillopt = 0
	if (IsHomun(myid)==1) then
		local htype=MyHomunType
		if htype==SERA and UseSeraPainkiller~=0 then
			skill = MH_PAIN_KILLER
			level = SkillList[htype][skill]
			skillopt = UseSeraPainkiller
		elseif htype==BAYERI and UseBayeriGoldeneTone~=0 then
			skill = MH_GOLDENE_TONE
			level = BayeriGoldeneToneLevel or SkillList[htype][skill]
			skillopt = UseBayeriGoldeneTone
		else
			return 0,0,0
		end
		-- level 0 = in cooldown (Goldene Tone's 120 s reuse outlasts its 30-120 s buff)
		if not IsSkillReady(skill) then
			level = 0
		end
		return skill,level,skillopt
	else
		level = SkillList[MercType][MER_KYRIE]
		if level~=nil then
			skill=MER_KYRIE
		else
			level=0
		end
		skillopt = UseKyrieOwner
		return skill,level,skillopt
	end
	return 0,0,0
end

function	GetOtherOwnerSkill(myid)
	local level = 0
	local skill = 0
	local skillopt = 0
	if (IsHomun(myid)==1) then
		return 0,0,0
	else
		level=SkillList[MercType][MER_INCAGI]
		if level~=nil then
			skill=MER_INCAGI
		else
			level=0
		end
		return skill,level,UseIncAgiOwner
	end
end



function GetHealingSkill(myid)
	local level = 0
	local skill = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if htype < 17 then --if it's not a homun S just run it through modulo. 
			homuntype=modulo(MyHomunType,4)
		else --If it's a homun S, get the OldHomunType
			if htype == EIRA and HealOwnerBreeze == 1 then --Handling for Eira silent breeze
				skill=MH_SILENT_BREEZE
				if not IsSkillReady(skill) then
					level=0
				else
					level=EiraSilentBreezeLevel or SkillList[EIRA][MH_SILENT_BREEZE]
				end
				return skill,level
			end
			homuntype=modulo(DetectedOldHomunType,4)
		end
		if (homuntype==1) then -- It's a lif
			skill=HLIF_HEAL
			if GetTick() < AutoSkillCooldown[skill] then
				level=0
			elseif (LifHealLevel==nil) then
				level=SkillList[LIF][HLIF_HEAL]
			else
				level=LifHealLevel
			end
		elseif homuntype==0 then -- It's a vani
			skill=HVAN_CHAOTIC
			if GetTick() < AutoSkillCooldown[skill] then
				level=0
			elseif (VaniChaoticLevel==nil) then
				level=SkillList[VANILMIRTH][HVAN_CHAOTIC]
			else
				level=VaniChaoticLevel
			end
		end
	else
		--currently no merc healing skills
	end
	return skill,level
end

function GetSnipeSkill(myid)
	return GetAtkSkill(myid)
end

-- When several skill types are usable on the same cycle, the higher priority wins.
-- Types not listed (debuff, combo, grapple, minion) are never overridden by damage skills.
SkillTypePriority={}
SkillTypePriority[-1]=-1
SkillTypePriority[MAIN_ATK]=0
SkillTypePriority[S_ATK]=1
SkillTypePriority[CLASS_NEW_S]=2
SkillTypePriority[MOB_ATK]=3
SkillTypePriority[CLASS_NEW_MOB]=4

function SkillTypeOutranks(newtype,curtype)
	local newp=SkillTypePriority[newtype] or 10
	local curp=SkillTypePriority[curtype] or 10
	return newp > curp
end

function IsMobSkillType(skilltype)
	return skilltype==MOB_ATK or skilltype==CLASS_NEW_MOB
end

function IsSkillReady(skill)
	if AutoSkillCooldown~=nil and AutoSkillCooldown[skill]~=nil and GetTick() < AutoSkillCooldown[skill] then
		return false
	end
	-- without enough spirit spheres the server only answers "Skill failed"
	local sphereCost = SkillSphereCost and SkillSphereCost[skill]
	if sphereCost ~= nil and MySpheres < sphereCost then
		return false
	end
	return true
end

function GetSkillAoESize(skill,level)
	local aoeinfo=SkillAOEInfo[skill]
	if aoeinfo==nil or type(aoeinfo[1])~="table" then
		return nil
	end
	local size=aoeinfo[1][level]
	if size==nil then
		size=aoeinfo[1][#aoeinfo[1]]
	end
	return size
end

function IsCasterCenteredAoE(skill)
	local aoeinfo=SkillAOEInfo[skill]
	if aoeinfo~=nil and aoeinfo[2]==1 then
		return true
	end
	return GetSkillInfo(skill,7)==0
end

-- Decides whether an AoE skill is worth casting on target this cycle
function ShouldUseMobSkill(skill,level,target,tact_skillclass)
	local size=GetSkillAoESize(skill,level)
	if size~=nil and IsCasterCenteredAoE(skill) and GetDistanceAR(MyID,target) > (size-1)/2 then
		return false -- the current target would not even be hit
	end
	if tact_skillclass==CLASS_MOB then
		return true
	end
	local mobmode=0
	if AutoMobMode==2 then
		mobmode=1
	end
	return GetMobCount(skill,level,target,mobmode) >= AutoMobCount
end

function GetTargetedSkills(myid)
	local s,l,Mainatk,Satk,ComboAtk,GrappleAtk,Mobatk,Debuffatk,Minionatk,NewSAtk,NewMobatk
	s,l=GetAtkSkill(myid)
	Mainatk={MAIN_ATK,s,l}
	s,l=GetSAtkSkill(myid)
	Satk={S_ATK,s,l}
	s,l=GetComboSkill(myid)
	ComboAtk={COMBO_ATK,s,l}
	s,l=GetGrappleSkill(myid)
	GrappleAtk={GRAPPLE_ATK,s,l}
	s,l=GetMobSkill(myid)
	Mobatk={MOB_ATK,s,l}
	s,l=GetDebuffSkill(myid)
	Debuffatk={DEBUFF_ATK,s,l}
	s,l=GetMinionSkill(myid)
	Minionatk={MINION_ATK,s,l}
	s,l=GetNewSAtkSkill(myid)
    NewSAtk={CLASS_NEW_S,s,l}
	s,l=GetNewMobSkill(myid)
    NewMobatk={CLASS_NEW_MOB,s,l}
	local result={Mainatk,Satk,ComboAtk,GrappleAtk,Mobatk,Debuffatk,Minionatk,NewSAtk,NewMobatk}
	return result
end

function GetSkillInfo(skill,info,level)
	if skill~=nil and info~=nil then
		if SkillInfo[skill]~=nil then
			local t=SkillInfo[skill][info]
			if t~=nil then
				if level ==nil and type(t)=="table" then
					logappend("AAI_ERROR","Attempted to call GetSkillInfo() with only 2 arguments "..skill.." "..info)
					if type(t)~="table" and t~=nil then
						return t
					else
						return -1
					end
				elseif type(t)~="table" then
					return t
				elseif SkillInfo[skill][info][level]~=nil then
					return SkillInfo[skill][info][level]
				else
					logappend("AAI_ERROR","GetSkillInfo(): No skill info type "..info.." found for this level "..FormatSkill(skill,level))
					if info==2 then
						local r=0
						if (_VERSION=="Lua 5.1") then
							r=GetV(V_SKILLATTACKRANGE_LEVEL,MyID,skill,level)
						else
							r=GetV(V_SKILLATTACKRANGE,MyID,skill)
						end
						logappend("AAI_ERROR","GetSkillInfo(): Range for unknown skill requested, returning builtin value"..r.." "..skill)
						return r
					elseif SkillInfo[skill][info][1]~= nil then
						return SkillInfo[skill][info][1]
					else
						return 0
					end
				end
			else
				logappend("AAI_ERROR","GetSkillInfo(): No skill info type "..info.." found for this skill "..FormatSkill(skill,level).." called by ")
				if info==2 then
					logappend("AAI_ERROR","GetSkillInfo(): Range for unknown skill requested, returning builtin value"..GetV(V_SKILLATTACKRANGE,MyID,skill))
					return GetV(V_SKILLATTACKRANGE,MyID,skill)
				elseif info==7 then
					return 1
				else
					return 0
				end
			end
		else
			logappend("AAI_ERROR","GetSkillInfo(): No skill info found for this skill "..FormatSkill(skill,level).." info type requested "..info)
			if info==2 then
				logappend("AAI_ERROR","GetSkillInfo(): Range for unknown skill requested, returning builtin value"..GetV(V_SKILLATTACKRANGE,MyID,skill))
				return GetV(V_SKILLATTACKRANGE,MyID,skill)
			elseif info==7 then
				return 1
			else
				return 0
			end
		end
	else
		if skill==nil then
			skill="nil"
		end
		if level==nil then
			level="nil"
		end
		if info==nil then
			info="nil"
		end
		logappend("AAI_ERROR","GetSkillInfo(): Invalid arguments - skill="..skill.." level="..level.." info="..info)
		return 0
	end
end
--#######################
--### Other Functions ###
--#######################

function KiteOK(myid)
	mertype=GetV(V_MERTYPE,myid)
	if (mertype==nil) then
		if MyHomunType < 17 then
			homuntype=modulo(MyHomunType,4)
		else
			homuntype=modulo(DetectedOldHomunType or OldHomunType,4)
		end
		if ((homuntype==0 or homuntype==3 )and DoNotChase==1) then
			return 1
		else
			return 0
		end
	elseif (mertype < 11) then
		return 1
	else
		return 0
	end
end

function DoSkill(skill,level,target,mode,targx,targy)
	if skill==0 or level==0 or skill==nil or level==nil then
		logappend("AAI_ERROR","doskill called skill:"..tostring(skill).."level:"..tostring(level).."target"..tostring(target).."mode"..tostring(mode).."state "..MyState.."pstate "..MyPState)
		return 0
	end
	-- Never request a level the server will refuse (each refusal is a "Skill failed" message)
	level = GetCastableSkillLevel(skill, level)
	if level <= 0 then
		logappend("AAI_SKILLFAIL", "DoSkill: "..FormatSkill(skill,1).." not learned, skipped")
		if AutoSkillCooldown[skill] ~= nil then
			AutoSkillCooldown[skill] = GetTick() + 60000
		end
		return 0
	end
	-- the server needs MORE SP than the cost for homunculus/mercenary skills (rAthena
	-- skill_check_condition_mercenary: sp <= cost fails); at exactly the cost the request
	-- would only produce "Skill failed" until the next SP regen tick
	if GetV(V_SP,MyID) <= GetSkillInfo(skill,3,level) then
		TraceAI("DoSkill: "..FormatSkill(skill,level).." skipped, SP not above cost")
		return 0
	end
	TraceAI("doskill called skill:"..skill.."level:"..level.."target"..tostring(target))
	-- SP before the request, so the skill watcher can spot the cost even if SP regen lands in between
	local spBeforeCast = GetV(V_SP,MyID)
	local targetmode=GetSkillInfo(skill,7)
	if skill==HFLI_SBR44 and AllowSBR44~=1 then
		logappend("AAI_ERROR","Attempt to use SBR 44 blocked. If you really want to use this, set AllowSBR44 = 1")
	elseif targetmode==0 then
		SkillObject(MyID,level,skill,MyID)
	elseif targetmode==1 then
		SkillObject(MyID,level,skill,target)
	elseif targetmode==2 then
		if targx == nil then
			local x,y=GetV(V_POSITION,target)
			SkillGround(MyID,level,skill,x,y)
		else
			SkillGround(MyID,level,skill,targx,targy)
		end
	end
	if mode~=nil and mode > 0 then
		CastSkillMode=mode
		CastSkill=skill
		CastSkillLevel=level
		CastSkillTime=GetTick()
		CastSkillState=0
		CastSkillSP=spBeforeCast
		logappend("AAI_SKILLFAIL", "Mode set "..mode.." skill "..skill.." level "..level)
	else
		if mode~=nil then --mode is negative, call the plugin mode handler.
			DoSkillHandleMode(skill,level,target,mode,targx,targy)
			logappend("AAI_SKILLFAIL", "Mode set "..mode.." skill "..skill.." level "..level)
		end
		-- Attack skills: watch them too (mode 0) so refused levels can be detected, unless a
		-- buff cast is already being watched.
		if CastSkill==0 then
			CastSkillMode=0
			CastSkill=skill
			CastSkillLevel=level
			CastSkillTime=GetTick()
			CastSkillState=0
			CastSkillSP=spBeforeCast
		end
	end
	local t=GetTick();
	delay=AutoSkillDelay + GetSkillInfo(skill,4,level)+GetSkillInfo(skill,5,level)*CastTimeRatio
	AutoSkillCastTimeout=delay+t
	if AutoSkillCooldown[skill]~=nil then
		AutoSkillCooldown[skill]=t+GetSkillInfo(skill,9,level)+delay
		-- cooldowns survive map changes on the server; save them so the reloaded AI
		-- doesn't recast early and get "Skill failed"
		if GetSkillInfo(skill,9,level) > 0 then
			UpdateTimeoutFile()
		end
	elseif (skill==MH_VOLCANIC_ASH) then --handle the three ash timeouts
		-- a slot is busy until that ash disappears ([8]), not just until the reuse delay ends
		local ashBusy = math.max(GetSkillInfo(skill,8,level), GetSkillInfo(skill,9,level)) + delay
		if (AshTimeout[1] < t) then
			AshTimeout[1]=t+ashBusy
		elseif (AshTimeout[2] < t) then
			AshTimeout[2]=t+ashBusy
		else 
			AshTimeout[3]=t+ashBusy
		end
	end
	delay = delay + GetSkillInfo(skill,6,level)
	AutoSkillTimeout=t+delay
	if AutoSkillCooldown[skill]~=nil then
		TraceAI("DoSkill: "..skill.." level:"..level.." target:"..target.." mode:"..targetmode.." delay "..delay.." cooldown: "..AutoSkillCooldown[skill]-GetTick())
	else
		TraceAI("DoSkill: "..skill.." level:"..level.." target:"..target.." mode:"..targetmode.." delay "..delay)
	end
	-- spheres the server takes for this skill (SkillSphereCost only exists in the homunculus AI)
	local sphereCost = SkillSphereCost and SkillSphereCost[skill]
	if sphereCost ~= nil then
		MySpheres = math.max(0,MySpheres - sphereCost)
	end
	if skill==MH_MIDNIGHT_FRENZY then
		ComboSVTimeout=0
		UpdateTimeoutFile()
	elseif skill==MH_SILVERVEIN_RUSH then
		ComboSVTimeout=GetTick()+2000
		ComboSCTimeout=0
		UpdateTimeoutFile()
	elseif skill==MH_SONIC_CRAW then
		ComboSCTimeout=GetTick()+2000
		ComboSVTimeout=0
	elseif skill==MH_TINDER_BREAKER then
		ComboSCTimeout=GetTick()+2000
		ComboSVTimeout=0
	elseif skill==MH_CBC then
		ComboSVTimeout=GetTick()+2000
		ComboSCTimeout=0
	elseif skill==MH_EQC then
		ComboSVTimeout=0
		ComboSCTimeout=0
	elseif skill==MH_BLAZING_AND_FURIOUS then
		-- the server uses up every sphere after counting the hits
		MySpheres=0
		ComboSCTimeout=0
		ComboSVTimeout=0
		UpdateTimeoutFile()
	elseif skill==MH_THE_ONE_FIGHTER_RISES then
		MySpheres = math.min(10,MySpheres + 1)
		ComboSCTimeout=0
		ComboSVTimeout=0
		UpdateTimeoutFile()
	else --Combo wasn't used, so kill the timeouts
		ComboSCTimeout=0
		ComboSVTimeout=0
	end

	TraceAI("DoSkill: "..skill.." level:"..level.." target:"..target.." mode:"..targetmode.." delay "..delay)
	logappend("AAI_SKILLFAIL", "DoSkill: "..skill.." level:"..level.." target:"..target.." mode:"..targetmode.." delay "..delay)
	-- the level actually requested (may be lower than asked, see GetCastableSkillLevel)
	return level
end

function modtwroSend()
	if modtwROSkillGroundID~=0 then
		logappend("AAI_Lag","Calling skillground function")
		OldSkillGround(MyID,modtwROSkillGroundLV,modtwROSkillGroundID,modtwROSkillGroundX,modtwROSkillGroundY)
		LagReductionCD=LagReduction
	elseif modtwROSkillObjectID~=0 then
		logappend("AAI_Lag","Calling SkillObject function")
		OldSkillObject(MyID,modtwROSkillObjectLV,modtwROSkillObjectID,modtwROSkillObjectTarg)
		LagReductionCD=LagReduction
	else
		if modtwRODidMove==0 then
			if modtwROMoveX~=0 then
				logappend("AAI_Lag","Calling Move function, didmove=0")
				OldMove(MyID,modtwROMoveX,modtwROMoveY)
				LagReductionCD=LagReduction
				modtwRODidMove=1
				modtwRODidAttack=0
			end
		else
			modtwRODidMove=0
		end
		if modtwRODidAttack==0 then
			if modtwROAttackTarget~=0 then
				logappend("AAI_Lag","Calling attack function, didattack=0")
				OldAttack(MyID,modtwROAttackTarget)
				LagReductionCD=LagReduction
				modtwRODidAttack=1
			end
		else
			modtwRODidAttack=0
		end
	end
	modtwROSkillGroundLV,modtwROSkillGroundID,modtwROSkillGroundX,modtwROSkillGroundY,modtwROSkillObjectLV,modtwROSkillObjectID,modtwROSkillObjectTarg,modtwROAttackTarget=0,0,0,0,0,0,0,0
	modtwROAttackTarget,modtwROMoveX,modtwROMoveY=0,0,0
end








-- I SHOULDNT HAVE TO CODE THIS!
function modulo(a,b)
	return a-math.floor(a/b)*b
end
-- OR THIS!
function len(listin)
	local length=0
	for k,v in pairs(listin) do
		length = length + 1
	end
	return length
end

function absceil(x)
	if x>0 then
		return math.ceil(x)
	else
		return math.floor(x)
	end
end

function absunit(x)
	if x==0 then
		return 0
	elseif x > 0 then
		return 1
	else -- x < 0
		return -1
	end
end


-- Logging functions:

function 	FormatSkill(skill,level)
	if skill==nil then
		return "nil skill"
	end
	local output
	local name
	if SkillInfo[skill]==nil then
		name=skill
	elseif SkillInfo[skill][1]==nil then
		name=skill
	else
		name=SkillInfo[skill][1]
	end
	output = name.." ("..skill..")"
	if level == nil then
		return output
	else 
		return output.." level "..level
	end
end

function FormatMotion(motion)
	if motion==0 then
		return "Standing ("..motion..")"
	elseif motion==1 then
		return "Moving ("..motion..")"
	elseif motion==2 or motion==9 then 
		return "Attacking ("..motion..")"
	elseif motion==3 then 
		return "Dead ("..motion..")"
	elseif motion==4 then 
		return "Flinching  ("..motion..")"
	elseif motion==5 then
		return "Bending over ("..motion..")"
	elseif motion==6 then
		return "Sitting ("..motion..")"
	elseif motion==7 then 
		return "Using skill ("..motion..")"
	elseif motion==8 then
		return "Casting ("..motion..")"
	else
		return motion
	end
end

function formatval (val)
	if val == nil then
		return "nil"
	elseif type(val)=="string" then
		return "'"..val.."'"
	elseif type(val)=="function" then
		return "function"
	end
	return val
end

function logappend (filename,message)
	if LogEnable[filename]==1 then
		TraceAI("Logging "..filename.." - "..message)
		outfile = io.open("./"..filename..".log", "a")
		outfile:seek("end")
		if filename=="AAI_SKILLFAIL" then
			outfile:write(os.date("%c").." ("..GetTick()..") "..TypeString..MyState.."\t"..message.."\n")
		else
			outfile:write(os.date("%c").." "..TypeString..MyState.."\t"..message.."\n")
		end
		outfile:close()
	end
end

function formatmypos(steps)
	if steps>10 then
		logappend("AAI_ERROR","Invalid number of steps called for formatmypos() "..steps)
		steps=10
	elseif steps <1 then
		logappend("AAI_ERROR","Invalid number of steps called for formatmypos() "..steps)
		steps=1
	end
	local outstring=""
	for v=1,steps,1 do
		outstring=outstring.."("..MyPosX[v]..","..MyPosY[v]..")"
		if v~=step then
			outstring=outstring..","
		end
	end
	return outstring
end

function formatpos(x,y)
	return x..","..y
end
--------------------------------------------
-- List utility
--------------------------------------------
List = {}

function List.new ()
	return { first = 0, last = -1}
end

function List.pushleft (list, value)
	local first = list.first-1
	list.first  = first
	list[first] = value;
end

function List.pushright (list, value)
	local last = list.last + 1
	list.last = last
	list[last] = value
end

function List.popleft (list)
	local first = list.first
	if first > list.last then 
		return nil
	end
	local value = list[first]
	list[first] = nil         -- to allow garbage collection
	list.first = first+1
	return value
end

function List.popright (list)
	local last = list.last
	if list.first > last then
		return nil
	end
	local value = list[last]
	list[last] = nil
	list.last = last-1
	return value 
end

function List.clear (list)
	for i,v in ipairs(list) do
		list[i] = nil
	end
--[[
	if List.size(list) == 0 then
		return
	end
	local first = list.first
	local last  = list.last
	for i=first, last do
		list[i] = nil
	end
--]]
	list.first = 0
	list.last = -1
end

function List.size (list)
	local size = list.last - list.first + 1
	return size
end

---
-- Gets the actual learned level of a given skill for a homunculus or mercenary.
-- This is the definitive, corrected version that uses the modern V_SKILLATTACKRANGE_LEVEL
-- constant, correctly handles Homunculus S types checking old skills, and correctly
-- interprets the API's return value of 1 for "not learned".
--
-- @param myid (number): The ID of the homunculus or mercenary.
-- @param skillID (number): The constant for the skill to check (e.g., HFLI_MOON).
-- @return (number): The learned level of the skill (1-10), 0 if not learned, nil if unknown
--         (no skill list for this unit).
-- @return (boolean): true if the level is certain. The client reports range 1 both for
--         "not learned" and for levels whose real range is 1, so for those skills the
--         first value is only an upper bound.
---
function GetLearnedSkillLevel(myid, skillID)
    local maxLevel = 0
    
    if IsHomun(myid) == 1 then
        -- For homunculi, we need to determine which type to use for the lookup.
        local htype = MyHomunType
        local typeForLookup = htype -- Default to the current type (EIRA, BAYERI, etc.)

        -- Check if the skill belongs to one of the original four homunculi.
        if SkillList[LIF][skillID] or SkillList[AMISTR][skillID] or SkillList[FILIR][skillID] or SkillList[VANILMIRTH][skillID] then
            -- It's an old skill. Use our reliable detection function to get the base type.
            typeForLookup = GetDetectedHomunType(myid)
        end
        
        -- Now, get the max possible level for this skill from the correct table entry.
        if SkillList[typeForLookup] == nil then
            return nil, false
        end
        maxLevel = SkillList[typeForLookup][skillID] or 0
    else
        local mtype = GetMerType(myid)
        if not (SkillListM and SkillListM[mtype]) then
            return nil, false
        end
        maxLevel = SkillListM[mtype][skillID] or 0
    end

    -- If the skill doesn't exist for this unit type, it can't be learned.
    if maxLevel == 0 then
        return 0, true
    end

    -- Loop downwards from the max level to find the actual learned level.
    local ambiguous = 0
    for level = maxLevel, 1, -1 do
        -- It returns 1 if the skill is NOT known at this level.
        if GetV(V_SKILLATTACKRANGE_LEVEL, myid, skillID, level) ~= 1 then
            return level, true
        end
        -- Levels whose real range is 1 (e.g. Blast Forge, Toxin Lv1-4) also return 1 when learned.
        -- The highest such level is an upper bound; lower levels must still be probed.
        if ambiguous == 0 and SkillInfo[skillID] ~= nil and GetSkillInfo(skillID, 2, level) == 1 then
            ambiguous = level
        end
    end

    if ambiguous > 0 then
        return ambiguous, false
    end
    return 0, true
end

-- What the AI knows about the learned level of each skill, used by DoSkill() so it never asks the
-- server for a level the homunculus doesn't have (each refusal shows "Skill failed" in the client).
--   cap      highest level we may request (0 = not learned)
--   upper    upper bound from the client probe
--   exact    cap came from an unambiguous probe (cap == upper)
--   ok       highest level seen succeeding this session (never step below it)
--   rejects  consecutive refused casts at the current cap
--   probeUp  may try cap+1 once (the skill may have been levelled since it was saved)
-- Skills whose level can't be probed (range 1) are saved per owner in data/H_<owner>Skills.lua as
-- {cap, os.time() of the last check}, so the stepping down isn't repeated on every map change and
-- the level-up try happens at most once per LEARNED_SKILL_RECHECK_SECONDS.
LearnedSkill = {}
LearnedSkillSaved = nil
SKILL_REJECTS_TO_STEP_DOWN = 2
LEARNED_SKILL_RECHECK_SECONDS = 3600

function LearnedSkillFile()
	return ConfigPath.."data/H_"..OwnerID.."Skills.lua"
end

function SaveLearnedSkills()
	local OutFile = io.open(LearnedSkillFile(), "w")
	if OutFile == nil then
		logappend("AAI_ERROR", "Failed to save learned skill levels for owner "..OwnerID)
		return
	end
	for skill, saved in pairs(LearnedSkillSaved) do
		OutFile:write("LearnedSkillSaved["..skill.."]={"..saved[1]..","..saved[2].."}\n")
	end
	OutFile:close()
end

function SaveLearnedSkillCap(skill, cap)
	LearnedSkillSaved[skill] = { cap, os.time() }
	SaveLearnedSkills()
end

function SetLearnedSkillCap(skill, info, cap)
	info.cap = cap
	info.rejects = 0
	if not info.exact then
		SaveLearnedSkillCap(skill, cap)
	end
end

function GetSkillLevelCap(skill)
	local info = LearnedSkill[skill]
	if info == nil then
		local cap, exact = nil, false
		if IsHomun(MyID) == 1 then
			cap, exact = GetLearnedSkillLevel(MyID, skill)
		end
		info = { cap = cap, upper = cap, exact = exact, ok = 0, rejects = 0, probeUp = false }
		LearnedSkill[skill] = info
		if cap ~= nil and not exact then
			if LearnedSkillSaved == nil then
				LearnedSkillSaved = {}
				pcall(dofile, LearnedSkillFile())
			end
			local saved = LearnedSkillSaved[skill]
			if type(saved) == "table" and type(saved[1]) == "number" and saved[1] < cap then
				info.cap = math.max(saved[1], 0)
				info.probeUp = os.time() - (tonumber(saved[2]) or 0) >= LEARNED_SKILL_RECHECK_SECONDS
			end
		end
		if cap ~= nil then
			logappend("AAI_SKILLFAIL", "Learned level of "..FormatSkill(skill, info.cap).." upper "..cap.." exact="..tostring(exact))
		end
	end
	return info.cap
end

-- Level DoSkill() should actually request (0 = don't cast).
function GetCastableSkillLevel(skill, level)
	local cap = GetSkillLevelCap(skill)
	if cap == nil or level <= cap then
		return level
	end
	local info = LearnedSkill[skill]
	if info.probeUp and cap < info.upper then
		info.probeUp = false
		SaveLearnedSkillCap(skill, cap)
		logappend("AAI_SKILLFAIL", "Trying "..FormatSkill(skill, cap + 1).." in case it was levelled up")
		return cap + 1
	end
	return cap
end

-- Called by the skill watcher once it knows whether a cast went through.
function SkillCastResult(skill, level, success)
	local info = LearnedSkill[skill]
	if info == nil or info.cap == nil then
		return
	end
	if success then
		info.rejects = 0
		if level > info.ok then
			info.ok = level
		end
		if level > info.cap then
			SetLearnedSkillCap(skill, info, level)
			info.probeUp = true
		end
		return
	end
	-- A refused level-up probe just confirms the saved level.
	-- Probed levels are trusted; failures there are range/interrupt problems, not the level.
	if level > info.cap or info.exact or level ~= info.cap or level <= info.ok then
		return
	end
	info.rejects = info.rejects + 1
	if info.rejects >= SKILL_REJECTS_TO_STEP_DOWN then
		TraceAI("Server keeps refusing "..FormatSkill(skill, level)..", lowering to level "..(level - 1))
		logappend("AAI_SKILLFAIL", "Server keeps refusing "..FormatSkill(skill, level)..", lowering to level "..(level - 1))
		SetLearnedSkillCap(skill, info, level - 1)
		if info.cap > 0 and AutoSkillCooldown[skill] ~= nil then
			AutoSkillCooldown[skill] = 1
		end
	end
end

---
-- Gets the fundamental base type for ANY homunculus.
-- This is the final, corrected version that uses unambiguous skill checks
-- and the correct API return value logic.
-- It runs a detection check only on the very first call. If detection
-- fails, it uses the global OldHomunType as a failsafe.
-- @param myid (number): The ID of the homunculus.
-- @return (number): The numeric constant for the homunculus base type (LIF, AMISTR, etc.).
---
function GetDetectedHomunType(myid)
    -- If we have already run the detection, return the stored result instantly.
    if DetectedOldHomunType ~= nil then
        return DetectedOldHomunType
    end

    TraceAI("=== Running Homunculus Type Detection (v5) ===")

    -- THE FIX: We check for a single, defining skill for each type.
    -- The check `GetV(V_SKILLATTACKRANGE, myid, SKILL_ID) ~= 1` is the correct
    -- way to see if a skill is known, as it returns 1 if not learned.
    -- We prioritize checking skills that do NOT have a range of 1 to avoid ambiguity.
    
    -- Check for Filir using a buff (range 0) first. This is the most reliable check.
    if GetV(V_SKILLATTACKRANGE, myid, HFLI_FLEET) ~= 1 then
        DetectedOldHomunType = FILIR
        TraceAI("Homunculus base type auto-detected as: Filir")
        return DetectedOldHomunType
    -- Check for Vanilmirth (range 9)
    elseif GetV(V_SKILLATTACKRANGE, myid, HVAN_CAPRICE) ~= 1 then
        DetectedOldHomunType = VANILMIRTH
        TraceAI("Homunculus base type auto-detected as: Vanilmirth")
        return DetectedOldHomunType
    -- Check for Amistr using a buff (range 0)
    elseif GetV(V_SKILLATTACKRANGE, myid, HAMI_DEFENCE) ~= 1 then
        DetectedOldHomunType = AMISTR
        TraceAI("Homunculus base type auto-detected as: Amistr")
        return DetectedOldHomunType
    -- Check for Lif using a buff (range 0)
    elseif GetV(V_SKILLATTACKRANGE, myid, HLIF_HEAL) ~= 1 then
        DetectedOldHomunType = LIF
        TraceAI("Homunculus base type auto-detected as: Lif")
        return DetectedOldHomunType
    end

    -- Failsafe: If auto-detection fails (e.g., no skills learned),
    -- fall back to the user's manually configured setting.
    TraceAI("Auto-detection failed. Using manually configured OldHomunType as failsafe.")
    DetectedOldHomunType = OldHomunType -- The global variable from the config
    return DetectedOldHomunType
end

---
-- Checks and corrects user configuration at startup. This is the definitive version.
-- 1. Disables all skill options not relevant to the current homunculus type.
-- 2. For relevant skills, it uses GetLearnedSkillLevel to find the actual learned level.
-- 3. If the user's configured level is higher than the learned level, it adjusts it down.
-- 4. If a skill is not learned at all (level 0), it disables the 'Use' option for it.
--
-- @param myid (number): The ID of the homunculus.
-- @param logstring (string): The existing log string to append messages to.
-- @return (string): The updated log string with any new messages.
---
function LintSkillOptions(myid, logstring)
    local htype = MyHomunType
    
    -- A master list of all homunculus-specific skill options, grouped by type.
    -- { "Use" option, "Level" option (or nil), skill_constant, friendly_skill_name }
    local allSkillOptions = {
        [EIRA] = {
            { "UseEiraEraseCutter", "EiraEraseCutterLevel", MH_ERASER_CUTTER, "Eraser Cutter" },
            { "UseEiraXenoSlasher", "EiraXenoSlasherLevel", MH_XENO_SLASHER, "Xeno Slasher" },
            { "UseEiraSilentBreeze", "EiraSilentBreezeLevel", MH_SILENT_BREEZE, "Silent Breeze" },
            { "UseEiraOveredBoost", nil, MH_OVERED_BOOST, "Overed Boost" },
            { "UseEiraLightOfRegene", "EiraLightOfRegeneLevel", MH_LIGHT_OF_REGENE, "Light of Regenerate" },
            { "UseEiraTwisterCutter", "EiraTwisterCutterLevel", MH_TWISTER_CUTTER, "Twister Cutter" },
            { "UseEiraAbsoluteZephyr", "EiraAbsoluteZephyrLevel", MH_ABSOLUTE_ZEPHYR, "Absolute Zephyr" }
        },
        [BAYERI] = {
            { "UseBayeriStahlHorn", "BayeriStahlHornLevel", MH_STAHL_HORN, "Stahl Horn" },
            { "UseBayeriHailegeStar", "BayeriHailegeStarLevel", MH_HEILIGE_STANGE, "Heilige Stange" },
            { "UseBayeriAngriffModus", nil, MH_ANGRIFFS_MODUS, "Angriffs Modus" },
            { "UseBayeriGoldenPherze", nil, MH_GOLDENE_FERSE, "Goldene Ferse" },
            { "UseBayeriSteinWand", "BayeriSteinWandLevel", MH_STEINWAND, "Stein Wand" },
            { "UseBayeriGlanzenSpies", "BayeriGlanzenSpiesLevel", MH_GLANZEN_SPIES, "Glanzen Spies" },
            { "UseBayeriHeiligePferd", "BayeriHeiligePferdLevel", MH_HEILIGE_PFERD, "Heilige Pferd" },
            { "UseBayeriGoldeneTone", "BayeriGoldeneToneLevel", MH_GOLDENE_TONE, "Goldene Tone" }
        },
        [SERA] = {
            { "UseSeraParalyze", "SeraParalyzeLevel", MH_NEEDLE_OF_PARALYZE, "Needle of Paralyze" },
            { "UseSeraPoisonMist", "SeraPoisonMistLevel", MH_POISON_MIST, "Poison Mist" },
            { "UseSeraPainkiller", nil, MH_PAIN_KILLER, "Pain Killer" },
            { "UseSeraCallLegion", "SeraCallLegionLevel", MH_SUMMON_LEGION, "Summon Legion" },
            { "UseSeraToxinOfMandara", "SeraToxinOfMandaraLevel", MH_TOXIN_OF_MANDARA, "Toxin of Mandara" },
            { "UseSeraNeedleStinger", "SeraNeedleStingerLevel", MH_NEEDLE_STINGER, "Needle Stinger" }
        },
        [DIETER] = {
            { "UseDieterLavaSlide", "DieterLavaSlideLevel", MH_LAVA_SLIDE, "Lava Slide" },
            { "UseDieterVolcanicAsh", nil, MH_VOLCANIC_ASH, "Volcanic Ash" },
            { "UseDieterMagmaFlow", nil, MH_MAGMA_FLOW, "Magma Flow" },
            { "UseDieterGraniticArmor", nil, MH_GRANITIC_ARMOR, "Granitic Armor" },
            { "UseDieterPyroclastic", "DieterPyroclasticLevel", MH_PYROCLASTIC, "Pyroclastic" },
            { "UseDieterTempering", nil, MH_TEMPERING, "Tempering" },
            { "UseDieterBlastForge", nil, MH_BLAST_FORGE, "Blast Forge" }
        },
        [ELEANOR] = {
            { "UseEleanorSonicClaw", "EleanorSonicClawLevel", MH_SONIC_CRAW, "Sonic Claw" },
            { "UseEleanorBlazingAndFurious", "EleanorBlazingAndFuriousLevel", MH_BLAZING_AND_FURIOUS, "Blazing and Furious" },
            { "UseEleanorTheOneFighterRises", "EleanorTheOneFighterRisesLevel", MH_THE_ONE_FIGHTER_RISES, "The One Fighter Rises" }
        }
    }

    -- Stage 1: Disable all options for homunculus types that we are NOT.
    for homunType, options in pairs(allSkillOptions) do
        if homunType ~= htype then
            for _, optionData in ipairs(options) do
                local useOptionName = optionData[1]
                if _G[useOptionName] ~= nil and _G[useOptionName] ~= 0 then
                    logstring = logstring .. "Disabling irrelevant option: " .. useOptionName .. "\n"
                    _G[useOptionName] = 0
                end
            end
        end
    end

    -- Stage 2: For our current homunculus type, check skills and adjust levels.
    if allSkillOptions[htype] then
        for _, optionData in ipairs(allSkillOptions[htype]) do
            local useOptionName = optionData[1]
            local levelOptionName = optionData[2]
            local skillConst = optionData[3]
            local skillName  = optionData[4]

            local learnedLevel = GetSkillLevelCap(skillConst)
            local info = LearnedSkill[skillConst]

            if learnedLevel == nil then
                -- No skill list for this unit, nothing to check.
            elseif learnedLevel == 0 and info.exact then
                -- Skill is not learned at all.
                if _G[useOptionName] and _G[useOptionName] ~= 0 then
                    logstring = logstring .. "Disabled " .. useOptionName .. " - skill ("..skillName..") not learned.\n"
                    _G[useOptionName] = 0
                end
            else
                -- Check if the configured level is too high (if a level option exists). For skills whose
                -- level can't be probed keep the upper bound, so DoSkill() can still try one level up.
                if levelOptionName and _G[levelOptionName] and _G[levelOptionName] > info.upper then
                    logstring = logstring .. "Adjusting " .. levelOptionName .. " from " .. _G[levelOptionName] .. " to learned level " .. info.upper .. ".\n"
                    _G[levelOptionName] = info.upper
                end
                if not info.exact then
                    logstring = logstring .. skillName .. ": learned level can't be read exactly (range 1); using level " .. learnedLevel .. " (at most " .. info.upper .. "), adjusted if the server refuses it.\n"
                end
            end
        end
    end

    return logstring
end

-- New Functions
function GetSOwner2ndBuffSkill(myid)
	local level = 0
	local skill = 0
	local skillopt = 0
	if (IsHomun(myid)==1) then
		htype=MyHomunType
		if	htype==DIETER and UseDieterTempering~=0 then
			skill = MH_TEMPERING
			level = SkillList[htype][skill]
			skillopt = UseDieterTempering
		elseif htype==EIRA and UseEiraLightOfRegene~=nil and UseEiraLightOfRegene~=0 then
			skill = MH_LIGHT_OF_REGENE
			skillopt = UseEiraLightOfRegene
			level = math.min(EiraLightOfRegeneLevel or 5, SkillList[htype][skill] or 0)
			if level > 0 and (GetV(V_MOTION,OwnerID)==MOTION_DEAD or HPPercent(OwnerID) > (EiraLightOfRegeneOwnerHP or 100)) then
				-- level 0 with a valid skill means "not now"; returning 0,0,0 would disable the buff for good
				return skill,0,skillopt
			end
		end
	end
	if level==0 then
		return 0,0,0
	end
	return skill,level,skillopt
end

-- Returns the first enabled, off-cooldown skill from {enabled, skill, configuredLevel} entries
function PickReadySkill(htype,candidates)
	for i,c in ipairs(candidates) do
		if c[1]~=nil and c[1]~=0 and c[2]~=nil and IsSkillReady(c[2]) then
			local level=c[3] or SkillList[htype][c[2]] or 0
			if level~=0 then
				return c[2],level
			end
		end
	end
	return 0,0
end

function GetNewSAtkSkill(myid)
	if IsHomun(myid)~=1 then
		return 0,0
	end
	local htype=MyHomunType
	if htype==EIRA then
		return PickReadySkill(htype,{{UseEiraTwisterCutter,MH_TWISTER_CUTTER,EiraTwisterCutterLevel}})
	elseif htype==BAYERI then
		return PickReadySkill(htype,{{UseBayeriGlanzenSpies,MH_GLANZEN_SPIES,BayeriGlanzenSpiesLevel}})
	elseif htype==SERA then
		return PickReadySkill(htype,{{UseSeraNeedleStinger,MH_NEEDLE_STINGER,SeraNeedleStingerLevel}})
	elseif htype==ELEANOR then
		return PickReadySkill(htype,{
			{UseEleanorBlazingAndFurious,MH_BLAZING_AND_FURIOUS,EleanorBlazingAndFuriousLevel},
			{UseEleanorTheOneFighterRises,MH_THE_ONE_FIGHTER_RISES,EleanorTheOneFighterRisesLevel}})
	end
	return 0,0
end

function GetNewMobSkill(myid)
	if IsHomun(myid)~=1 then
		return 0,0
	end
	local htype=MyHomunType
	if htype==EIRA then
		return PickReadySkill(htype,{{UseEiraAbsoluteZephyr,MH_ABSOLUTE_ZEPHYR,EiraAbsoluteZephyrLevel}})
	elseif htype==SERA then
		return PickReadySkill(htype,{{UseSeraToxinOfMandara,MH_TOXIN_OF_MANDARA,SeraToxinOfMandaraLevel}})
	elseif htype==BAYERI then
		return PickReadySkill(htype,{{UseBayeriHeiligePferd,MH_HEILIGE_PFERD,BayeriHeiligePferdLevel}})
	elseif htype==DIETER then
		return PickReadySkill(htype,{{UseDieterBlastForge,MH_BLAST_FORGE,nil}})
	end
	return 0,0
end