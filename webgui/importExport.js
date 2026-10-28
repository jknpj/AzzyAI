// importExport.js
// Handles import/export of settings and tactics for AzzyAIConfig Web GUI

export function downloadJSON(filename, data) {
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

export function uploadJSON(callback) {
  const input = document.createElement('input');
  input.type = 'file';
  input.accept = '.json,application/json';
  input.onchange = e => {
    const file = e.target.files[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = evt => {
      try {
        const data = JSON.parse(evt.target.result);
        callback(data);
      } catch (err) {
        alert('Invalid JSON file.');
      }
    };
    reader.readAsText(file);
  };
  input.click();
}

// Persistent settings using localStorage
export function saveSettings(key, data) {
  try {
    localStorage.setItem(key, JSON.stringify(data));
  } catch (e) {
    alert('Failed to save settings: ' + e.message);
  }
}

export function loadSettings(key) {
  try {
    const data = localStorage.getItem(key);
    return data ? JSON.parse(data) : null;
  } catch (e) {
    alert('Failed to load settings: ' + e.message);
    return null;
  }
}

export function clearSettings(key) {
  try {
    localStorage.removeItem(key);
  } catch (e) {
    alert('Failed to clear settings: ' + e.message);
  }
}

// Example usage (to be called from menu event handlers):
// saveSettings('homunculus_settings', homunculusSettingsObject);
// const settings = loadSettings('homunculus_settings');
// clearSettings('homunculus_settings');
// downloadJSON('homunculus_settings.json', homunculusSettingsObject);
// uploadJSON(data => { /* handle imported data */ });
