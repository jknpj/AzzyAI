// walkOptionsInit.js
// Handles initialization for Homunculus Walk/Follow Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const walkOptions = [
  { name: 'FollowOwner', value: true, desc: 'Follow owner when idle.' },
  { name: 'FollowDistance', value: 5, desc: 'Distance to maintain from owner.' },
  { name: 'AvoidObstacles', value: true, desc: 'Avoid obstacles when following.' },
  { name: 'AutoTeleport', value: false, desc: 'Teleport if stuck while following.' },
  { name: 'WalkSpeed', value: 'Normal', desc: 'Walking speed setting.' }
];

function filterWalkOptions() {
  const val = document.getElementById('walkSearchBox').value;
  renderOptionsTable(walkOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'walkOptionsTableBody');
}

renderOptionsTable(walkOptions, 'walkOptionsTableBody');
setupOptionTableEvents(walkOptions, 'walkOptionsTableBody', 'walkOptionDescription');
document.getElementById('walkSearchBox').oninput = filterWalkOptions;
