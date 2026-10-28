// mercFriendingOptionsInit.js
// Handles initialization for Mercenary Friending Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const mercFriendingOptions = [
  { name: 'AutoFriendOwner', value: true, desc: 'Automatically friend owner.' },
  { name: 'AutoFriendParty', value: true, desc: 'Automatically friend party members.' },
  { name: 'FriendDistance', value: 10, desc: 'Distance to maintain from friends.' },
  { name: 'FriendPriority', value: 'Owner>Party', desc: 'Priority order for friending.' },
  { name: 'FriendSkillList', value: 'All', desc: 'Skills to use for friending.' }
];

function filterMercFriendingOptions() {
  const val = document.getElementById('mercFriendingSearchBox').value;
  renderOptionsTable(mercFriendingOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'mercFriendingOptionsTableBody');
}

renderOptionsTable(mercFriendingOptions, 'mercFriendingOptionsTableBody');
setupOptionTableEvents(mercFriendingOptions, 'mercFriendingOptionsTableBody', 'mercFriendingOptionDescription');
document.getElementById('mercFriendingSearchBox').oninput = filterMercFriendingOptions;
