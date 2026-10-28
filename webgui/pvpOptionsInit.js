// pvpOptionsInit.js
// Handles initialization for Homunculus PVP Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const pvpOptions = [
  { name: 'EnablePVP', value: false, desc: 'Enable PVP mode.' },
  { name: 'PVPAttackOwner', value: false, desc: 'Attack owner in PVP.' },
  { name: 'PVPAttackParty', value: false, desc: 'Attack party members in PVP.' },
  { name: 'PVPAttackOthers', value: true, desc: 'Attack other players in PVP.' },
  { name: 'PVPMode', value: 'Aggressive', desc: 'PVP mode setting.' }
];

function filterPvpOptions() {
  const val = document.getElementById('pvpSearchBox').value;
  renderOptionsTable(pvpOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'pvpOptionsTableBody');
}

renderOptionsTable(pvpOptions, 'pvpOptionsTableBody');
setupOptionTableEvents(pvpOptions, 'pvpOptionsTableBody', 'pvpOptionDescription');
document.getElementById('pvpSearchBox').oninput = filterPvpOptions;
