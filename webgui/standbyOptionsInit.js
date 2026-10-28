// standbyOptionsInit.js
// Handles initialization for Homunculus Standby Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const standbyOptions = [
  { name: 'EnableStandby', value: true, desc: 'Enable standby mode.' },
  { name: 'StandbyDistance', value: 8, desc: 'Distance to maintain while in standby.' },
  { name: 'StandbySkillList', value: 'None', desc: 'Skills to use while in standby.' },
  { name: 'StandbyPriority', value: 'Idle>Owner>Party', desc: 'Priority order for standby.' },
  { name: 'StandbyTimeout', value: 30000, desc: 'Timeout before leaving standby (ms).' }
];

function filterStandbyOptions() {
  const val = document.getElementById('standbySearchBox').value;
  renderOptionsTable(standbyOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'standbyOptionsTableBody');
}

renderOptionsTable(standbyOptions, 'standbyOptionsTableBody');
setupOptionTableEvents(standbyOptions, 'standbyOptionsTableBody', 'standbyOptionDescription');
document.getElementById('standbySearchBox').oninput = filterStandbyOptions;
