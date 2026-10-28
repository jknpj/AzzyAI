// homSkillListOptionsInit.js
// Handles initialization for Homunculus Skill List Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const homSkillListOptions = [
  { name: 'SkillA', value: true, desc: 'Enable SkillA for homunculus.' },
  { name: 'SkillB', value: false, desc: 'Enable SkillB for homunculus.' },
  { name: 'SkillC', value: true, desc: 'Enable SkillC for homunculus.' },
  { name: 'SkillD', value: false, desc: 'Enable SkillD for homunculus.' },
  { name: 'SkillE', value: true, desc: 'Enable SkillE for homunculus.' }
];

function filterHomSkillListOptions() {
  const val = document.getElementById('homSkillListSearchBox').value;
  renderOptionsTable(homSkillListOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'homSkillListOptionsTableBody');
}

renderOptionsTable(homSkillListOptions, 'homSkillListOptionsTableBody');
setupOptionTableEvents(homSkillListOptions, 'homSkillListOptionsTableBody', 'homSkillListOptionDescription');
document.getElementById('homSkillListSearchBox').oninput = filterHomSkillListOptions;
