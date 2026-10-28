// kitingOptionsInit.js
// Handles initialization for Homunculus Kiting Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const kitingOptions = [
  { name: 'EnableKiting', value: true, desc: 'Enable kiting mode.' },
  { name: 'KitingDistance', value: 6, desc: 'Distance to maintain while kiting.' },
  { name: 'KitingSkillList', value: 'All', desc: 'Skills to use for kiting.' },
  { name: 'KitingPriority', value: 'Owner>Self', desc: 'Priority order for kiting.' },
  { name: 'KitingTimeout', value: 20000, desc: 'Timeout before stopping kiting (ms).' }
];

function filterKitingOptions() {
  const val = document.getElementById('kitingSearchBox').value;
  renderOptionsTable(kitingOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'kitingOptionsTableBody');
}

renderOptionsTable(kitingOptions, 'kitingOptionsTableBody');
setupOptionTableEvents(kitingOptions, 'kitingOptionsTableBody', 'kitingOptionDescription');
document.getElementById('kitingSearchBox').oninput = filterKitingOptions;
