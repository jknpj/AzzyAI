// autoskillOptionsInit.js
// Handles initialization for Homunculus AutoSkill Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const autoskillOptions = [
  { name: 'UseAutoHeal', value: true, desc: 'Automatically use healing skills when needed.' },
  { name: 'UseAutoBuff', value: true, desc: 'Automatically use buff skills.' },
  { name: 'UseAutoAttack', value: true, desc: 'Automatically use attack skills.' },
  { name: 'AutoSkillDelay', value: 500, desc: 'Delay between auto skill uses (ms).' },
  { name: 'AutoSkillPriority', value: 'Heal>Buff>Attack', desc: 'Priority order for auto skills.' }
];

function filterAutoskillOptions() {
  const val = document.getElementById('autoskillSearchBox').value;
  renderOptionsTable(autoskillOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'autoskillOptionsTableBody');
}

renderOptionsTable(autoskillOptions, 'autoskillOptionsTableBody');
setupOptionTableEvents(autoskillOptions, 'autoskillOptionsTableBody', 'autoskillOptionDescription');
document.getElementById('autoskillSearchBox').oninput = filterAutoskillOptions;
