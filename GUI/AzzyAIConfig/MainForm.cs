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
            FormClosing += MainForm_FormClosing;
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
            // Set the homunculusConfigControl selected object to the homunculus configurations
            homunculusConfigControl.SelectedObject = _hconf;

            // Set the propertyGridMercenary selected object to the mercanery configurations
            propertyGridMercenary.SelectedObject = _mconf;
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
            MessageBox.Show("AzzyAI LATAM HG\nAI by Dr. Azzy; GUI by Machiavellian and contributors\n\nIncludes current Homunculus S settings and official/private server selection.\n\nIcon by Kami Kali",
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
