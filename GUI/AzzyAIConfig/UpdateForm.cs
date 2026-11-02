// UpdateForm.cs
//
// The dialogs behind Update > "Update AI from git..." and Update > "Update monster/skill data...".
// Built in code (no designer file) because it is small and has no resources.

using System;
using System.Drawing;
using System.IO;
using System.Threading;
using System.Windows.Forms;

namespace AzzyAIConfig
{
    public enum UpdateMode { Ai, Databases }

    public class UpdateForm : Form
    {
        readonly UpdateMode _mode;
        RadioButton _radioGit, _radioZip;
        TextBox _clone, _url, _branch, _ref, _log;
        CheckBox _presets;
        Button _start, _close;
        volatile bool _running;

        public UpdateForm(UpdateMode mode)
        {
            _mode = mode;
            Text = mode == UpdateMode.Ai ? "Update AI from git" : "Update monster/skill data from rAthena";
            StartPosition = FormStartPosition.CenterParent;
            ClientSize = new Size(640, 470);
            FormBorderStyle = FormBorderStyle.FixedDialog;
            MaximizeBox = false;
            MinimizeBox = false;
            Build();
        }

        Label AddLabel(string text, int x, int y, int w, int h)
        {
            Label l = new Label();
            l.Text = text;
            l.SetBounds(x, y, w, h);
            Controls.Add(l);
            return l;
        }

        TextBox AddBox(string text, int x, int y, int w)
        {
            TextBox t = new TextBox();
            t.Text = text;
            t.SetBounds(x, y, w, 22);
            Controls.Add(t);
            return t;
        }

        void Build()
        {
            string clone = UpdateTools.FindClone();
            int y = 12;
            if (_mode == UpdateMode.Ai)
            {
                AddLabel("Updates the Lua files in this folder:\n" + UpdateTools.AiDir + "\n" +
                    "Your H_Config.lua, M_Config.lua, tactics, extra and avoid lists are never overwritten. " +
                    "Replaced files are backed up to the backup folder.", 12, y, 616, 64);
                y += 72;
                _radioGit = new RadioButton();
                _radioGit.Text = "Pull a local git clone, then copy its USER_AI folder here";
                _radioGit.Checked = clone.Length > 0;
                _radioGit.SetBounds(12, y, 616, 20);
                Controls.Add(_radioGit);
                y += 24;
                _clone = AddBox(clone, 32, y, 500);
                AddBrowse(32 + 508, y);
                y += 32;
                _radioZip = new RadioButton();
                _radioZip.Text = "Download the branch from GitHub (no git needed)";
                _radioZip.Checked = clone.Length == 0;
                _radioZip.SetBounds(12, y, 616, 20);
                Controls.Add(_radioZip);
                y += 24;
                _url = AddBox(UpdateTools.DefaultRepoUrl, 32, y, 380);
                _branch = AddBox(UpdateTools.DefaultBranch, 420, y, 120);
                y += 32;
                _presets = new CheckBox();
                _presets.Text = "Also overwrite the homunculus presets (Types\\*.lua); off keeps your own";
                _presets.SetBounds(12, y, 616, 20);
                Controls.Add(_presets);
                y += 28;
            }
            else
            {
                AddLabel("Runs tools\\harness\\rathena_sync.py in your git clone (needs Python and network) to regenerate " +
                    "MobInfo.lua (monster race/element/size/boss flags, element table) from the rAthena databases, then copies " +
                    "MobInfo.lua here. The clone also gets the harness fixtures and webgui\\mobdata.js to commit.",
                    12, y, 616, 64);
                y += 72;
                AddLabel("Git clone folder:", 12, y, 616, 18);
                y += 20;
                _clone = AddBox(clone, 12, y, 520);
                AddBrowse(12 + 528, y);
                y += 32;
                AddLabel("rAthena commit/branch/tag (empty = latest master):", 12, y, 400, 18);
                y += 20;
                _ref = AddBox("", 12, y, 200);
                y += 32;
            }
            _log = new TextBox();
            _log.Multiline = true;
            _log.ReadOnly = true;
            _log.ScrollBars = ScrollBars.Both;
            _log.WordWrap = false;
            _log.Font = new Font(FontFamily.GenericMonospace, 8.5f);
            _log.SetBounds(12, y, 616, ClientSize.Height - y - 52);
            Controls.Add(_log);
            _start = new Button();
            _start.Text = "Start";
            _start.SetBounds(ClientSize.Width - 188, ClientSize.Height - 40, 84, 28);
            _start.Click += delegate { Begin(); };
            Controls.Add(_start);
            _close = new Button();
            _close.Text = "Close";
            _close.SetBounds(ClientSize.Width - 96, ClientSize.Height - 40, 84, 28);
            _close.Click += delegate { Close(); };
            Controls.Add(_close);
            AcceptButton = _start;
            CancelButton = _close;
            FormClosing += delegate(object s, FormClosingEventArgs e) { if (_running) e.Cancel = true; };
        }

        void AddBrowse(int x, int y)
        {
            Button b = new Button();
            b.Text = "Browse...";
            b.SetBounds(x, y - 1, 84, 24);
            b.Click += delegate
            {
                using (FolderBrowserDialog d = new FolderBrowserDialog())
                {
                    d.Description = "Folder of your AzzyAI git clone (contains USER_AI and tools)";
                    if (d.ShowDialog(this) == DialogResult.OK) _clone.Text = d.SelectedPath;
                }
            };
            Controls.Add(b);
        }

        void Log(string line)
        {
            if (IsDisposed) return;
            BeginInvoke(new MethodInvoker(delegate { _log.AppendText(line + "\r\n"); }));
        }

        void Begin()
        {
            bool useGit = _mode == UpdateMode.Databases || _radioGit.Checked;
            string clone = _clone.Text.Trim();
            if (useGit && !UpdateTools.IsClone(clone))
            {
                MessageBox.Show(this, "Pick the folder of your AzzyAI git clone (it contains USER_AI and tools\\harness).",
                    Text, MessageBoxButtons.OK, MessageBoxIcon.Warning);
                return;
            }
            if (useGit) UpdateTools.SaveClone(clone);
            string url = _url != null ? _url.Text.Trim() : "";
            string branch = _branch != null ? _branch.Text.Trim() : "";
            string rathenaRef = _ref != null ? _ref.Text.Trim() : "";
            bool presets = _presets != null && _presets.Checked;
            _start.Enabled = false;
            _running = true;
            _log.Clear();
            Thread t = new Thread(delegate() { Work(useGit, clone, url, branch, rathenaRef, presets); });
            t.IsBackground = true;
            t.Start();
        }

        void Work(bool useGit, string clone, string url, string branch, string rathenaRef, bool presets)
        {
            string temp = Path.Combine(Path.GetTempPath(), "azzyai-update-" + Guid.NewGuid().ToString("N"));
            UpdateResult result = null;
            bool ok = false;
            try
            {
                if (_mode == UpdateMode.Databases)
                {
                    ok = RunSync(clone, rathenaRef);
                    if (ok)
                    {
                        string src = Path.Combine(clone, "USER_AI");
                        result = ApplyOnly(src, "MobInfo.lua");
                    }
                }
                else
                {
                    string src;
                    if (useGit)
                    {
                        ok = UpdateTools.PullClone(clone, Log);
                        src = Path.Combine(clone, "USER_AI");
                    }
                    else
                    {
                        src = UpdateTools.DownloadBranch(url, branch, temp, Log);
                        ok = true;
                    }
                    if (ok) result = UpdateTools.Apply(src, UpdateTools.AiDir, presets, Log);
                }
            }
            catch (Exception ex)
            {
                ok = false;
                Log("ERROR: " + ex.Message);
            }
            try { if (Directory.Exists(temp)) Directory.Delete(temp, true); }
            catch (IOException) { }
            catch (UnauthorizedAccessException) { }
            _running = false;
            if (IsDisposed) return;
            BeginInvoke(new MethodInvoker(delegate { Finished(ok, result); }));
        }

        bool RunSync(string clone, string rathenaRef)
        {
            string[] py = UpdateTools.FindPython();
            if (py == null)
            {
                Log("Python 3 was not found. Install it from python.org (tick \"Add to PATH\") and try again.");
                return false;
            }
            string args = (py[1] + " tools\\harness\\rathena_sync.py").Trim();
            if (rathenaRef.Length > 0) args += " --ref \"" + rathenaRef.Replace("\"", "") + "\"";
            Log("Running: " + py[0] + " " + args);
            if (UpdateTools.Run(py[0], args, clone, Log) != 0)
            {
                Log("rathena_sync.py failed; MobInfo.lua here was not changed.");
                return false;
            }
            return true;
        }

        UpdateResult ApplyOnly(string srcDir, string name)
        {
            // Apply() copies a whole folder, so stage just this file in a one-file folder
            string stage = Path.Combine(Path.GetTempPath(), "azzyai-update-one-" + Guid.NewGuid().ToString("N"));
            Directory.CreateDirectory(stage);
            try
            {
                File.Copy(Path.Combine(srcDir, name), Path.Combine(stage, name), true);
                return UpdateTools.Apply(stage, UpdateTools.AiDir, false, Log);
            }
            finally
            {
                try { Directory.Delete(stage, true); }
                catch (IOException) { }
            }
        }

        void Finished(bool ok, UpdateResult r)
        {
            _start.Enabled = true;
            if (!ok || r == null)
            {
                Log("Update did not finish.");
                return;
            }
            Log(string.Format("Done: {0} updated, {1} added, {2} of your files kept.", r.Updated, r.Added, r.Kept));
            if (r.BackupDir != null) Log("Backup of replaced files: " + r.BackupDir);
            if (_mode == UpdateMode.Databases && r.Updated + r.Added == 0) Log("MobInfo.lua was already up to date.");
            if (r.Updated + r.Added > 0) Log("Reload the AI in the game client for the changes to take effect.");
            if (r.ExeStaged &&
                MessageBox.Show(this, "A newer AzzyAIConfig.exe was downloaded. Restart the GUI now to use it?",
                    Text, MessageBoxButtons.YesNo, MessageBoxIcon.Question) == DialogResult.Yes)
            {
                UpdateTools.RestartWithStagedExe();
                Application.Exit();
            }
        }
    }
}
