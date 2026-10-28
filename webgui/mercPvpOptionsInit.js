// mercPvpOptionsInit.js
// Handles initialization for Mercenary PVP Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const mercPvpOptions = [
  { name: 'EnablePVP', value: false, desc: 'Enable PVP mode.' },
  { name: 'PVPAttackOwner', value: false, desc: 'Attack owner in PVP.' },
  { name: 'PVPAttackParty', value: false, desc: 'Attack party members in PVP.' },
  { name: 'PVPAttackOthers', value: true, desc: 'Attack other players in PVP.' },
  { name: 'PVPMode', value: 'Aggressive', desc: 'PVP mode setting.' }
];

function filterMercPvpOptions() {
  const val = document.getElementById('mercPvpSearchBox').value;
  renderOptionsTable(mercPvpOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercPvpOptionsTableBody');
}

renderOptionsTable(mercPvpOptions, 'mercPvpOptionsTableBody');
setupOptionTableEvents(mercPvpOptions, 'mercPvpOptionsTableBody', 'mercPvpOptionDescription');
document.getElementById('mercPvpSearchBox').oninput = filterMercPvpOptions;
