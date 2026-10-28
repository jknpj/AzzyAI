import { TacticsEditor } from './tacticsEditor.js';

// Homunculus Tactics
const homTacticsEditor = new TacticsEditor({
  data: [
    { name: 'Aggressive', type: 'Attack', skill: 'Skill1', notes: 'Attacks all monsters.' },
    { name: 'Defensive', type: 'Tank', skill: 'Skill2', notes: 'Tanks for owner.' },
    { name: 'Ignore', type: 'Ignore', skill: '', notes: 'Ignores certain monsters.' }
  ],
  listId: 'homTactList',
  formId: 'homTactForm',
  searchId: 'homTactSearch',
  fields: [
    { id: 'tactName', key: 'name' },
    { id: 'tactType', key: 'type' },
    { id: 'tactSkill', key: 'skill' },
    { id: 'tactNotes', key: 'notes' }
  ],
  typeOptions: ['Tank', 'Ignore', 'Attack', 'React', 'Snipe']
});
homTacticsEditor.init();
// Mercenary Tactics
const merTacticsEditor = new TacticsEditor({
  data: [
    { name: 'Aggressive', type: 'Attack', skill: 'SkillA', notes: 'Attacks all monsters.' },
    { name: 'Defensive', type: 'Tank', skill: 'SkillB', notes: 'Tanks for owner.' },
    { name: 'Ignore', type: 'Ignore', skill: '', notes: 'Ignores certain monsters.' }
  ],
  listId: 'merTactList',
  formId: 'merTactForm',
  searchId: 'merTactSearch',
  fields: [
    { id: 'merTactName', key: 'name' },
    { id: 'merTactType', key: 'type' },
    { id: 'merTactSkill', key: 'skill' },
    { id: 'merTactNotes', key: 'notes' }
  ],
  typeOptions: ['Tank', 'Ignore', 'Attack', 'React', 'Snipe']
});
merTacticsEditor.init();

// PVP Tactics
const pvpTacticsEditor = new TacticsEditor({
  data: [
    { name: 'Aggressive', type: 'Attack', skill: 'SkillP', notes: 'Attacks all players.' },
    { name: 'Defensive', type: 'Tank', skill: 'SkillQ', notes: 'Tanks for owner.' },
    { name: 'Ignore', type: 'Ignore', skill: '', notes: 'Ignores certain players.' }
  ],
  listId: 'pvpTactList',
  formId: 'pvpTactForm',
  searchId: 'pvpTactSearch',
  fields: [
    { id: 'pvpTactName', key: 'name' },
    { id: 'pvpTactType', key: 'type' },
    { id: 'pvpTactSkill', key: 'skill' },
    { id: 'pvpTactNotes', key: 'notes' }
  ],
  typeOptions: ['Tank', 'Ignore', 'Attack', 'React', 'Snipe']
});
pvpTacticsEditor.init();

// M_PvpTact Tactics
const mPvpTacticsEditor = new TacticsEditor({
  data: [
    { name: 'Aggressive', type: 'Attack', skill: 'SkillM', notes: 'Attacks all monsters.' },
    { name: 'Defensive', type: 'Tank', skill: 'SkillN', notes: 'Tanks for owner.' },
    { name: 'Ignore', type: 'Ignore', skill: '', notes: 'Ignores certain monsters.' }
  ],
  listId: 'mPvpTactList',
  formId: 'mPvpTactForm',
  searchId: 'mPvpTactSearch',
  fields: [
    { id: 'mPvpTactName', key: 'name' },
    { id: 'mPvpTactType', key: 'type' },
    { id: 'mPvpTactSkill', key: 'skill' },
    { id: 'mPvpTactNotes', key: 'notes' }
  ],
  typeOptions: ['Tank', 'Ignore', 'Attack', 'React', 'Snipe']
});
mPvpTacticsEditor.init();

// Export helpers for persistent settings (PVP and M_PvpTact)
export function getPvpTactics() {
  return pvpTacticsEditor.getData();
}
export function setPvpTactics(data) {
  alert('[DEBUG] setPvpTactics called: ' + JSON.stringify(data));
  pvpTacticsEditor.setData(data);
}
export function getMPvpTactics() {
  return mPvpTacticsEditor.getData();
}
export function setMPvpTactics(data) {
  alert('[DEBUG] setMPvpTactics called: ' + JSON.stringify(data));
  mPvpTacticsEditor.setData(data);
}

// Export helpers for persistent settings
export function getHomunculusTactics() {
  return homTacticsEditor.getData();
}
export function setHomunculusTactics(data) {
  alert('[DEBUG] setHomunculusTactics called: ' + JSON.stringify(data));
  homTacticsEditor.setData(data);
}
export function getMercenaryTactics() {
  return merTacticsEditor.getData();
}
export function setMercenaryTactics(data) {
  alert('[DEBUG] setMercenaryTactics called: ' + JSON.stringify(data));
  merTacticsEditor.setData(data);
}
