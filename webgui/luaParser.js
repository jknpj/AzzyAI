// Minimal Lua parser stub for AzzyAIConfig Web GUI
// This attaches a parseLuaTable function to window that parses a very basic subset of Lua tables.
// For production, replace with a robust Lua parser or use a WASM Lua runtime.


// Improved Lua table parser for AzzyAIConfig Web GUI
// Handles nested tables, arrays, comments, and basic Lua syntax (not full Lua)
(function() {
  function parseLuaTable(lua) {
    if (typeof lua !== 'string') return null;
    // Remove comments (--) and blank lines
    lua = lua.replace(/--.*$/gm, '').replace(/\r/g, '').trim();
    // Remove 'return' at start
    if (lua.startsWith('return')) lua = lua.slice(6).trim();
    // Try to find a table
    const start = lua.indexOf('{');
    const end = lua.lastIndexOf('}');
    if (start !== -1 && end !== -1) {
      const tableStr = lua.slice(start, end + 1);
      try {
        const parsed = parseTable(tableStr);
        if (parsed === null) {
          alert('[DEBUG] parseLuaTable: parseTable returned null for string:\n' + tableStr.slice(0, 200));
        }
        return parsed;
      } catch (e) {
        alert('[DEBUG] parseLuaTable: Exception: ' + e + '\nInput:\n' + tableStr.slice(0, 200));
        return null;
      }
    }
    // If not a table, try flat assignment (key = value) config
    const obj = {};
    const lines = lua.split(/\n|;/);
    let found = false;
    for (let line of lines) {
      line = line.trim();
      if (!line) continue;
      const m = line.match(/^([a-zA-Z0-9_]+)\s*=\s*(.+)$/);
      if (!m) continue;
      let key = m[1];
      let val = m[2].trim();
      // Remove trailing comment
      val = val.replace(/--.*$/, '').trim();
      // Remove quotes if present
      if ((val.startsWith('"') && val.endsWith('"')) || (val.startsWith("'") && val.endsWith("'"))) {
        val = val.slice(1, -1);
      } else if (val === 'true') {
        val = true;
      } else if (val === 'false') {
        val = false;
      } else if (!isNaN(Number(val))) {
        val = Number(val);
      }
      obj[key] = val;
      found = true;
    }
    if (found) return obj;
    alert('[DEBUG] parseLuaTable: No table or flat assignments found in string:\n' + lua.slice(0, 200));
    return null;
  }

  // Parse a Lua table string into a JS object/array
  function parseTable(str) {
    str = str.trim();
    if (!str.startsWith('{') || !str.endsWith('}')) return null;
    str = str.slice(1, -1).trim();
    if (!str) return {};
    // Remove trailing commas and tolerate nil values
    str = str.replace(/,\s*}/g, '}').replace(/\bnil\b/g, 'null');
    let i = 0;
    const len = str.length;
    const result = [];
    const obj = {};
    let isArray = true;
    let hasKeys = false;
    while (i < len) {
      // Skip whitespace
      while (i < len && /\s/.test(str[i])) i++;
      if (i >= len) break;
      // Key or value?
      let key = null, value = null;
      if (str[i] === '[') {
        // [key] = value
        i++;
        let keyEnd = str.indexOf(']', i);
        key = str.slice(i, keyEnd).trim();
        i = keyEnd + 1;
        while (i < len && /\s|=/.test(str[i])) i++;
        [value, i] = parseValue(str, i);
        isArray = false;
        hasKeys = true;
      } else {
        // key = value OR value (array)
        let eq = str.indexOf('=', i);
        let comma = str.indexOf(',', i);
        let brace = str.indexOf('{', i);
        if (eq !== -1 && (comma === -1 || eq < comma) && (brace === -1 || eq < brace)) {
          // key = value
          key = str.slice(i, eq).trim();
          i = eq + 1;
          [value, i] = parseValue(str, i);
          isArray = false;
          hasKeys = true;
        } else {
          // value (array)
          [value, i] = parseValue(str, i);
        }
      }
      // Store value
      if (key !== null) {
        if (value !== undefined && value !== null) obj[key] = value;
      } else {
        if (value !== undefined && value !== null) result.push(value);
      }
      // Skip comma
      while (i < len && (str[i] === ',' || /\s/.test(str[i]))) i++;
    }
    // If any keys were found, return object, else array
    if (hasKeys) return obj;
    return result;
  }

  // Parse a value (number, string, boolean, or table)
  function parseValue(str, i) {
    while (i < str.length && /\s/.test(str[i])) i++;
    if (str[i] === '{') {
      // Nested table
      let depth = 1, j = i + 1;
      while (j < str.length && depth > 0) {
        if (str[j] === '{') depth++;
        else if (str[j] === '}') depth--;
        j++;
      }
      const tableStr = str.slice(i, j);
      return [parseTable(tableStr), j];
    } else if (str[i] === '"' || str[i] === "'") {
      // String
      const quote = str[i];
      let j = i + 1;
      while (j < str.length && str[j] !== quote) j++;
      const val = str.slice(i + 1, j);
      return [val, j + 1];
    } else {
      // Number, boolean, or bareword
      let j = i;
      while (j < str.length && !/[,}]|\s/.test(str[j])) j++;
      let raw = str.slice(i, j).trim();
      if (raw === 'true') return [true, j];
      if (raw === 'false') return [false, j];
      if (!isNaN(Number(raw))) return [Number(raw), j];
      return [raw, j];
    }
  }

  window.parseLuaTable = parseLuaTable;
})();
