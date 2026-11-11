// UpdateTools.cs
//
// Logic behind the Update menu: refresh this USER_AI folder from the git repository (a local clone
// or a GitHub zip) and refresh the monster/skill data from rAthena. Files the user edits are kept.

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Text;

namespace AzzyAIConfig
{
    public class UpdateResult
    {
        public int Updated;
        public int Added;
        public int Kept;
        public bool ExeStaged;
        public string BackupDir;
    }

    public static class UpdateTools
    {
        public const string DefaultRepoUrl = "https://github.com/jknpj/AzzyAI";
        public const string DefaultBranch = "main";
        public const string ExeName = "AzzyAIConfig.exe";

        // Files the user edits (settings, tactics, lists): never overwritten, only created when missing
        static readonly string[] UserFiles =
        {
            "H_Config.lua", "M_Config.lua", "H_Tactics.lua", "M_Tactics.lua", "H_PVP_Tact.lua", "M_PVP_Tact.lua",
            "H_Extra.lua", "M_Extra.lua", "H_Avoid.lua", "A_Friends.lua", "Mob_ID.lua"
        };

        // Set by tests; the folder of the running exe otherwise
        public static string AiDirOverride;

        public static string AiDir
        {
            get { return (AiDirOverride ?? AppDomain.CurrentDomain.BaseDirectory).TrimEnd('\\', '/'); }
        }

        static string SettingsFile
        {
            get { return Path.Combine(AiDir, "AzzyAIConfig.update.txt"); }
        }

        public static bool IsClone(string dir)
        {
            return !string.IsNullOrEmpty(dir) && Directory.Exists(Path.Combine(dir, "USER_AI"))
                && File.Exists(Path.Combine(dir, "tools", "harness", "rathena_sync.py"));
        }

        // The saved clone, else a clone that contains this folder
        public static string FindClone()
        {
            try
            {
                if (File.Exists(SettingsFile))
                {
                    string saved = File.ReadAllText(SettingsFile).Trim();
                    if (IsClone(saved)) return saved;
                }
            }
            catch (IOException) { }
            for (DirectoryInfo d = new DirectoryInfo(AiDir); d != null; d = d.Parent)
            {
                if (IsClone(d.FullName)) return d.FullName;
            }
            return "";
        }

        public static void SaveClone(string dir)
        {
            try { File.WriteAllText(SettingsFile, dir); }
            catch (IOException) { }
            catch (UnauthorizedAccessException) { }
        }

        public static int Run(string exe, string args, string workDir, Action<string> log)
        {
            ProcessStartInfo psi = new ProcessStartInfo(exe, args);
            psi.WorkingDirectory = workDir;
            psi.UseShellExecute = false;
            psi.CreateNoWindow = true;
            psi.RedirectStandardOutput = true;
            psi.RedirectStandardError = true;
            psi.StandardOutputEncoding = Encoding.UTF8;
            psi.StandardErrorEncoding = Encoding.UTF8;
            psi.EnvironmentVariables["PYTHONIOENCODING"] = "utf-8";
            psi.EnvironmentVariables["PYTHONUNBUFFERED"] = "1";
            using (Process p = new Process())
            {
                p.StartInfo = psi;
                DataReceivedEventHandler h = delegate(object s, DataReceivedEventArgs e) { if (e.Data != null) log(e.Data); };
                p.OutputDataReceived += h;
                p.ErrorDataReceived += h;
                try { p.Start(); }
                catch (System.ComponentModel.Win32Exception ex)
                {
                    log("Could not start " + exe + ": " + ex.Message);
                    return -1;
                }
                p.BeginOutputReadLine();
                p.BeginErrorReadLine();
                p.WaitForExit();
                return p.ExitCode;
            }
        }

        // "python" or "py -3", whichever runs; null when there is no Python
        public static string[] FindPython()
        {
            string[][] candidates = { new[] { "python", "" }, new[] { "py", "-3" } };
            foreach (string[] c in candidates)
            {
                int code = Run(c[0], (c[1] + " --version").Trim(), AiDir, delegate(string s) { });
                if (code == 0) return c;
            }
            return null;
        }

        public static bool PullClone(string clone, Action<string> log)
        {
            log("git pull in " + clone);
            if (Run("git", "pull --ff-only", clone, log) != 0)
            {
                log("git pull failed. Fix the clone (local changes, another branch, no network) and try again.");
                return false;
            }
            Run("git", "log -1 --format=\"Now at %h %s\"", clone, log);
            return true;
        }

        // Downloads the branch zip and returns the USER_AI folder inside it; tempRoot is deleted by the caller
        public static string DownloadBranch(string repoUrl, string branch, string tempRoot, Action<string> log)
        {
            string url = repoUrl.TrimEnd('/') + "/archive/refs/heads/" + branch + ".zip";
            string zip = Path.Combine(tempRoot, "ai.zip");
            string extracted = Path.Combine(tempRoot, "x");
            Directory.CreateDirectory(tempRoot);
            log("Downloading " + url);
            ServicePointManager.SecurityProtocol = (SecurityProtocolType)3072;
            using (WebClient wc = new WebClient())
            {
                wc.Proxy = WebRequest.GetSystemWebProxy();
                wc.Proxy.Credentials = CredentialCache.DefaultCredentials;
                wc.DownloadFile(url, zip);
            }
            log("Extracting...");
            string cmd = "Expand-Archive -LiteralPath '" + zip + "' -DestinationPath '" + extracted + "' -Force";
            if (Run("powershell", "-NoProfile -ExecutionPolicy Bypass -Command \"" + cmd + "\"", tempRoot, log) != 0)
                throw new IOException("Could not extract the downloaded zip.");
            foreach (string d in Directory.GetDirectories(extracted))
            {
                string ai = Path.Combine(d, "USER_AI");
                if (Directory.Exists(ai)) return ai;
            }
            throw new IOException("The downloaded zip has no USER_AI folder (wrong URL or branch?).");
        }

        static bool SameContent(string a, string b)
        {
            if (!File.Exists(a) || !File.Exists(b)) return false;
            FileInfo fa = new FileInfo(a), fb = new FileInfo(b);
            if (fa.Length != fb.Length) return false;
            byte[] x = File.ReadAllBytes(a), y = File.ReadAllBytes(b);
            for (int i = 0; i < x.Length; i++) if (x[i] != y[i]) return false;
            return true;
        }

        static bool IsUserFile(string rel, bool overwritePresets)
        {
            foreach (string u in UserFiles)
                if (string.Equals(u, rel, StringComparison.OrdinalIgnoreCase)) return true;
            return !overwritePresets && rel.StartsWith("Types" + Path.DirectorySeparatorChar, StringComparison.OrdinalIgnoreCase);
        }

        static void Backup(UpdateResult r, string dst, string rel)
        {
            if (r.BackupDir == null)
                r.BackupDir = Path.Combine(AiDir, "backup", "update-" + DateTime.Now.ToString("yyyyMMdd-HHmmss"));
            string to = Path.Combine(r.BackupDir, rel);
            Directory.CreateDirectory(Path.GetDirectoryName(to));
            File.Copy(dst, to, true);
        }

        // Copies srcDir over dstDir, keeping user files; the running exe is staged as AzzyAIConfig.exe.new
        public static UpdateResult Apply(string srcDir, string dstDir, bool overwritePresets, Action<string> log)
        {
            if (string.Equals(Path.GetFullPath(srcDir).TrimEnd('\\'), Path.GetFullPath(dstDir).TrimEnd('\\'),
                StringComparison.OrdinalIgnoreCase))
                throw new InvalidOperationException("The source is this same folder; nothing to update.");
            UpdateResult r = new UpdateResult();
            string root = Path.GetFullPath(srcDir).TrimEnd('\\') + "\\";
            foreach (string file in Directory.GetFiles(srcDir, "*", SearchOption.AllDirectories))
            {
                string rel = Path.GetFullPath(file).Substring(root.Length);
                string dst = Path.Combine(dstDir, rel);
                if (rel.EndsWith(".pdb", StringComparison.OrdinalIgnoreCase)) continue;
                if (string.Equals(rel, ExeName, StringComparison.OrdinalIgnoreCase))
                {
                    if (!SameContent(file, dst))
                    {
                        File.Copy(file, dst + ".new", true);
                        r.ExeStaged = true;
                        log("GUI program: staged as " + ExeName + ".new");
                    }
                    continue;
                }
                bool exists = File.Exists(dst);
                if (exists && SameContent(file, dst)) continue;
                if (exists && IsUserFile(rel, overwritePresets))
                {
                    r.Kept++;
                    log("Kept yours (upstream differs): " + rel);
                    continue;
                }
                Directory.CreateDirectory(Path.GetDirectoryName(dst));
                if (exists) Backup(r, dst, rel);
                File.Copy(file, dst, true);
                if (exists) r.Updated++; else r.Added++;
                log((exists ? "Updated: " : "Added:   ") + rel);
            }
            return r;
        }

        // Replaces the exe after this process exits, then starts it again
        public static void RestartWithStagedExe()
        {
            string exe = Path.Combine(AiDir, ExeName);
            string cmd = "/c ping 127.0.0.1 -n 3 >nul & move /y \"" + exe + ".new\" \"" + exe + "\" & start \"\" \"" + exe + "\"";
            ProcessStartInfo psi = new ProcessStartInfo("cmd.exe", cmd);
            psi.WorkingDirectory = AiDir;
            psi.CreateNoWindow = true;
            psi.UseShellExecute = false;
            Process.Start(psi);
        }
    }
}
