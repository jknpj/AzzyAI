// friendingOptionsInit.js
// Handles initialization for Homunculus Friending Options tab
import { renderOptionsTable, setupOptionTableEvents } from './optionsTable.js';

const friendingOptions = [
  { name: 'AutoFriendOwner', value: true, desc: 'Automatically friend owner.' },
  { name: 'AutoFriendParty', value: true, desc: 'Automatically friend party members.' },
  { name: 'FriendDistance', value: 10, desc: 'Distance to maintain from friends.' },
  { name: 'FriendPriority', value: 'Owner>Party', desc: 'Priority order for friending.' },
  { name: 'FriendSkillList', value: 'All', desc: 'Skills to use for friending.' }
];

function filterFriendingOptions() {
  const val = document.getElementById('friendingSearchBox').value;
  renderOptionsTable(friendingOptions.filter(opt => opt.name.toLowerCase().includes(val.toLowerCase())), 'friendingOptionsTableBody');
}

renderOptionsTable(friendingOptions, 'friendingOptionsTableBody');
setupOptionTableEvents(friendingOptions, 'friendingOptionsTableBody', 'friendingOptionDescription');
document.getElementById('friendingSearchBox').oninput = filterFriendingOptions;
