// berserkOptionsInit.js
// Handles initialization for Homunculus Berserk Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const berserkOptions = [
  { name: 'EnableBerserk', value: false, desc: 'Enable berserk mode.' },
  { name: 'BerserkHPThreshold', value: 20, desc: 'HP threshold to trigger berserk mode.' },
  { name: 'BerserkSkill', value: 'Berserk', desc: 'Skill to use in berserk mode.' },
  { name: 'BerserkDuration', value: 10000, desc: 'Duration of berserk mode (ms).' },
  { name: 'BerserkCooldown', value: 60000, desc: 'Cooldown between berserk activations (ms).' }
];

function filterBerserkOptions() {
  const val = document.getElementById('berserkSearchBox').value;
  renderOptionsTable(berserkOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'berserkOptionsTableBody');
}

renderOptionsTable(berserkOptions, 'berserkOptionsTableBody');
setupOptionTableEvents(berserkOptions, 'berserkOptionsTableBody', 'berserkOptionDescription');
document.getElementById('berserkSearchBox').oninput = filterBerserkOptions;
