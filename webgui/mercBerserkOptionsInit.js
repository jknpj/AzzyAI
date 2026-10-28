// mercBerserkOptionsInit.js
// Handles initialization for Mercenary Berserk Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const mercBerserkOptions = [
  { name: 'EnableBerserk', value: false, desc: 'Enable berserk mode.' },
  { name: 'BerserkHPThreshold', value: 20, desc: 'HP threshold to trigger berserk mode.' },
  { name: 'BerserkSkill', value: 'Berserk', desc: 'Skill to use in berserk mode.' },
  { name: 'BerserkDuration', value: 10000, desc: 'Duration of berserk mode (ms).' },
  { name: 'BerserkCooldown', value: 60000, desc: 'Cooldown between berserk activations (ms).' }
];

function filterMercBerserkOptions() {
  const val = document.getElementById('mercBerserkSearchBox').value;
  renderOptionsTable(mercBerserkOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercBerserkOptionsTableBody');
}

renderOptionsTable(mercBerserkOptions, 'mercBerserkOptionsTableBody');
setupOptionTableEvents(mercBerserkOptions, 'mercBerserkOptionsTableBody', 'mercBerserkOptionDescription');
document.getElementById('mercBerserkSearchBox').oninput = filterMercBerserkOptions;
