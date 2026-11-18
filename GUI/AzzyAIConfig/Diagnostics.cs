// Diagnostics.cs
//
// Collects what a bug report needs - AAIStartH.txt, the AI logs, the player's configuration and the
// learned skill levels - into one zip the user attaches to a GitHub issue. Nothing is sent anywhere:
// the files are copied to a folder, the user reviews them and decides what to share.

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;

namespace AzzyAIConfig
{
    public class DiagnosticsEntry
    {
        public string Name;     // path inside the bundle
        public string Source;   // file this came from
        public string Content;  // text written to the bundle, already trimmed and redacted
        public bool Truncated;
    }

    public static class Diagnostics
    {
        public const string RepoUrl = "https://github.com/jknpj/AzzyAI";
        public const string IssueUrl = RepoUrl + "/issues/new?template=bug_report.yml";

        // Enough context for a report, small enough to attach: the logs are per-owner and grow
        // until the client restarts, so a tail is taken instead of the whole file.
        const int TailLines = 400;

        static readonly string[] StartupLogs = { "AAIStartH.txt", "AAIStartM.txt" };
        static readonly string[] LogTails = { "AAI_ERROR.log", "AAI_SKILLFAIL.log", "AAI_PROBE.log" };
        static readonly string[] ConfigFiles =
        {
            "H_Config.lua", "M_Config.lua", "H_Tactics.lua", "M_Tactics.lua", "H_PVP_Tact.lua",
            "M_PVP_Tact.lua", "H_Extra.lua", "M_Extra.lua"
        };

        public static string AiDir
        {
            get { return UpdateTools.AiDir; }
        }

        // The AI writes the logs to the client's working folder, normally the one above USER_AI;
        // the exe can be moved, so the newest startup report decides.
        public static string FindLogDir()
        {
            string best = null;
            DateTime newest = DateTime.MinValue;
            List<string> candidates = new List<string>();
            string dir = AiDir;
            for (int depth = 0; depth < 3 && !string.IsNullOrEmpty(dir); depth++)
            {
                candidates.Add(dir);
                DirectoryInfo parent = Directory.GetParent(dir);
                dir = parent == null ? null : parent.FullName;
            }
            candidates.Add(Environment.CurrentDirectory);
            foreach (string candidate in candidates)
            {
                foreach (string name in StartupLogs)
                {
                    string path = Path.Combine(candidate, name);
                    if (!File.Exists(path)) continue;
                    DateTime time = File.GetLastWriteTime(path);
                    if (best == null || time > newest)
                    {
                        best = candidate;
                        newest = time;
                    }
                }
            }
            return best ?? AiDir;
        }

        // "Constant version:2.0.2" from the startup report, else the exe's own version.
        public static string Release()
        {
            string dir = FindLogDir();
            foreach (string name in StartupLogs)
            {
                string path = Path.Combine(dir, name);
                if (!File.Exists(path)) continue;
                try
                {
                    Match m = Regex.Match(File.ReadAllText(path, Encoding.UTF8), @"Constant version:(\S+)");
                    if (m.Success) return m.Groups[1].Value.Trim();
                }
                catch (IOException) { }
                catch (UnauthorizedAccessException) { }
            }
            return typeof(Diagnostics).Assembly.GetName().Version.ToString();
        }

        public static List<DiagnosticsEntry> Collect(out string logDir)
        {
            logDir = FindLogDir();
            List<string> owners = OwnerIds();
            List<DiagnosticsEntry> list = new List<DiagnosticsEntry>();
            foreach (string name in StartupLogs) AddFile(list, logDir, name, name, owners, false);
            foreach (string name in LogTails) AddFile(list, logDir, name, name, owners, true);
            foreach (string name in ConfigFiles) AddFile(list, AiDir, name, name, owners, false);
            AddFolder(list, Path.Combine(AiDir, "Types"), "Types", owners);
            AddFolder(list, Path.Combine(AiDir, "data"), "data", owners);
            return list;
        }

        // Owner IDs appear in the data file names and in some log lines; they are not needed to
        // reproduce a report and are replaced before anything is written.
        static List<string> OwnerIds()
        {
            List<string> ids = new List<string>();
            string data = Path.Combine(AiDir, "data");
            if (!Directory.Exists(data)) return ids;
            foreach (string path in Directory.GetFiles(data, "*.lua"))
            {
                Match m = Regex.Match(Path.GetFileName(path), @"^[HM]_(\d+)");
                if (m.Success && !ids.Contains(m.Groups[1].Value)) ids.Add(m.Groups[1].Value);
            }
            return ids;
        }

        // The id is glued to words in the data file names (H_2000001Skills.lua), so only digits
        // are used as the boundary; \b would not match between "1" and "S".
        static string Redact(string text, List<string> owners)
        {
            foreach (string id in owners)
                text = Regex.Replace(text, @"(?<!\d)" + Regex.Escape(id) + @"(?!\d)", "<owner>");
            return text;
        }

        static void AddFile(List<DiagnosticsEntry> list, string dir, string name, string entryName,
            List<string> owners, bool tail)
        {
            string path = Path.Combine(dir, name);
            if (!File.Exists(path)) return;
            string text;
            try
            {
                text = File.ReadAllText(path, Encoding.UTF8);
            }
            catch (IOException) { return; }
            catch (UnauthorizedAccessException) { return; }
            bool truncated;
            string tailed = Tail(text, tail, out truncated);
            DiagnosticsEntry entry = new DiagnosticsEntry();
            entry.Name = Redact(entryName, owners);
            entry.Source = path;
            entry.Content = Redact(tailed, owners);
            entry.Truncated = truncated;
            list.Add(entry);
        }

        static void AddFolder(List<DiagnosticsEntry> list, string dir, string prefix, List<string> owners)
        {
            if (!Directory.Exists(dir)) return;
            string[] files = Directory.GetFiles(dir, "*.lua");
            Array.Sort(files, StringComparer.OrdinalIgnoreCase);
            foreach (string path in files)
                AddFile(list, dir, Path.GetFileName(path),
                    prefix + "/" + Path.GetFileName(path), owners, false);
        }

        static string Tail(string text, bool tail, out bool truncated)
        {
            truncated = false;
            if (!tail || text.Length == 0) return text;
            string[] lines = text.Replace("\r\n", "\n").Split('\n');
            if (lines.Length <= TailLines) return text;
            truncated = true;
            StringBuilder sb = new StringBuilder();
            sb.Append("... ").Append(lines.Length - TailLines).Append(" earlier lines left out ...\n");
            for (int i = lines.Length - TailLines; i < lines.Length; i++)
                sb.Append(lines[i]).Append('\n');
            return sb.ToString();
        }

        // Text shown in the dialog and copied to the clipboard: what goes out, and why.
        public static string Summary(List<DiagnosticsEntry> entries, string logDir)
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("AzzyAI ").Append(Release()).Append(" diagnostics\n");
            sb.Append("Exported: ").Append(DateTime.Now.ToString("yyyy-MM-dd HH:mm", CultureInfo.InvariantCulture)).Append('\n');
            sb.Append("AI folder: ").Append(AiDir).Append('\n');
            sb.Append("Log folder: ").Append(logDir).Append('\n');
            sb.Append("Owner IDs in file names and log lines were replaced by <owner>.\n");
            sb.Append("Nothing is uploaded: the files stay on this computer until you attach them.\n\n");
            sb.Append("Included files:\n");
            foreach (DiagnosticsEntry entry in entries)
            {
                sb.Append("  ").Append(entry.Name);
                if (entry.Truncated) sb.Append(" (last ").Append(TailLines).Append(" lines)");
                sb.Append('\n');
            }
            if (entries.Count == 0)
                sb.Append("  (no AAIStartH.txt found: start the AI in game once, then export again)\n");
            sb.Append('\n');
            foreach (DiagnosticsEntry entry in entries)
            {
                if (!entry.Name.StartsWith("AAIStart", StringComparison.Ordinal)) continue;
                sb.Append("--- ").Append(entry.Name).Append(" ---\n");
                sb.Append(entry.Content.TrimEnd()).Append('\n');
                break;
            }
            return sb.ToString();
        }

        // Writes the bundle next to the Desktop and returns the file that was created.
        public static string Export(List<DiagnosticsEntry> entries, string logDir)
        {
            return Export(entries, logDir, null);
        }

        public static string Export(List<DiagnosticsEntry> entries, string logDir, string crash)
        {
            if (!string.IsNullOrEmpty(crash))
            {
                DiagnosticsEntry entry = new DiagnosticsEntry();
                entry.Name = "crash.txt";
                entry.Source = "";
                entry.Content = crash;
                entries.Add(entry);
            }
            string stamp = DateTime.Now.ToString("yyyyMMdd-HHmm", CultureInfo.InvariantCulture);
            string root = Path.Combine(Path.GetTempPath(), "AzzyAI-diag-" + stamp);
            string files = Path.Combine(root, "AzzyAI-diag");
            Directory.CreateDirectory(files);
            foreach (DiagnosticsEntry entry in entries)
            {
                string target = Path.Combine(files, entry.Name.Replace('/', Path.DirectorySeparatorChar));
                string parent = Path.GetDirectoryName(target);
                if (!string.IsNullOrEmpty(parent)) Directory.CreateDirectory(parent);
                File.WriteAllText(target, entry.Content, new UTF8Encoding(false));
            }
            File.WriteAllText(Path.Combine(files, "info.txt"), Summary(entries, logDir), new UTF8Encoding(false));

            string desktop = Environment.GetFolderPath(Environment.SpecialFolder.DesktopDirectory);
            string outDir = string.IsNullOrEmpty(desktop) || !Directory.Exists(desktop) ? files : desktop;
            string zip = Path.Combine(outDir, "AzzyAI-diag-" + stamp + ".zip");
            if (TryZip(files, zip)) return zip;
            // No PowerShell 5 (Compress-Archive): one text file keeps the same information.
            string txt = Path.Combine(outDir, "AzzyAI-diag-" + stamp + ".txt");
            StringBuilder sb = new StringBuilder();
            sb.Append(Summary(entries, logDir)).Append('\n');
            foreach (DiagnosticsEntry entry in entries)
                sb.Append("\n===== ").Append(entry.Name).Append(" =====\n").Append(entry.Content);
            File.WriteAllText(txt, sb.ToString(), new UTF8Encoding(false));
            return txt;
        }

        static bool TryZip(string folder, string zip)
        {
            try
            {
                ProcessStartInfo psi = new ProcessStartInfo("powershell.exe",
                    "-NoProfile -NonInteractive -ExecutionPolicy Bypass -Command \"Compress-Archive -Path '" +
                    folder.Replace("'", "''") + "\\*' -DestinationPath '" + zip.Replace("'", "''") + "' -Force\"");
                psi.UseShellExecute = false;
                psi.CreateNoWindow = true;
                psi.RedirectStandardError = true;
                psi.RedirectStandardOutput = true;
                using (Process p = Process.Start(psi))
                {
                    if (p == null) return false;
                    string error = p.StandardError.ReadToEnd();
                    p.WaitForExit(120000);
                    bool done = p.HasExited && p.ExitCode == 0;
                    if (!done) Trace.WriteLine("Compress-Archive failed: " + error);
                }
            }
            catch (Exception e)
            {
                Trace.WriteLine("Compress-Archive could not run: " + e.Message);
            }
            return File.Exists(zip);
        }
    }
}
