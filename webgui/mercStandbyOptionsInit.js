// mercStandbyOptionsInit.js
// Handles initialization for Mercenary Standby Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const mercStandbyOptions = [
  { name: 'EnableStandby', value: true, desc: 'Enable standby mode.' },
  { name: 'StandbyDistance', value: 8, desc: 'Distance to maintain while in standby.' },
  { name: 'StandbySkillList', value: 'None', desc: 'Skills to use while in standby.' },
  { name: 'StandbyPriority', value: 'Idle>Owner>Party', desc: 'Priority order for standby.' },
  { name: 'StandbyTimeout', value: 30000, desc: 'Timeout before leaving standby (ms).' }
];

function filterMercStandbyOptions() {
  const val = document.getElementById('mercStandbySearchBox').value;
  renderOptionsTable(mercStandbyOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercStandbyOptionsTableBody');
}

renderOptionsTable(mercStandbyOptions, 'mercStandbyOptionsTableBody');
setupOptionTableEvents(mercStandbyOptions, 'mercStandbyOptionsTableBody', 'mercStandbyOptionDescription');
document.getElementById('mercStandbySearchBox').oninput = filterMercStandbyOptions;
