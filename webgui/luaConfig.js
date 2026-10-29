// luaConfig.js
// Reads and writes H_Config.lua / M_Config.lua the same way the C# GUI does:
// values are matched by "Name = value", replaced in place (keeping the rest of
// the file untouched) and appended when missing. LastSavedDate is refreshed.
(function () {
  'use strict';

  var TOKEN = '(-?(?:\\d+(?:\\.\\d*)?|\\.\\d+)|[A-Za-z_][A-Za-z0-9_]*)';

  function escapeRegex(s) {
    return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  }

  function luaName(entry) {
    return entry.lua || entry.name;
  }

  function assignRegex(name, flags) {
    return new RegExp('^([ \\t]*)' + escapeRegex(name) + '[ \\t]*=[ \\t]*' + TOKEN, flags || 'gm');
  }

  function eolOf(text) {
    return /\r\n/.test(text) ? '\r\n' : '\n';
  }

  function padName(name) {
    var s = name;
    while (s.length < 25) s += ' ';
    return s;
  }

  function clamp(entry, v) {
    if (typeof entry.min === 'number' && v < entry.min) v = entry.min;
    if (typeof entry.max === 'number' && v > entry.max) v = entry.max;
    return v;
  }

  function defaults(schema) {
    var values = {};
    schema.forEach(function (e) { values[e.name] = e.default; });
    return values;
  }

  // Returns the raw token of the last uncommented assignment, or null.
  function readToken(text, name) {
    var re = assignRegex(name);
    var m, last = null;
    while ((m = re.exec(text)) !== null) last = m[2];
    return last;
  }

  function fromToken(entry, token) {
    switch (entry.type) {
      case 'bool':
        if (token === 'true') return true;
        if (token === 'false') return false;
        if (!/^-?\d/.test(token)) return undefined;
        return parseFloat(token) !== 0;
      case 'serverType':
        if (token === 'SERVER_PRIVATE' || token === '1') return 1;
        if (token === 'SERVER_OFFICIAL' || token === '0') return 0;
        return undefined;
      case 'double': {
        var d = parseFloat(token);
        return isNaN(d) ? undefined : clamp(entry, d);
      }
      default: {
        if (!/^-?\d/.test(token)) return undefined;
        var n = parseInt(token, 10);
        if (isNaN(n)) return undefined;
        return entry.type === 'int' ? clamp(entry, n) : n;
      }
    }
  }

  function toToken(entry, value) {
    switch (entry.type) {
      case 'bool': return value ? '1' : '0';
      case 'serverType': return Number(value) === 1 ? 'SERVER_PRIVATE' : 'SERVER_OFFICIAL';
      case 'double': return String(Number(value));
      default: return String(Math.trunc(Number(value)));
    }
  }

  // Load values from a Lua config text. Missing values keep their defaults.
  function load(text, schema) {
    var values = defaults(schema);
    schema.forEach(function (e) {
      var token = readToken(text, luaName(e));
      if (token === null) return;
      var v = fromToken(e, token);
      if (v !== undefined) values[e.name] = v;
    });
    return values;
  }

  function setAssignment(text, name, token) {
    var re = assignRegex(name);
    if (re.test(text)) {
      return text.replace(assignRegex(name), function (all, indent) {
        return indent + padName(name) + '= ' + token;
      });
    }
    var eol = eolOf(text);
    var sep = text.length === 0 || /\n$/.test(text) ? '' : eol;
    return text + sep + padName(name) + '= ' + token + eol;
  }

  function nowString() {
    return new Date().toLocaleString();
  }

  // Writes values into the template text, preserving everything else.
  function save(text, schema, values) {
    var out = text || '';
    schema.forEach(function (e) {
      var v = values[e.name];
      if (v === undefined) v = e.default;
      out = setAssignment(out, luaName(e), toToken(e, v));
    });
    var dateLine = padName('LastSavedDate') + '= "' + nowString() + '"';
    var dateRe = /^[ \t]*LastSavedDate[ \t]*=[ \t]*".*"/m;
    if (dateRe.test(out)) {
      out = out.replace(dateRe, dateLine);
    } else {
      var eol = eolOf(out);
      out += (/\n$/.test(out) ? '' : eol) + dateLine + eol;
    }
    return out;
  }

  window.AzzyLuaConfig = {
    load: load,
    save: save,
    defaults: defaults,
    clamp: clamp,
    padName: padName,
    eolOf: eolOf,
    nowString: nowString
  };
})();
