// webgui-data.js
// Dumps the web GUI's option schema (enums + options, with descriptions), its pt/es translations
// (UI strings and setting descriptions), the skill names/descriptions/stats and the monster/map
// database to JSON for the Rust GUI, so the two front ends share one source of truth instead of
// hand-copied data.
//
//   node tools/webgui-data.js            # write tools/azzy-native/data/
//   node tools/webgui-data.js --check    # compare without writing (CI)
'use strict';

const fs = require('fs');
const path = require('path');
const vm = require('vm');

const REPO = path.join(__dirname, '..');
const WEBGUI = path.join(REPO, 'webgui');
const OUT_DIR = path.join(REPO, 'tools', 'azzy-native', 'data');
const FILES = ['homSchema.js', 'merSchema.js', 'mobdata.js'];
const I18N = ['i18n.strings.js', 'i18n.desc.pt.js', 'i18n.desc.es.js'];
// skilldata.js must run first: skills.js reads window.AzzySkillData when it loads.
const SKILLS = ['skilldata.js', 'skills.js'];

function load() {
  const window = {};
  const sandbox = { window, console };
  vm.createContext(sandbox);
  for (const name of FILES) {
    const file = path.join(WEBGUI, name);
    vm.runInContext(fs.readFileSync(file, 'utf8'), sandbox, { filename: file });
  }
  for (const name of SKILLS) {
    const file = path.join(WEBGUI, name);
    if (!fs.existsSync(file)) throw new Error(`${name} not found in webgui/`);
    vm.runInContext(fs.readFileSync(file, 'utf8'), sandbox, { filename: file });
  }
  // The translations are plain objects on window, like the schema.
  for (const name of I18N) {
    const file = path.join(WEBGUI, name);
    if (!fs.existsSync(file)) throw new Error(`${name} not found in webgui/`);
    vm.runInContext(fs.readFileSync(file, 'utf8'), sandbox, { filename: file });
  }
  return {
    hom: { enums: window.AzzyHomEnums || {}, schema: window.AzzyHomSchema || [] },
    mer: { enums: window.AzzyMerEnums || {}, schema: window.AzzyMerSchema || [] },
    mobs: window.AzzyMobData || null,
    i18n: { strings: window.AzzyI18nStrings || {}, desc: window.AzzyI18nDesc || {} },
    skills: skill_data(window.AzzySkills && window.AzzySkills.list, window.AzzySkillData)
  };
}

// Official names, in-game descriptions (with the client's ^RRGGBB colours) and the LATAM numbers at
// max level, merged by skill id so the native GUI does not hand-copy them.
function skill_data(list, data) {
  const out = {};
  const fallback = { en: '', pt: '', es: '' };
  for (const skill of list || []) {
    const official = (data || {})[skill.id] || {};
    out[skill.id] = {
      homun: skill.homun,
      maxlv: skill.maxlv,
      stats: skill.stats,
      match: skill.match,
      name: {
        en: (official.name && official.name.en) || skill.name.en,
        pt: (official.name && official.name.pt) || skill.name.pt || skill.name.en,
        es: (official.name && official.name.es) || skill.name.es || skill.name.en
      },
      desc: {
        en: (official.desc && official.desc.en) || fallback.en,
        pt: (official.desc && official.desc.pt) || fallback.pt,
        es: (official.desc && official.desc.es) || fallback.es
      }
    };
  }
  return out;
}

// Stable formatting: sorted object keys, two-space indent. Same bytes on every run and platform.
function dump(value) {
  const sorted = (v) => {
    if (Array.isArray(v)) return v.map(sorted);
    if (v && typeof v === 'object') {
      const out = {};
      for (const key of Object.keys(v).sort()) out[key] = sorted(v[key]);
      return out;
    }
    return v;
  };
  return JSON.stringify(sorted(value), null, 2) + '\n';
}

function main() {
  if (!fs.existsSync(WEBGUI)) {
    console.log('webgui/ not found (submodule not checked out), skipping');
    return 0;
  }
  const data = load();
  const outputs = [
    ['options.json', dump({ hom: data.hom, mer: data.mer }), `${data.hom.schema.length} homunculus, ${data.mer.schema.length} mercenary options`],
    ['i18n.json', dump(data.i18n), `${Object.keys(data.i18n.strings).join('/')} UI strings and setting descriptions`],
    ['skilldata.json', dump(data.skills), `${Object.keys(data.skills).length} skills with names, descriptions and stats`]
  ];
  if (data.mobs) {
    const count = Object.keys(data.mobs.mobs || {}).length;
    outputs.push(['mobs.json', dump(data.mobs), `${count} monsters, ${Object.keys(data.mobs.maps || {}).length} maps`]);
  }
  const check = process.argv.includes('--check');
  if (check) {
    for (const [name, text] of outputs) {
      const target = path.join(OUT_DIR, name);
      const current = fs.existsSync(target) ? fs.readFileSync(target, 'utf8') : '';
      if (current !== text) {
        console.log(`ERROR: tools/azzy-native/data/${name} is stale`);
        console.log('  run: node tools/webgui-data.js');
        return 1;
      }
      console.log(`${name} matches the web GUI data`);
    }
    return 0;
  }
  fs.mkdirSync(OUT_DIR, { recursive: true });
  for (const [name, text, summary] of outputs) {
    const target = path.join(OUT_DIR, name);
    fs.writeFileSync(target, text);
    console.log(`wrote ${path.relative(REPO, target)}: ${summary}`);
  }
  return 0;
}

process.exit(main());
