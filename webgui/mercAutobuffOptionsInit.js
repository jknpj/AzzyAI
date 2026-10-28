// mercAutobuffOptionsInit.js
// Handles initialization for Mercenary Autobuff Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const mercAutobuffOptions = [
  { name: 'AutoBuffOwner', value: true, desc: 'Automatically buff owner.' },
  { name: 'AutoBuffSelf', value: true, desc: 'Automatically buff self.' },
  { name: 'BuffInterval', value: 60000, desc: 'Interval between buffs (ms).' },
  { name: 'BuffPriority', value: 'Owner>Self', desc: 'Priority order for buffing.' },
  { name: 'BuffSkillList', value: 'All', desc: 'Skills to use for buffing.' }
];

function filterMercAutobuffOptions() {
  const val = document.getElementById('mercAutobuffSearchBox').value;
  renderOptionsTable(mercAutobuffOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercAutobuffOptionsTableBody');
}

renderOptionsTable(mercAutobuffOptions, 'mercAutobuffOptionsTableBody');
setupOptionTableEvents(mercAutobuffOptions, 'mercAutobuffOptionsTableBody', 'mercAutobuffOptionDescription');
document.getElementById('mercAutobuffSearchBox').oninput = filterMercAutobuffOptions;
