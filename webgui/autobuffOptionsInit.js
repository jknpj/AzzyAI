// autobuffOptionsInit.js
// Handles initialization for Homunculus Autobuff Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const autobuffOptions = [
  { name: 'AutoBuffOwner', value: true, desc: 'Automatically buff owner.' },
  { name: 'AutoBuffSelf', value: true, desc: 'Automatically buff self.' },
  { name: 'BuffInterval', value: 60000, desc: 'Interval between buffs (ms).' },
  { name: 'BuffPriority', value: 'Owner>Self', desc: 'Priority order for buffing.' },
  { name: 'BuffSkillList', value: 'All', desc: 'Skills to use for buffing.' }
];

function filterAutobuffOptions() {
  const val = document.getElementById('autobuffSearchBox').value;
  renderOptionsTable(autobuffOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'autobuffOptionsTableBody');
}

renderOptionsTable(autobuffOptions, 'autobuffOptionsTableBody');
setupOptionTableEvents(autobuffOptions, 'autobuffOptionsTableBody', 'autobuffOptionDescription');
document.getElementById('autobuffSearchBox').oninput = filterAutobuffOptions;
