using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;

namespace AzzyAIConfig
{
    // Per-homunculus-type presets: Types\<Name>.lua lists only the options that differ from the
    // global H_Config.lua. The AI loads the file for the active homunculus right after H_Config.lua.
    static class TypePreset
    {
        public const string Global = "Global (all homunculi)";
        public const string Folder = "Types";
        static readonly string[] Ignored = { "LastSavedDate" };
        static readonly Regex Assignment = new Regex(@"^[ \t]*([A-Za-z_][A-Za-z0-9_]*)[ \t]*=[ \t]*([^\r\n]*)(?=\r?$)", RegexOptions.Multiline);

        public static readonly string[] Names =
            { "Lif", "Amistr", "Filir", "Vanilmirth", "Eira", "Bayeri", "Sera", "Dieter", "Eleanor" };

        public static string PathFor(string name)
        {
            return Path.Combine(Folder, name + ".lua");
        }

        // key -> value, in file order; later assignments win, as in Lua
        public static List<KeyValuePair<string, string>> Parse(string text)
        {
            var result = new List<KeyValuePair<string, string>>();
            foreach (Match m in Assignment.Matches(text))
            {
                if (m.Value.TrimStart().StartsWith("--", StringComparison.Ordinal)) continue;
                string key = m.Groups[1].Value;
                string value = StripComment(m.Groups[2].Value).Trim();
                int index = result.FindIndex(p => p.Key == key);
                if (index >= 0) result.RemoveAt(index);
                result.Add(new KeyValuePair<string, string>(key, value));
            }
            return result;
        }

        static string StripComment(string value)
        {
            bool quoted = false;
            for (int i = 0; i < value.Length; i++)
            {
                if (value[i] == '"') quoted = !quoted;
                else if (!quoted && value[i] == '-' && i + 1 < value.Length && value[i + 1] == '-')
                    return value.Substring(0, i);
            }
            return value;
        }

        static bool SameValue(string a, string b)
        {
            double x, y;
            if (double.TryParse(a, NumberStyles.Float, CultureInfo.InvariantCulture, out x) &&
                double.TryParse(b, NumberStyles.Float, CultureInfo.InvariantCulture, out y))
                return x == y;
            return a == b;
        }

        // The global text with the overrides of a type file applied on top
        public static string Merge(string globalText, string typeText)
        {
            string result = globalText;
            foreach (var pair in Parse(typeText))
            {
                string key = pair.Key, value = pair.Value;
                var pattern = new Regex(@"^([ \t]*" + Regex.Escape(key) + @"[ \t]*=[ \t]*)[^\r\n]*(?=\r?$)", RegexOptions.Multiline);
                if (pattern.IsMatch(result))
                    result = pattern.Replace(result, m => m.Groups[1].Value + value);
                else
                    result += Environment.NewLine + key.PadRight(25) + "= " + value;
            }
            return result;
        }

        // Writes Types\<name>.lua with the options of savedText that differ from the baseline
        // (Defaults.lua then H_Config.lua). Options of the old file the GUI doesn't know are kept.
        public static void Write(string name, string defaultsText, string globalText, string savedText)
        {
            var baseline = new Dictionary<string, string>();
            foreach (var pair in Parse(defaultsText)) baseline[pair.Key] = pair.Value;
            foreach (var pair in Parse(globalText)) baseline[pair.Key] = pair.Value;
            var saved = Parse(savedText);
            var savedKeys = new HashSet<string>();
            foreach (var pair in saved) savedKeys.Add(pair.Key);

            string path = PathFor(name);
            string oldText = File.Exists(path) ? File.ReadAllText(path) : "";
            var lines = new List<string>();
            foreach (var pair in Parse(oldText))
                if (!savedKeys.Contains(pair.Key) && Array.IndexOf(Ignored, pair.Key) < 0)
                    lines.Add(pair.Key.PadRight(25) + "= " + pair.Value);
            foreach (var pair in saved)
            {
                string original;
                if (Array.IndexOf(Ignored, pair.Key) >= 0) continue;
                if (baseline.TryGetValue(pair.Key, out original) && SameValue(original, pair.Value)) continue;
                lines.Add(pair.Key.PadRight(25) + "= " + pair.Value);
            }

            var header = new StringBuilder();
            foreach (string line in oldText.Replace("\r", "").Split('\n'))
            {
                if (!line.StartsWith("--", StringComparison.Ordinal)) break;
                header.Append(line).Append(Environment.NewLine);
            }
            if (header.Length == 0)
                header.Append("-- " + name + " preset. Loaded after H_Config.lua: only options that differ for " + name + " are listed.")
                    .Append(Environment.NewLine)
                    .Append("-- Delete a line to use the global H_Config.lua value, or delete this file to disable the preset.")
                    .Append(Environment.NewLine);

            Directory.CreateDirectory(Folder);
            File.WriteAllText(path, header + string.Join(Environment.NewLine, lines.ToArray()) +
                (lines.Count > 0 ? Environment.NewLine : ""));
        }
    }
}
