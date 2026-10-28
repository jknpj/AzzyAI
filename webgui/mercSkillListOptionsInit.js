// mercSkillListOptionsInit.js
// Handles initialization for Mercenary Skill List Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const mercSkillListOptions = [
  { name: 'Skill1', value: true, desc: 'Enable Skill1 for mercenary.' },
  { name: 'Skill2', value: false, desc: 'Enable Skill2 for mercenary.' },
  { name: 'Skill3', value: true, desc: 'Enable Skill3 for mercenary.' },
  { name: 'Skill4', value: false, desc: 'Enable Skill4 for mercenary.' },
  { name: 'Skill5', value: true, desc: 'Enable Skill5 for mercenary.' }
];

function filterMercSkillListOptions() {
  const val = document.getElementById('mercSkillListSearchBox').value;
  renderOptionsTable(mercSkillListOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercSkillListOptionsTableBody');
}

renderOptionsTable(mercSkillListOptions, 'mercSkillListOptionsTableBody');
setupOptionTableEvents(mercSkillListOptions, 'mercSkillListOptionsTableBody', 'mercSkillListOptionDescription');
document.getElementById('mercSkillListSearchBox').oninput = filterMercSkillListOptions;
