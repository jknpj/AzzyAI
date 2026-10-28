// mercWalkOptionsInit.js
// Handles initialization for Mercenary Walk/Follow Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const mercWalkOptions = [
  { name: 'FollowOwner', value: true, desc: 'Follow owner when idle.' },
  { name: 'FollowDistance', value: 5, desc: 'Distance to maintain from owner.' },
  { name: 'AvoidObstacles', value: true, desc: 'Avoid obstacles when following.' },
  { name: 'AutoTeleport', value: false, desc: 'Teleport if stuck while following.' },
  { name: 'WalkSpeed', value: 'Normal', desc: 'Walking speed setting.' }
];

function filterMercWalkOptions() {
  const val = document.getElementById('mercWalkSearchBox').value;
  renderOptionsTable(mercWalkOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercWalkOptionsTableBody');
}

renderOptionsTable(mercWalkOptions, 'mercWalkOptionsTableBody');
setupOptionTableEvents(mercWalkOptions, 'mercWalkOptionsTableBody', 'mercWalkOptionDescription');
document.getElementById('mercWalkSearchBox').oninput = filterMercWalkOptions;
