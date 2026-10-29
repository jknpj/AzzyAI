// tactics.js
// Parsing/serialization and option definitions for H_Tactics.lua, M_Tactics.lua,
// H_PVP_Tact.lua and M_PVP_Tact.lua, mirroring the C# configuration GUI.
(function () {
  'use strict';

  // Field indexes inside MyTact[ID]={...}
  var F = { BASIC: 0, SKILL: 1, KITE: 2, CAST: 3, PUSH: 4, DEBUFF: 5, CLASS: 6, RESCUE: 7, SP: 8, SNIPE: 9, KS: 10, WEIGHT: 11, CHASE: 12 };

  var BASIC = [
    ['Tank', 'TACT_TANK'],
    ['Ignore', 'TACT_IGNORE'],
    ['Attack (low)', 'TACT_ATTACK_L'],
    ['Attack (medium)', 'TACT_ATTACK_M'],
    ['Attack (high)', 'TACT_ATTACK_H'],
    ['React (low)', 'TACT_REACT_L'],
    ['React (medium)', 'TACT_REACT_M'],
    ['React (high)', 'TACT_REACT_H'],
    ['React (self)', 'TACT_REACT_SELF'],
    ['Snipe (low)', 'TACT_SNIPE_L'],
    ['Snipe (medium)', 'TACT_SNIPE_M'],
    ['Snipe (high)', 'TACT_SNIPE_H'],
    ['Attack (low) React (medium)', 'TACT_ATK_L_REACT_M'],
    ['Attack (last)', 'TACT_ATTACK_LAST'],
    ['Attack (top)', 'TACT_ATTACK_TOP']
  ];

  var KITE = [['Never', 'KITE_NEVER'], ['React', 'KITE_REACT'], ['Always', 'KITE_ALWAYS']];
  var KS = [['Never', 'KS_NEVER'], ['Always', 'KS_ALWAYS'], ['Polite', 'KS_POLITE']];
  var CHASE = [['Normal', 'CHASE_NORMAL'], ['Always', 'CHASE_ALWAYS'], ['Never', 'CHASE_NEVER'], ['Pause for SP', 'CHASE_CLEVER']];
  var PUSH = [['Never', 'PUSH_NEVER'], ['Off self', 'PUSH_SELF'], ['Off self + friend', 'PUSH_FRIEND']];

  var HOM_CAST = [
    ['No', 'CAST_PASSIVE'],
    ['React to casts', 'CAST_REACT'],
    ['React w/Any', 'CAST_REACT_ANY'],
    ['React w/Breeze', 'CAST_REACT_BREEZE'],
    ['React w/Old', 'CAST_REACT_MAIN'],
    ['React w/S', 'CAST_REACT_S'],
    ['React w/Mob', 'CAST_REACT_MOB'],
    ['React w/Debuff', 'CAST_REACT_DEBUFF'],
    ['React w/Minions', 'CAST_REACT_MINION']
  ];
  var MER_CAST = [
    ['No', 'CAST_PASSIVE'],
    ['React to casts', 'CAST_REACT'],
    ['React w/Any', 'CAST_REACT_ANY'],
    ['React w/LexDiv', 'CAST_REACT_LEXDIV'],
    ['React w/Main', 'CAST_REACT_MAIN'],
    ['React w/Mob', 'CAST_REACT_MOB'],
    ['React w/Provoke', 'CAST_REACT_PROVOKE'],
    ['React w/Sandman', 'CAST_REACT_SANDMAN'],
    ['React w/Freeze', 'CAST_REACT_FREEZE'],
    ['React w/DecAgi', 'CAST_REACT_DECAGI'],
    ['React w/Debuff', 'CAST_REACT_DEBUFF'],
    ['React w/Crash', 'CAST_REACT_CRASH']
  ];

  var HOM_CLASS = [
    ['Any skill', 'CLASS_BOTH'],
    ['Non-Homun S skills', 'CLASS_OLD'],
    ['Homun S skills', 'CLASS_S'],
    ['AoE attacks', 'CLASS_MOB'],
    ['Combo (first skill)', 'CLASS_COMBO_1'],
    ['Combo (full combos)', 'CLASS_COMBO_2'],
    ['Summon minions', 'CLASS_MINION'],
    ['Grapple (Tinder Breaker)', 'CLASS_GRAPPLE'],
    ['Grapple (CBC)', 'CLASS_GRAPPLE_1'],
    ['Grapple (EQC)', 'CLASS_GRAPPLE_2'],
    ['Minions + Pre-S skill', 'CLASS_MIN_OLD'],
    ['Minions + Homun S', 'CLASS_MIN_S']
  ];
  var MER_CLASS = [['All skills', 'CLASS_BOTH'], ['Mob skills', 'CLASS_MOB']];

  var HOM_RESCUE = [
    ['Never', 'RESCUE_NEVER'],
    ['Friends', 'RESCUE_FRIEND'],
    ["Owner's merc", 'RESCUE_RETAINER'],
    ['Self (homunculus)', 'RESCUE_SELF'],
    ['Owner', 'RESCUE_OWNER'],
    ['All of the above', 'RESCUE_ALL']
  ];
  var MER_RESCUE = [
    ['Never', 'RESCUE_NEVER'],
    ['Friends', 'RESCUE_FRIEND'],
    ["Owner's homun", 'RESCUE_RETAINER'],
    ['Self (mercenary)', 'RESCUE_SELF'],
    ['Owner', 'RESCUE_OWNER'],
    ['All of the above', 'RESCUE_ALL']
  ];

  // Debuff = kind x while (Chasing -> _C, Attacking -> _A). null = DEBUFF_NEVER.
  var HOM_DEBUFF = [['Never', null], ['Any', 'ANY'], ['Silent Breeze', 'BREEZE'], ['Volcanic Ash', 'ASH']];
  var MER_DEBUFF = [
    ['Never', null], ['Any', 'ANY'], ['Crash', 'CRASH'], ['Provoke', 'PROVOKE'], ['Sandman', 'SANDMAN'],
    ['Freeze Trap', 'FREEZE'], ['Decrease Agi', 'DECAGI'], ['Lex Divina', 'LEXDIV']
  ];

  // Editor layouts. Each field: [index, label, kind, options]
  var LAYOUTS = {
    hom: {
      file: 'H_Tactics.lua', pvp: false, idLabel: 'Monster ID', nameLabel: 'Monster Name', onceLevel: true,
      fields: [
        [F.BASIC, 'Basic Behavior', 'select', BASIC],
        [F.SKILL, 'Use Attack Skills', 'skill'],
        [F.DEBUFF, 'Debuff', 'debuff', HOM_DEBUFF],
        [F.KITE, 'Kiting', 'select', KITE],
        [F.CAST, 'React', 'select', HOM_CAST],
        [F.CLASS, 'Skill Class', 'select', HOM_CLASS],
        [F.SNIPE, 'Snipe OK', 'snipe'],
        [F.RESCUE, 'Rescue', 'select', HOM_RESCUE],
        [F.SP, 'SP', 'sp'],
        [F.KS, 'Kill-steal', 'select', KS],
        [F.WEIGHT, 'Weight', 'weight'],
        [F.CHASE, 'Chase', 'select', CHASE]
      ]
    },
    mer: {
      file: 'M_Tactics.lua', pvp: false, idLabel: 'Monster ID', nameLabel: 'Monster Name', onceLevel: false,
      warning: 'WARNING: In order for non-default tactics to work, you must create a MobID file. These must be re-generated after each maintenance. If you do not know what this means, you don\'t have it.',
      fields: [
        [F.BASIC, 'Basic Behavior', 'select', BASIC],
        [F.DEBUFF, 'Use Debuffs', 'debuff', MER_DEBUFF],
        [F.SKILL, 'Use Attack Skills', 'skill'],
        [F.KITE, 'Kiting', 'select', KITE],
        [F.CAST, 'React', 'select', MER_CAST],
        [F.CLASS, 'Skill Class', 'select', MER_CLASS],
        [F.SNIPE, 'Snipe OK', 'snipe'],
        [F.PUSH, 'Push', 'select', PUSH],
        [F.RESCUE, 'Rescue', 'select', MER_RESCUE],
        [F.SP, 'SP', 'sp'],
        [F.KS, 'Kill Steal', 'select', KS],
        [F.WEIGHT, 'Weight', 'weight'],
        [F.CHASE, 'Chase', 'select', CHASE]
      ]
    },
    hompvp: {
      file: 'H_PVP_Tact.lua', pvp: true, idLabel: 'Player ID or Friend Class', onceLevel: true,
      fields: [
        [F.BASIC, 'Basic Behavior', 'select', BASIC],
        [F.CAST, 'React', 'select', HOM_CAST],
        [F.DEBUFF, 'Use Debuffs', 'debuff', HOM_DEBUFF],
        [F.SKILL, 'Use Attack Skills', 'skill'],
        [F.KITE, 'Kiting', 'select', KITE],
        [F.CLASS, 'Skill Class', 'select', HOM_CLASS],
        [F.RESCUE, 'Rescue', 'select', HOM_RESCUE]
      ]
    },
    merpvp: {
      file: 'M_PVP_Tact.lua', pvp: true, idLabel: 'Player ID or Friend Class', onceLevel: false,
      fields: [
        [F.BASIC, 'Basic Behavior', 'select', BASIC],
        [F.CAST, 'React', 'select', MER_CAST],
        [F.DEBUFF, 'Use Debuffs', 'debuff', MER_DEBUFF],
        [F.SKILL, 'Use Attack Skills', 'skill'],
        [F.PUSH, 'Use Pushback', 'select', PUSH],
        [F.KITE, 'Kiting', 'select', KITE],
        [F.CLASS, 'Skill Class', 'select', MER_CLASS],
        [F.RESCUE, 'Rescue', 'select', MER_RESCUE]
      ]
    }
  };

  var TACT_DEFAULTS = ['TACT_IGNORE', 'SKILL_ALWAYS', 'KITE_NEVER', 'CAST_REACT', 'PUSH_NEVER', 'DEBUFF_NEVER',
    'CLASS_BOTH', 'RESCUE_NEVER', '-1', 'SNIPE_OK', 'KS_NEVER', '1', 'CHASE_NORMAL'];
  var PVP_DEFAULTS = ['TACT_REACT_L', 'SKILL_ALWAYS', 'KITE_NEVER', 'CAST_REACT', 'PUSH_NEVER', 'DEBUFF_NEVER',
    'CLASS_S', 'RESCUE_NEVER'];

  function newTact(id, name) {
    return { id: String(id), name: name || '', values: TACT_DEFAULTS.slice() };
  }

  function newPvpTact(id) {
    return { id: String(id), values: PVP_DEFAULTS.slice() };
  }

  function splitValues(body, count, defaults) {
    var parts = body.split(',').map(function (s) { return s.trim(); });
    var values = defaults.slice();
    for (var i = 0; i < count && i < parts.length; i++) {
      if (parts[i] !== '') values[i] = parts[i];
    }
    return values;
  }

  function isComment(line) {
    return /^\s*--/.test(line);
  }

  // MyTact[ID]={13 values} --Name
  function parseTactics(text) {
    var list = [];
    (text || '').split(/\r\n|\r|\n/).forEach(function (line) {
      if (isComment(line)) return;
      var m = /MyTact\[\s*(.*?)\s*\]\s*=\s*\{(.*?)\}(?:.*?--(.*))?/.exec(line);
      if (!m) return;
      var t = { id: m[1], name: (m[3] || '').trim(), values: splitValues(m[2], 13, TACT_DEFAULTS) };
      list.push(t);
    });
    if (!list.some(function (t) { return t.id === '0'; })) {
      list.unshift(newTact(0, 'Default'));
    }
    return list;
  }

  function compareIds(a, b) {
    var na = Number(a), nb = Number(b);
    var an = a !== '' && !isNaN(na), bn = b !== '' && !isNaN(nb);
    if (an && bn) return na - nb;
    if (an) return -1;
    if (bn) return 1;
    return a < b ? -1 : a > b ? 1 : 0;
  }

  function formatWeight(token) {
    var n = Number(token);
    return isNaN(n) ? token : String(n);
  }

  // Regenerates the file like H_Tactics.Save / M_Tactics.Save.
  function serializeTactics(list, eol) {
    eol = eol || '\r\n';
    var lines = ['MyTact={}'];
    list.slice().sort(function (a, b) { return compareIds(a.id, b.id); }).forEach(function (t) {
      var v = t.values.slice();
      v[F.WEIGHT] = formatWeight(v[F.WEIGHT]);
      lines.push('MyTact[' + t.id + ']={' + v.join(',') + '} --' + t.name);
    });
    lines.push(window.AzzyLuaConfig.padName('TactLastSavedDate') + '= "' + window.AzzyLuaConfig.nowString() + '"');
    return lines.join(eol);
  }

  // MyPVPTact[ID]={8 values}
  function parsePvp(text) {
    var list = [];
    (text || '').split(/\r\n|\r|\n/).forEach(function (line) {
      if (isComment(line)) return;
      var m = /MyPVPTact\[\s*(.*?)\s*\]\s*=\s*\{(.*?)\}/.exec(line);
      if (!m) return;
      list.push({ id: m[1], values: splitValues(m[2], 8, PVP_DEFAULTS) });
    });
    if (!list.some(function (t) { return t.id === '0'; })) {
      list.unshift(newPvpTact(0));
    }
    return list;
  }

  function escapeRegex(s) {
    return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  }

  // Updates the existing PVP file in place like PVP_Tact.Save, appending new
  // entries and dropping entries that were removed in the editor.
  function serializePvp(originalText, list) {
    var text = originalText || '';
    var eol = window.AzzyLuaConfig.eolOf(text);
    var keep = {};
    list.forEach(function (t) { keep[t.id] = true; });

    text = text.split(/\r\n|\n/).filter(function (line) {
      if (isComment(line)) return true;
      var m = /MyPVPTact\[\s*(.*?)\s*\]\s*=\s*\{/.exec(line);
      return !m || keep[m[1]];
    }).join(eol);

    list.forEach(function (t) {
      var body = t.values.slice(0, 8).join(',');
      var re = new RegExp('^([^\\r\\n-]*MyPVPTact\\[\\s*' + escapeRegex(t.id) + '\\s*\\]\\s*=\\s*\\{)[^}\\r\\n]*(\\})', 'm');
      if (re.test(text)) {
        text = text.replace(re, function (all, a, b) { return a + body + b; });
      } else {
        text += (text === '' || /\n$/.test(text) ? '' : eol) + 'MyPVPTact[' + t.id + ']={' + body + '}';
      }
    });
    return text;
  }

  window.AzzyTactics = {
    F: F,
    LAYOUTS: LAYOUTS,
    newTact: newTact,
    newPvpTact: newPvpTact,
    parseTactics: parseTactics,
    serializeTactics: serializeTactics,
    parsePvp: parsePvp,
    serializePvp: serializePvp
  };
})();
