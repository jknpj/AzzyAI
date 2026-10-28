// mercKitingOptionsInit.js
// Handles initialization for Mercenary Kiting Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const mercKitingOptions = [
  { name: 'EnableKiting', value: true, desc: 'Enable kiting mode.' },
  { name: 'KitingDistance', value: 6, desc: 'Distance to maintain while kiting.' },
  { name: 'KitingSkillList', value: 'All', desc: 'Skills to use for kiting.' },
  { name: 'KitingPriority', value: 'Owner>Self', desc: 'Priority order for kiting.' },
  { name: 'KitingTimeout', value: 20000, desc: 'Timeout before stopping kiting (ms).' }
];

function filterMercKitingOptions() {
  const val = document.getElementById('mercKitingSearchBox').value;
  renderOptionsTable(mercKitingOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercKitingOptionsTableBody');
}

renderOptionsTable(mercKitingOptions, 'mercKitingOptionsTableBody');
setupOptionTableEvents(mercKitingOptions, 'mercKitingOptionsTableBody', 'mercKitingOptionDescription');
document.getElementById('mercKitingSearchBox').oninput = filterMercKitingOptions;
