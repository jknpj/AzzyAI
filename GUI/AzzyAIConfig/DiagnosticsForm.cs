// DiagnosticsForm.cs
//
// The dialog behind Support > "Export diagnostics for a bug report...". It shows what the bundle
// contains, writes it, and points the user to the GitHub issue form. Nothing leaves the computer
// without the user attaching the file.

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Windows.Forms;

namespace AzzyAIConfig
{
    public class DiagnosticsForm : Form
    {
        readonly TextBox _text = new TextBox();
        readonly CheckBox _reviewed = new CheckBox();
        readonly Button _export = new Button();
        readonly Button _copy = new Button();
        readonly Button _issue = new Button();
        readonly Button _close = new Button();
        readonly Label _steps = new Label();

        List<DiagnosticsEntry> _entries;
        string _logDir;
        string _summary = "";
        string _exported;

        public DiagnosticsForm()
        {
            Text = "Export diagnostics";
            StartPosition = FormStartPosition.CenterParent;
            ClientSize = new Size(760, 560);
            MinimumSize = new Size(520, 380);

            _steps.AutoSize = false;
            _steps.Dock = DockStyle.Top;
            _steps.Height = 42;
            _steps.Padding = new Padding(8, 6, 8, 0);
            _steps.Text = "1. Export the file.  2. In the issue form, attach it.  3. Describe what happened.\r\n" +
                "The startup report, the AI logs, your configuration and the learned skill levels go in the bundle.";

            _text.Dock = DockStyle.Fill;
            _text.Multiline = true;
            _text.ReadOnly = true;
            _text.WordWrap = false;
            _text.ScrollBars = ScrollBars.Both;
            _text.Font = new Font("Consolas", 9F);
            _text.BackColor = SystemColors.Window;

            Panel bottom = new Panel();
            bottom.Dock = DockStyle.Bottom;
            bottom.Height = 76;

            _reviewed.Dock = DockStyle.Top;
            _reviewed.Height = 26;
            _reviewed.Padding = new Padding(8, 0, 8, 0);
            _reviewed.Text = "I reviewed the files and removed anything personal (owner IDs are already replaced by <owner>).";
            _reviewed.CheckedChanged += delegate { UpdateButtons(); };

            FlowLayoutPanel buttons = new FlowLayoutPanel();
            buttons.Dock = DockStyle.Fill;
            buttons.FlowDirection = FlowDirection.RightToLeft;
            buttons.Padding = new Padding(8, 6, 8, 8);
            buttons.WrapContents = false;

            _close.Text = "Close";
            _close.Width = 88;
            _close.DialogResult = DialogResult.Cancel;

            _issue.Text = "Open a GitHub issue";
            _issue.Width = 150;
            _issue.Click += delegate { OpenIssue(); };

            _copy.Text = "Copy summary";
            _copy.Width = 116;
            _copy.Click += delegate { CopySummary(); };

            _export.Text = "Export .zip";
            _export.Width = 110;
            _export.Click += delegate { Export(); };

            buttons.Controls.Add(_close);
            buttons.Controls.Add(_issue);
            buttons.Controls.Add(_copy);
            buttons.Controls.Add(_export);

            bottom.Controls.Add(buttons);
            bottom.Controls.Add(_reviewed);

            Controls.Add(_text);
            Controls.Add(bottom);
            Controls.Add(_steps);
            CancelButton = _close;

            Load += delegate { LoadEntries(); };
            UpdateButtons();
        }

        void LoadEntries()
        {
            try
            {
                _entries = Diagnostics.Collect(out _logDir);
                _summary = Diagnostics.Summary(_entries, _logDir);
                _text.Text = _summary.Replace("\n", "\r\n");
            }
            catch (Exception e)
            {
                _entries = new List<DiagnosticsEntry>();
                _summary = "";
                _text.Text = "Could not read the files: " + e.Message;
            }
        }

        void UpdateButtons()
        {
            bool ready = _reviewed.Checked && _entries != null;
            _export.Enabled = ready;
            _issue.Enabled = ready;
            _copy.Enabled = _summary.Length > 0;
        }

        void CopySummary()
        {
            try
            {
                Clipboard.SetText(_summary);
                _copy.Text = "Copied";
            }
            catch (Exception)
            {
                MessageBox.Show(this, "The clipboard is busy; try again.", "Copy summary",
                    MessageBoxButtons.OK, MessageBoxIcon.Warning);
            }
        }

        void Export()
        {
            try
            {
                Cursor = Cursors.WaitCursor;
                _exported = Diagnostics.Export(_entries, _logDir);
            }
            catch (Exception e)
            {
                Cursor = Cursors.Default;
                MessageBox.Show(this, "Could not write the diagnostics: " + e.Message, "Export diagnostics",
                    MessageBoxButtons.OK, MessageBoxIcon.Error);
                return;
            }
            Cursor = Cursors.Default;
            ShowInExplorer(_exported);
            MessageBox.Show(this, "Saved to:\n" + _exported + "\n\nAttach this file to the issue.",
                "Export diagnostics", MessageBoxButtons.OK, MessageBoxIcon.Information);
        }

        void OpenIssue()
        {
            string title = "AzzyAI " + Diagnostics.Release() + ": ";
            string url = Diagnostics.IssueUrl + "&title=" + Uri.EscapeDataString(title);
            try
            {
                Process.Start(url);
            }
            catch (Exception e)
            {
                MessageBox.Show(this, "Could not open the browser: " + e.Message + "\n\n" + url,
                    "GitHub issue", MessageBoxButtons.OK, MessageBoxIcon.Warning);
            }
        }

        static void ShowInExplorer(string path)
        {
            try
            {
                Process.Start("explorer.exe", "/select,\"" + path + "\"");
            }
            catch (Exception)
            {
                // the message box already shows the path
            }
        }
    }
}
