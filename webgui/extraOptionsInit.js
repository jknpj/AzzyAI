// extraOptionsInit.js
// Handles initialization for Extra tab options (placeholder for future expansion)
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

// Example extra options data (expand as needed)
const extraOptions = [
  { name: 'DebugMode', value: false, desc: 'Enable debug mode for advanced logging.' },
  { name: 'ShowAdvanced', value: false, desc: 'Show advanced settings.' }
];

function filterExtraOptions() {
  const val = document.getElementById('extraSearchBox').value;
  renderOptionsTable(extraOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'extraOptionsTableBody');
}

renderOptionsTable(extraOptions, 'extraOptionsTableBody');
setupOptionTableEvents(extraOptions, 'extraOptionsTableBody', 'extraOptionDescription');
document.getElementById('extraSearchBox').oninput = filterExtraOptions;
