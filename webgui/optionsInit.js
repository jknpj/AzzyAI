// optionsInit.js
// Handles initialization for Homunculus and Mercenary options tables
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

// Homunculus options data
let options = [
  { name: 'AggroHP', value: 0, desc: 'Seek out and attack monsters when HP > this value.' },
  { name: 'AggroSP', value: 0, desc: 'Attack when SP > this value.' },
  { name: 'AssumeHomun', value: true, desc: 'Assume Homunculus is present.' },
  { name: 'AttackLastFullSP', value: false, desc: 'Attack only if last attack was with full SP.' },
  { name: 'AttackTimeLimit', value: 100000, desc: 'Maximum time to attack.' },
  { name: 'DanceMinSP', value: 0, desc: 'Minimum SP to dance.' },
  { name: 'DoNotAttackMoving', value: false, desc: 'Do not attack moving monsters.' },
  { name: 'DoNotChase', value: false, desc: 'Do not chase monsters.' },
  { name: 'KiteMonsters', value: false, desc: 'Kite monsters.' },
  { name: 'LagReduction', value: 0, desc: 'Lag reduction setting.' },
  { name: 'LiveMobID', value: false, desc: 'Live Mob ID.' },
  { name: 'MobileAggroDist', value: 12, desc: 'Mobile aggro distance.' },
  { name: 'OldHomunType', value: 'Amistr', desc: 'Old Homunculus type.' },
  { name: 'OpportunisticTargeting', value: false, desc: 'Opportunistic targeting.' },
  { name: 'PainkillerFriends', value: true, desc: 'Painkiller for friends.' },
  { name: 'PainkillerFriendsSave', value: true, desc: 'Save painkiller for friends.' },
  { name: 'RescueOwnerLowHP', value: 0, desc: 'Rescue owner when HP is low.' },
  { name: 'StationaryAggroDist', value: 12, desc: 'Stationary aggro distance.' },
  { name: 'SuperPassive', value: false, desc: 'Super passive mode.' },
  { name: 'TankMonsterLimit', value: 4, desc: 'Tank monster limit.' },
  { name: 'UseAttackSkill', value: true, desc: 'Use attack skill.' },
  { name: 'UseAvoid', value: false, desc: 'Use avoid.' },
  { name: 'UseDanceAttack', value: true, desc: 'Use dance attack.' },
  { name: 'AutoComboSpheres', value: 10, desc: 'Sphere threshold for starting Eleanor combos.' },
  { name: 'EleanorDoNotSwitchMode', value: false, desc: 'Prevent the AI from switching Eleanor style to continue a combo.' },
  { name: 'UseEleanorSonicClaw', value: true, desc: 'Enable Eleanor Sonic Claw.' },
  { name: 'UseDieterLavaSlide', value: true, desc: 'Enable Dieter Lava Slide.' },
  { name: 'UseDieterVolcanicAsh', value: true, desc: 'Enable Dieter Volcanic Ash debuff.' },
  { name: 'UseDieterBlastForge', value: false, desc: 'Enable Dieter Blast Forge.' },
  { name: 'UseDieterTempering', value: 0, desc: 'Choose when Dieter uses Tempering (0=never, 1=idle, -1=chase, -2=low idle, 2=berserk, 3=always).' }
];
function filterOptions() {
  const val = document.getElementById('searchBox').value;
  renderOptionsTable(options.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'optionsTableBody');
}
renderOptionsTable(options, 'optionsTableBody');
setupOptionTableEvents(options, 'optionsTableBody', 'optionDescription');
document.getElementById('searchBox').oninput = filterOptions;

// Mercenary options data
let mercOptions = [
  { name: 'AggroHP', value: 0, desc: 'Seek out and attack monsters when HP > this value.' },
  { name: 'AggroSP', value: 0, desc: 'Attack when SP > this value.' },
  { name: 'SuperPassive', value: false, desc: 'Super passive mode.' },
  { name: 'TankMonsterLimit', value: 4, desc: 'Tank monster limit.' },
  { name: 'UseAttackSkill', value: true, desc: 'Use attack skill.' },
  { name: 'UseAvoid', value: false, desc: 'Use avoid.' },
  { name: 'UseDanceAttack', value: true, desc: 'Use dance attack.' },
  { name: 'AttackTimeLimit', value: 100000, desc: 'Maximum time to attack.' },
  { name: 'DanceMinSP', value: 0, desc: 'Minimum SP to dance.' },
  { name: 'DoNotAttackMoving', value: false, desc: 'Do not attack moving monsters.' },
  { name: 'DoNotChase', value: false, desc: 'Do not chase monsters.' },
  { name: 'KiteMonsters', value: false, desc: 'Kite monsters.' },
  { name: 'LagReduction', value: 0, desc: 'Lag reduction setting.' },
  { name: 'MobileAggroDist', value: 12, desc: 'Mobile aggro distance.' },
  { name: 'StationaryAggroDist', value: 12, desc: 'Stationary aggro distance.' }
];
function filterMercOptions() {
  const val = document.getElementById('mercSearchBox').value;
  renderOptionsTable(mercOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercOptionsTableBody');
}
renderOptionsTable(mercOptions, 'mercOptionsTableBody');
setupOptionTableEvents(mercOptions, 'mercOptionsTableBody', 'mercOptionDescription');
document.getElementById('mercSearchBox').oninput = filterMercOptions;

// Export/import helpers for persistent settings
export function getHomunculusOptions() {
  return options.map(opt => ({ ...opt }));
}
export function setHomunculusOptions(newOpts) {
  alert('[DEBUG] setHomunculusOptions called: ' + JSON.stringify(newOpts));
  options = options.map(opt => {
    const found = newOpts.find(o => o.name === opt.name);
    return found ? { ...opt, value: found.value } : opt;
  });
  renderOptionsTable(options, 'optionsTableBody');
  setupOptionTableEvents(options, 'optionsTableBody', 'optionDescription');
}
export function getMercenaryOptions() {
  return mercOptions.map(opt => ({ ...opt }));
}
export function setMercenaryOptions(newOpts) {
  alert('[DEBUG] setMercenaryOptions called: ' + JSON.stringify(newOpts));
  mercOptions = mercOptions.map(opt => {
    const found = newOpts.find(o => o.name === opt.name);
    return found ? { ...opt, value: found.value } : opt;
  });
  renderOptionsTable(mercOptions, 'mercOptionsTableBody');
  setupOptionTableEvents(mercOptions, 'mercOptionsTableBody', 'mercOptionDescription');
}
