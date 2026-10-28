// Handles rendering and logic for Homunculus and Mercenary settings tables
export function renderOptionsTable(options, tableBodyId) {
  const tbody = document.getElementById(tableBodyId);
  tbody.innerHTML = '';
  options.forEach(opt => {
    const tr = document.createElement('tr');
    tr.innerHTML = `<td><span class="option-name" data-desc="${opt.desc}">${opt.name}</span></td><td><input class="form-control form-control-sm option-value" type="text" value="${opt.value}" data-name="${opt.name}"></td>`;
    tbody.appendChild(tr);
  });
}

export function setupOptionTableEvents(options, tableBodyId, descBoxId) {
  document.getElementById(tableBodyId).addEventListener('click', function(e) {
    if (e.target.classList.contains('option-name')) {
      document.getElementById(descBoxId).value = e.target.getAttribute('data-desc');
    }
  });
  document.getElementById(tableBodyId).addEventListener('focusin', function(e) {
    if (e.target.classList.contains('option-value')) {
      const opt = options.find(o => o.name === e.target.getAttribute('data-name'));
      document.getElementById(descBoxId).value = opt ? opt.desc : '';
    }
  });
}
