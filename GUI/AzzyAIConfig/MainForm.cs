// MainForm.cs
//
// Programmed by Machiavellian of iRO Chaos
//
// Description:
// This file contains partial code for the MainForm class object, which is
// used as the graphical user interface for the AzzyAI configuration
// utility.

using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text;
using System.Windows.Forms;

namespace AzzyAIConfig
{
    public partial class MainForm : Form
    {
        // Custom controls for H_Config and M_Config
        HomConf _hconf = new HomConf();
        MerConf _mconf = new MerConf();

        public MainForm()
        {
            // Pregenerated designer code
            InitializeComponent();
            AddUpdateMenu();
            AddSupportMenu();
            FormClosing += MainForm_FormClosing;
            FormClosing += SaveSession;
            Shown += delegate { LoadSession(); };
        }

        // Support menu: the diagnostics export (one file to attach) and the GitHub issue form
        private void AddSupportMenu()
        {
            ToolStripMenuItem support = new ToolStripMenuItem("Support");
            support.DropDownItems.Add("Export diagnostics for a bug report...", null,
                delegate { using (DiagnosticsForm f = new DiagnosticsForm()) f.ShowDialog(this); });
            support.DropDownItems.Add("Copy version and startup summary", null, delegate
            {
                try
                {
                    string logDir;
                    List<DiagnosticsEntry> entries = Diagnostics.Collect(out logDir);
                    Clipboard.SetText(Diagnostics.Summary(entries, logDir));
                }
                catch (Exception e)
                {
                    MessageBox.Show(this, "Could not read the summary: " + e.Message, "Support",
                        MessageBoxButtons.OK, MessageBoxIcon.Warning);
                }
            });
            support.DropDownItems.Add("Report a problem on GitHub", null, delegate
            {
                try
                {
                    System.Diagnostics.Process.Start(Diagnostics.IssueUrl);
                }
                catch (Exception e)
                {
                    MessageBox.Show(this, "Could not open the browser: " + e.Message + "\n\n" + Diagnostics.IssueUrl,
                        "GitHub", MessageBoxButtons.OK, MessageBoxIcon.Warning);
                }
            });
            ToolStripItem help = null;
            foreach (ToolStripItem item in MainMenuStrip.Items)
                if (item.Text == "Help") help = item;
            MainMenuStrip.Items.Insert(help == null ? MainMenuStrip.Items.Count : MainMenuStrip.Items.IndexOf(help), support);
        }

        // The Update menu is built here instead of in the designer file
        private void AddUpdateMenu()
        {
            ToolStripMenuItem update = new ToolStripMenuItem("Update");
            update.DropDownItems.Add("Update AI from git...", null,
                delegate { using (UpdateForm f = new UpdateForm(UpdateMode.Ai)) f.ShowDialog(this); });
            update.DropDownItems.Add("Update monster/skill data from rAthena...", null,
                delegate { using (UpdateForm f = new UpdateForm(UpdateMode.Databases)) f.ShowDialog(this); });
            ToolStripItem help = null;
            foreach (ToolStripItem item in MainMenuStrip.Items)
                if (item.Text == "Help") help = item;
            MainMenuStrip.Items.Insert(help == null ? MainMenuStrip.Items.Count : MainMenuStrip.Items.IndexOf(help), update);
        }

        private bool TrySaveChanges()
        {
            try
            {
                SaveChanges();
                buttonApply.Enabled = false;
                applySettingsToolStripMenuItem.Enabled = false;
                revertToolStripMenuItem.Enabled = false;
                return true;
            }
            catch (IOException e)
            {
                ShowSaveError(e);
            }
            catch (UnauthorizedAccessException e)
            {
                ShowSaveError(e);
            }
            catch (InvalidOperationException e)
            {
                ShowSaveError(e);
            }
            return false;
        }

        private void ShowSaveError(Exception error)
        {
            MessageBox.Show(this, "Settings could not be saved completely. Your edits are still open.\n\n" + error.Message,
                "Save failed", MessageBoxButtons.OK, MessageBoxIcon.Error);
        }

        // Window size, position and the open tab survive a restart; a monitor change must not hide
        // the window off-screen, so the saved bounds are clamped to the current screen.
        static string SessionPath
        {
            get { return Path.Combine(UpdateTools.AiDir, "AzzyAIConfig.session.txt"); }
        }

        void LoadSession()
        {
            try
            {
                if (!File.Exists(SessionPath)) return;
                Dictionary<string, string> values = new Dictionary<string, string>();
                foreach (string line in File.ReadAllLines(SessionPath))
                {
                    int eq = line.IndexOf('=');
                    if (eq > 0) values[line.Substring(0, eq).Trim()] = line.Substring(eq + 1).Trim();
                }
                int w, h, x, y, tab;
                if (int.TryParse(Value(values, "width"), out w) && int.TryParse(Value(values, "height"), out h)
                    && int.TryParse(Value(values, "x"), out x) && int.TryParse(Value(values, "y"), out y)
                    && w >= 640 && h >= 480 && w <= 4000 && h <= 3000)
                {
                    Rectangle wanted = new Rectangle(x, y, w, h);
                    Rectangle area = Screen.FromRectangle(wanted).WorkingArea;
                    w = Math.Min(w, area.Width);
                    h = Math.Min(h, area.Height);
                    x = Math.Max(area.Left, Math.Min(x, area.Right - w));
                    y = Math.Max(area.Top, Math.Min(y, area.Bottom - h));
                    StartPosition = FormStartPosition.Manual;
                    Bounds = new Rectangle(x, y, w, h);
                }
                TabControl tabs = FindTabs(this);
                if (tabs != null && int.TryParse(Value(values, "tab"), out tab) && tab >= 0 && tab < tabs.TabCount)
                    tabs.SelectedIndex = tab;
            }
            catch (IOException) { }
            catch (UnauthorizedAccessException) { }
        }

        void SaveSession(object sender, FormClosingEventArgs e)
        {
            try
            {
                Rectangle bounds = WindowState == FormWindowState.Normal ? Bounds : RestoreBounds;
                TabControl tabs = FindTabs(this);
                File.WriteAllLines(SessionPath, new string[]
                {
                    "width=" + bounds.Width,
                    "height=" + bounds.Height,
                    "x=" + bounds.X,
                    "y=" + bounds.Y,
                    "tab=" + (tabs == null ? 0 : tabs.SelectedIndex)
                });
            }
            catch (IOException) { }
            catch (UnauthorizedAccessException) { }
        }

        // The designer keeps the tab control in a local variable, so it is looked up by type.
        static TabControl FindTabs(Control parent)
        {
            TabControl tabs = parent as TabControl;
            if (tabs != null) return tabs;
            foreach (Control child in parent.Controls)
            {
                tabs = FindTabs(child);
                if (tabs != null) return tabs;
            }
            return null;
        }

        static string Value(Dictionary<string, string> values, string key)
        {
            string v;
            return values.TryGetValue(key, out v) ? v : "";
        }

        private void MainForm_FormClosing(object sender, FormClosingEventArgs e)
        {
            if (!buttonApply.Enabled) return;
            var result = MessageBox.Show(this, "Save your configuration changes before closing?",
                "Unsaved changes", MessageBoxButtons.YesNoCancel, MessageBoxIcon.Question);
            e.Cancel = result == DialogResult.Cancel || (result == DialogResult.Yes && !TrySaveChanges());
        }

        void SaveChanges()
        {
            // Save the configurations
            _hconf.Save();
            _mconf.Save();
            homTactControl1.Save();
            merTactControl1.Save();
            extraControl1.Save();
            pvpTactControl1.Save();
            m_PvpTactControl1.Save();
        }

        private void MainForm_Load(object sender, EventArgs e)
        {
            homunculusConfigControl.ScopeChanging += homunculusConfigControl_ScopeChanging;
            homunculusConfigControl.ScopeChanged += homunculusConfigControl_ScopeChanged;
            // Set the homunculusConfigControl selected object to the homunculus configurations
            homunculusConfigControl.SelectedObject = _hconf;

            // Set the propertyGridMercenary selected object to the mercanery configurations
            propertyGridMercenary.SelectedObject = _mconf;
        }

        private void homunculusConfigControl_ScopeChanging(object sender, System.ComponentModel.CancelEventArgs e)
        {
            if (!buttonApply.Enabled) return;
            var result = MessageBox.Show(this, "Save your configuration changes before switching settings?",
                "Unsaved changes", MessageBoxButtons.YesNoCancel, MessageBoxIcon.Question);
            e.Cancel = result == DialogResult.Cancel || (result == DialogResult.Yes && !TrySaveChanges());
        }

        private void homunculusConfigControl_ScopeChanged(object sender, EventArgs e)
        {
            // The new scope was just loaded, so nothing is pending
            buttonApply.Enabled = false;
            applySettingsToolStripMenuItem.Enabled = false;
            revertToolStripMenuItem.Enabled = false;
        }

        private void ConfigChanged(object sender, EventArgs e)
        {
            // Check if buttonApply is not enabled
            if (!buttonApply.Enabled)
            {
                // Enable buttonApply
                buttonApply.Enabled = true;
            }

            // Check if the applySettingsToolStripMenuItem is not enabled
            if (!applySettingsToolStripMenuItem.Enabled)
            {
                // Enable applySettingsToolStripMenuItem
                applySettingsToolStripMenuItem.Enabled = true;
            }

            // Check if the revertToolStripMenuItem is not enabled
            if (!revertToolStripMenuItem.Enabled)
            {
                // Enable reverToolStripMenuItem
                revertToolStripMenuItem.Enabled = true;
            }
        }

        private void homunculusConfigControl_PropertyValueChanged(object sender, EventArgs e)
        {
            // Check if buttonApply is not enabled
            if (!buttonApply.Enabled)
            {
                // Enable buttonApply
                buttonApply.Enabled = true;
            }

            // Check if the applySettingsToolStripMenuItem is not enabled
            if (!applySettingsToolStripMenuItem.Enabled)
            {
                // Enable applySettingsToolStripMenuItem
                applySettingsToolStripMenuItem.Enabled = true;
            }

            // Check if the revertToolStripMenuItem is not enabled
            if (!revertToolStripMenuItem.Enabled)
            {
                // Enable reverToolStripMenuItem
                revertToolStripMenuItem.Enabled = true;
            }
        }

        private void propertyGridHomunculus_PropertyValueChanged(object s, PropertyValueChangedEventArgs e)
        {
            // Check if buttonApply is not enabled
            if (!buttonApply.Enabled)
            {
                // Enable buttonApply
                buttonApply.Enabled = true;
            }

            // Check if the applySettingsToolStripMenuItem is not enabled
            if (!applySettingsToolStripMenuItem.Enabled)
            {
                // Enable applySettingsToolStripMenuItem
                applySettingsToolStripMenuItem.Enabled = true;
            }

            // Check if the revertToolStripMenuItem is not enabled
            if (!revertToolStripMenuItem.Enabled)
            {
                // Enable reverToolStripMenuItem
                revertToolStripMenuItem.Enabled = true;
            }
        }

        private void propertyGridMercenary_PropertyValueChanged(object s, PropertyValueChangedEventArgs e)
        {
            // Check if buttonApply is not enabled
            if (!buttonApply.Enabled)
            {
                // Enable buttonApply
                buttonApply.Enabled = true;
            }

            // Check if the applySettingsToolStripMenuItem is not enabled
            if (!applySettingsToolStripMenuItem.Enabled)
            {
                // Enable applySettingsToolStripMenuItem
                applySettingsToolStripMenuItem.Enabled = true;
            }

            // Check if the revertToolStripMenuItem is not enabled
            if (!revertToolStripMenuItem.Enabled)
            {
                // Enable reverToolStripMenuItem
                revertToolStripMenuItem.Enabled = true;
            }
        }

        private void buttonQuit_Click(object sender, EventArgs e)
        {
            Close();
        }

        private void buttonApply_Click(object sender, EventArgs e)
        {
            TrySaveChanges();
        }

        private void revertToolStripMenuItem_Click(object sender, EventArgs e)
        {
            // Revert the homunculus configurations
            _hconf.Revert();

            // Revert the mercenary configurations
            _mconf.Revert();
            
            // Refresh the controls to show reverted values
            homunculusConfigControl.Refresh();
            propertyGridMercenary.Refresh();
            ConfigChanged(sender, e);
        }

        private void documentationToolStripMenuItem_Click(object sender, EventArgs e)
        {
            // Check if the documentation file exists
            if (System.IO.File.Exists("Documentation.pdf"))
            {
                // Start a new process to open the documentation file
                System.Diagnostics.Process p = new System.Diagnostics.Process();
                p.StartInfo.FileName = "Documentation.pdf";
                p.Start();
            }
            // If the documentation file does not exist
            else
            {
                // Popup an error message for the documentation file
                MessageBox.Show("Documentation file could not be found.", "Error!", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        private void aboutToolStripMenuItem_Click(object sender, EventArgs e)
        {
            // Create a new AzzyAIAboutBox
            //AzzyAIAboutBox about = new AzzyAIAboutBox();
            // Show the about box
            //about.ShowDialog();
            MessageBox.Show("AzzyAI " + Application.ProductVersion + "\nAI by Dr. Azzy; GUI by Machiavellian and contributors\n\nIncludes current Homunculus S settings and official/private server selection.\n\nIcon by Kami Kali\n\nRepository: " + Diagnostics.RepoUrl
                + "\nIssues: " + Diagnostics.IssueUrl + "\nWeb GUI: https://jknpj.github.io/AzzyAI-webgui/",
                "About AzzyAI", MessageBoxButtons.OK, MessageBoxIcon.Information);
        }

        private void homunculusSettingsToolStripMenuItem_Click(object sender, EventArgs e)
        {
            // Create a new open file dialog box
            OpenFileDialog ofd = new OpenFileDialog();

            // Turn on validate names
            ofd.ValidateNames = true;

            // Add a filter for lua files
            ofd.Filter = "(*.lua)|*.lua";

            // Set the open file dialog box title to "Import homunculus settings."
            ofd.Title = "Import homunculus settings.";

            // Show the dialog and check if the result is OK
            if (ofd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
            {
                // Open the file
                _hconf.Open(ofd.FileName);

                // Update the homunculusConfigControl values
                homunculusConfigControl.UpdateData();
                ConfigChanged(sender, e);
            }
        }

        private void mercenarySettingsToolStripMenuItem_Click(object sender, EventArgs e)
        {
            // Create a new open file dialog box
            OpenFileDialog ofd = new OpenFileDialog();

            // Turn on validate names
            ofd.ValidateNames = true;

            // Add a filter for lua files
            ofd.Filter = "(*.lua)|*.lua";

            // Set the open file dialog box title to "Import mercenary settings."
            ofd.Title = "Import mercenary settings.";

            // Show the dialog and check if the result is OK
            if (ofd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
            {
                // Open the file
                _mconf.Open(ofd.FileName);

                // Update the propertyGridHomunculus values
                propertyGridMercenary.Update();
                ConfigChanged(sender, e);
            }
        }

        private void homunculusTacticsToolStripMenuItem_Click(object sender, EventArgs e)
        {
            // Create a new open file dialog box
            OpenFileDialog ofd = new OpenFileDialog();

            // Turn on validate names
            ofd.ValidateNames = true;

            // Add a filter for lua files
            ofd.Filter = "(*.lua)|*.lua";

            // Set the open file dialog box title to "Import homunculus tactics."
            ofd.Title = "Import homunculus tactics.";

            // Show the dialog and check if the result is OK
            if (ofd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
            {
                // Open the file
                homTactControl1.Open(ofd.FileName);
            }
        }

        private void mercenaryTacticsToolStripMenuItem_Click(object sender, EventArgs e)
        {
            // Create a new open file dialog box
            OpenFileDialog ofd = new OpenFileDialog();

            // Turn on validate names
            ofd.ValidateNames = true;

            // Set the open file dialog box title to "Import mercenary tactics."
            ofd.Title = "Import mercenary tactics.";

            // Add a filter for lua files
            ofd.Filter = "(*.lua)|*.lua";

            // Show the dialog and check if the result is OK
            if (ofd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
            {
                // Open the file
                merTactControl1.Open(ofd.FileName);
            }
        }

        private void homunculusSettingsToolStripMenuItem1_Click(object sender, EventArgs e)
        {
            // Create a new save file dialog box
            SaveFileDialog sfd = new SaveFileDialog();

            // Turn on validate names
            sfd.ValidateNames = true;

            // Set the open file dialog box title to "Export homunculus settings."
            sfd.Title = "Export homunculus settings.";

            // Add a filter for lua files
            sfd.Filter = "(*.lua)|*.lua";

            // Show the dialog and check if the result is OK
            if (sfd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
            {
                // Save the file
                _hconf.Save(sfd.FileName);
            }
        }

        private void mercenarySettingsToolStripMenuItem1_Click(object sender, EventArgs e)
        {
            // Create a new save file dialog box
            SaveFileDialog sfd = new SaveFileDialog();

            // Turn on validate names
            sfd.ValidateNames = true;

            // Set the open file dialog box title to "Export mercenary settings."
            sfd.Title = "Export mercenary settings.";

            // Add a filter for lua files
            sfd.Filter = "(*.lua)|*.lua";

            // Show the dialog and check if the result is OK
            if (sfd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
            {
                // Save the file
                _mconf.Save(sfd.FileName);
            }
        }

        private void homunculusTacticsToolStripMenuItem1_Click(object sender, EventArgs e)
        {
            // Create a new save file dialog box
            SaveFileDialog sfd = new SaveFileDialog();

            // Turn on validate names
            sfd.ValidateNames = true;

            // Set the open file dialog box title to "Export homunculus tactics."
            sfd.Title = "Export homunculus tactics.";

            // Add a filter for lua files
            sfd.Filter = "(*.lua)|*.lua";

            // Show the dialog and check if the result is OK
            if (sfd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
            {
                // Save the file
                homTactControl1.Save(sfd.FileName);
            }
        }

        private void mercenaryTacticsToolStripMenuItem1_Click(object sender, EventArgs e)
        {
            // Create a new save file dialog box
            SaveFileDialog sfd = new SaveFileDialog();

            // Turn on validate names
            sfd.ValidateNames = true;

            // Set the open file dialog box title to "Export mercenary tactics."
            sfd.Title = "Export homunculus tactics.";

            // Add a filter for lua files
            sfd.Filter = "(*.lua)|*.lua";

            // Show the dialog and check if the result is OK
            if (sfd.ShowDialog() == System.Windows.Forms.DialogResult.OK)
            {
                // Save the file
                merTactControl1.Save(sfd.FileName);
            }
        }

        private void tabPage3_Click(object sender, EventArgs e)
        {

        }

        private void homTactControl1_Load(object sender, EventArgs e)
        {

        }

        private void label1_Click(object sender, EventArgs e)
        {

        }


    }
}
