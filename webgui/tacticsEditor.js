// Handles all tactics editor logic for Homunculus, Mercenary, PVP, M_PvpTact
export function TacticsEditor(config) {
  this.data = config.data;
  this.selected = null;
  this.listId = config.listId;
  this.formId = config.formId;
  this.searchId = config.searchId;
  this.fields = config.fields;
  this.typeOptions = config.typeOptions;
  this.renderList = function(filter = '') {
    const list = document.getElementById(this.listId);
    list.innerHTML = '';
    this.data.filter(t => t.name.toLowerCase().includes(filter.toLowerCase())).forEach((tact, idx) => {
      const li = document.createElement('li');
      li.className = 'list-group-item tact-list-item' + (this.selected === idx ? ' active' : '');
      li.textContent = tact.name;
      li.onclick = () => this.select(idx);
      list.appendChild(li);
    });
  };
  this.filter = function() {
    const val = document.getElementById(this.searchId).value;
    this.renderList(val);
  };
  this.select = function(idx) {
    this.selected = idx;
    const tact = this.data[idx];
    this.fields.forEach(f => {
      document.getElementById(f.id).value = tact[f.key];
    });
    document.getElementById(this.fields[0].id).focus();
    this.renderList(document.getElementById(this.searchId).value);
  };
  this.save = function() {
    if (this.selected === null) return;
    const updated = {};
    this.fields.forEach(f => {
      updated[f.key] = document.getElementById(f.id).value;
    });
    this.data[this.selected] = updated;
    this.renderList(document.getElementById(this.searchId).value);
  };
  this.add = function() {
    const newTact = {};
    this.fields.forEach(f => {
      newTact[f.key] = f.key === 'type' ? this.typeOptions[0] : '';
    });
    newTact.name = 'New Tactic';
    this.data.push(newTact);
    this.selected = this.data.length - 1;
    this.renderList(document.getElementById(this.searchId).value);
    this.select(this.selected);
  };
  this.delete = function() {
    if (this.selected === null) return;
    this.data.splice(this.selected, 1);
    this.selected = null;
    this.renderList(document.getElementById(this.searchId).value);
    document.getElementById(this.formId).reset();
  };
  this.init = function() {
    document.getElementById(this.searchId).oninput = () => this.filter();
    // Button event binding by explicit IDs
    if (this.formId === 'homTactForm') {
      document.getElementById('homTactSaveBtn').onclick = () => this.save();
      document.getElementById('homTactAddBtn').onclick = () => this.add();
      document.getElementById('homTactDeleteBtn').onclick = () => this.delete();
    } else if (this.formId === 'merTactForm') {
      document.getElementById('merTactSaveBtn').onclick = () => this.save();
      document.getElementById('merTactAddBtn').onclick = () => this.add();
      document.getElementById('merTactDeleteBtn').onclick = () => this.delete();
    } else if (this.formId === 'pvpTactForm') {
      document.getElementById('pvpTactSaveBtn').onclick = () => this.save();
      document.getElementById('pvpTactAddBtn').onclick = () => this.add();
      document.getElementById('pvpTactDeleteBtn').onclick = () => this.delete();
    } else if (this.formId === 'mPvpTactForm') {
      document.getElementById('mPvpTactSaveBtn').onclick = () => this.save();
      document.getElementById('mPvpTactAddBtn').onclick = () => this.add();
      document.getElementById('mPvpTactDeleteBtn').onclick = () => this.delete();
    }
    this.renderList();
  };
  this.getData = function() {
    return this.data.map(t => ({ ...t }));
  };
  this.setData = function(newData) {
    this.data = newData.map(t => ({ ...t }));
    this.selected = null;
    this.renderList(document.getElementById(this.searchId).value);
    document.getElementById(this.formId).reset();
  };
  this.searchId = config.searchId;
  this.fields = config.fields;
  this.typeOptions = config.typeOptions;
  this.renderList = function(filter = '') {
    const list = document.getElementById(this.listId);
    list.innerHTML = '';
    this.data.filter(t => t.name.toLowerCase().includes(filter.toLowerCase())).forEach((tact, idx) => {
      const li = document.createElement('li');
      li.className = 'list-group-item tact-list-item' + (this.selected === idx ? ' active' : '');
      li.textContent = tact.name;
      li.onclick = () => this.select(idx);
      list.appendChild(li);
    });
  };
  this.filter = function() {
    const val = document.getElementById(this.searchId).value;
    this.renderList(val);
  };
  this.select = function(idx) {
    this.selected = idx;
    const tact = this.data[idx];
    this.fields.forEach(f => {
      document.getElementById(f.id).value = tact[f.key];
    });
    document.getElementById(this.fields[0].id).focus();
    this.renderList(document.getElementById(this.searchId).value);
  };
  this.save = function() {
    if (this.selected === null) return;
    const updated = {};
    this.fields.forEach(f => {
      updated[f.key] = document.getElementById(f.id).value;
    });
    this.data[this.selected] = updated;
    this.renderList(document.getElementById(this.searchId).value);
  };
  this.add = function() {
    const newTact = {};
    this.fields.forEach(f => {
      newTact[f.key] = f.key === 'type' ? this.typeOptions[0] : '';
    });
    newTact.name = 'New Tactic';
    this.data.push(newTact);
    this.selected = this.data.length - 1;
    this.renderList(document.getElementById(this.searchId).value);
    this.select(this.selected);
  };
  this.delete = function() {
    if (this.selected === null) return;
    this.data.splice(this.selected, 1);
    this.selected = null;
    this.renderList(document.getElementById(this.searchId).value);
    document.getElementById(this.formId).reset();
  };
  this.init = function() {
    document.getElementById(this.searchId).oninput = () => this.filter();
    // Button event binding by explicit IDs
    if (this.formId === 'homTactForm') {
      document.getElementById('homTactSaveBtn').onclick = () => this.save();
      document.getElementById('homTactAddBtn').onclick = () => this.add();
      document.getElementById('homTactDeleteBtn').onclick = () => this.delete();
    } else if (this.formId === 'merTactForm') {
      document.getElementById('merTactSaveBtn').onclick = () => this.save();
      document.getElementById('merTactAddBtn').onclick = () => this.add();
      document.getElementById('merTactDeleteBtn').onclick = () => this.delete();
    } else if (this.formId === 'pvpTactForm') {
      document.getElementById('pvpTactSaveBtn').onclick = () => this.save();
      document.getElementById('pvpTactAddBtn').onclick = () => this.add();
      document.getElementById('pvpTactDeleteBtn').onclick = () => this.delete();
    } else if (this.formId === 'mPvpTactForm') {
      document.getElementById('mPvpTactSaveBtn').onclick = () => this.save();
      document.getElementById('mPvpTactAddBtn').onclick = () => this.add();
      document.getElementById('mPvpTactDeleteBtn').onclick = () => this.delete();
    }
    this.renderList();
  };
}
