// webgui_templates_check.js
// Checks that the web GUI's embedded H_Config.lua and M_Config.lua are the shipped preset.
//
// The web page starts a new configuration from webgui/templates.js. When that copy drifts from
// USER_AI/H_Config.lua / M_Config.lua, a player who configures through the web page gets different
// defaults from the ones the AI ships with (LavaSlideMode was 1 in the copy and 0 in the preset, so
// his Dieter laid the lava at the owner's feet instead of on the monsters).
//
// Usage (Node 18+; the web GUI CI job already runs node):
//     node tools/webgui_templates_check.js
'use strict';

const fs = require('fs');
const path = require('path');

const REPO = path.join(__dirname, '..');

function normalize(text) {
  return text.replace(/\r\n/g, '\n').replace(/^\n+|\n+$/g, '');
}

function embedded(templatesJs, name) {
  const pattern = new RegExp("'" + name.replace(/[.]/g, '\\.') + "': String\\.raw`([\\s\\S]*?)`");
  const match = pattern.exec(templatesJs);
  return match ? match[1] : null;
}

function main() {
  const templatesPath = path.join(REPO, 'webgui', 'templates.js');
  if (!fs.existsSync(templatesPath)) {
    console.log('webgui/templates.js not found (submodule not checked out), skipping');
    return 0;
  }
  const templatesJs = fs.readFileSync(templatesPath, 'utf8');
  let failed = false;
  for (const name of ['H_Config.lua', 'M_Config.lua']) {
    const shipped = normalize(fs.readFileSync(path.join(REPO, 'USER_AI', name), 'utf8'));
    const body = embedded(templatesJs, name);
    if (body === null) {
      console.log("ERROR: webgui/templates.js has no '" + name + "' template");
      failed = true;
      continue;
    }
    if (normalize(body) === shipped) {
      console.log(name + ': matches USER_AI/' + name);
      continue;
    }
    failed = true;
    const shippedLines = shipped.split('\n');
    const bodyLines = normalize(body).split('\n');
    console.log('ERROR: the web GUI copy of ' + name + ' differs from USER_AI/' + name);
    for (let i = 0; i < Math.max(shippedLines.length, bodyLines.length); i++) {
      const a = i < shippedLines.length ? shippedLines[i] : '<missing>';
      const b = i < bodyLines.length ? bodyLines[i] : '<missing>';
      if (a !== b) {
        console.log('  line ' + (i + 1) + ': USER_AI: ' + a);
        console.log('  line ' + (i + 1) + ': webgui : ' + b);
        break;
      }
    }
    console.log('  copy USER_AI/' + name + ' into webgui/templates.js (String.raw body) to fix it');
  }
  return failed ? 1 : 0;
}

process.exit(main());
