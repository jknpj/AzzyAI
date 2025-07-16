using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Drawing;
using System.Linq;
using System.Reflection;
using System.Windows.Forms;
using AzzyAIConfig.Properties;

namespace AzzyAIConfig
{
    public partial class HomunculusConfigControl : UserControl
    {
        private HomConf _hconf;
        private TabControl tabControl;
        private TextBox searchBox;
        private ComboBox homunculusSFilter;
        private ComboBox homunculusBaseFilter;
        private Label searchLabel;
        private Label homunculusSLabel;
        private Label homunculusBaseLabel;
        private TextBox helpTextBox;
        private Label helpLabel;
        private Dictionary<string, FilteredHomConfWrapper> categoryWrappers;

        public event EventHandler PropertyValueChanged;

        public HomunculusConfigControl()
        {
            InitializeComponent();
            SetupUI();
            categoryWrappers = new Dictionary<string, FilteredHomConfWrapper>();
        }

        internal HomConf SelectedObject
        {
            get { return _hconf; }
            set
            {
                _hconf = value;
                RefreshAllTabs();
            }
        }

        private void InitializeComponent()
        {
            this.SuspendLayout();
            
            // 
            // HomunculusConfigControl
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.Name = "HomunculusConfigControl";
            this.Size = new System.Drawing.Size(604, 351);
            
            this.ResumeLayout(false);
        }

        private void SetupUI()
        {
            // Create search and filter controls
            searchLabel = new Label
            {
                Text = "Search:",
                Location = new Point(10, 10),
                Size = new Size(50, 20),
                TextAlign = ContentAlignment.MiddleLeft
            };

            searchBox = new TextBox
            {
                Location = new Point(70, 10),
                Size = new Size(150, 20)
            };
            searchBox.TextChanged += SearchBox_TextChanged;

            homunculusSLabel = new Label
            {
                Text = "Homunc S:",
                Location = new Point(240, 10),
                Size = new Size(60, 20),
                TextAlign = ContentAlignment.MiddleLeft
            };

            homunculusSFilter = new ComboBox
            {
                Location = new Point(305, 10),
                Size = new Size(80, 20),
                DropDownStyle = ComboBoxStyle.DropDownList
            };
            homunculusSFilter.Items.AddRange(new string[] { "All Types", "Sera", "Eira", "Eleanor", "Bayeri", "Dieter" });
            homunculusSFilter.SelectedIndex = 0;
            homunculusSFilter.SelectedIndexChanged += Filter_Changed;

            homunculusBaseLabel = new Label
            {
                Text = "Homunc Base:",
                Location = new Point(400, 10),
                Size = new Size(80, 20),
                TextAlign = ContentAlignment.MiddleLeft
            };

            homunculusBaseFilter = new ComboBox
            {
                Location = new Point(485, 10),
                Size = new Size(80, 20),
                DropDownStyle = ComboBoxStyle.DropDownList
            };
            homunculusBaseFilter.Items.AddRange(new string[] { "All Types", "Lif", "Amistr", "Filir", "Vanilmirth" });
            homunculusBaseFilter.SelectedIndex = 0;
            homunculusBaseFilter.SelectedIndexChanged += Filter_Changed;

            // Create tab control
            tabControl = new TabControl
            {
                Location = new Point(10, 40),
                Size = new Size(this.Width - 20, this.Height - 150),
                Anchor = AnchorStyles.Top | AnchorStyles.Left | AnchorStyles.Right | AnchorStyles.Bottom
            };

            // Create tabs for each category
            string[] categories = {
                "Basic Options",
                "AutoSkill Options", 
                "Walk/Follow Options",
                "Autobuff Options",
                "Berserk Options",
                "Friending Options",
                "PVP Options",
                "Standby Options",
                "Kiting Options"
            };

            foreach (string category in categories)
            {
                TabPage tabPage = new TabPage(category);
                PropertyGrid propertyGrid = new PropertyGrid
                {
                    Dock = DockStyle.Fill,
                    PropertySort = PropertySort.Alphabetical,
                    ToolbarVisible = false,
                    HelpVisible = false
                };
                propertyGrid.PropertyValueChanged += PropertyGrid_PropertyValueChanged;
                propertyGrid.SelectedGridItemChanged += PropertyGrid_SelectedGridItemChanged;
                
                tabPage.Controls.Add(propertyGrid);
                tabControl.TabPages.Add(tabPage);
            }

            // Create help text area
            helpLabel = new Label
            {
                Text = "Description:",
                Location = new Point(10, this.Height - 100),
                Size = new Size(80, 20),
                Anchor = AnchorStyles.Bottom | AnchorStyles.Left,
                TextAlign = ContentAlignment.MiddleLeft
            };

            helpTextBox = new TextBox
            {
                Location = new Point(10, this.Height - 80),
                Size = new Size(this.Width - 20, 70),
                Anchor = AnchorStyles.Bottom | AnchorStyles.Left | AnchorStyles.Right,
                Multiline = true,
                ReadOnly = true,
                ScrollBars = ScrollBars.Vertical,
                Text = "Select a property to see its description here."
            };

            // Add controls to the user control
            this.Controls.Add(searchLabel);
            this.Controls.Add(searchBox);
            this.Controls.Add(homunculusSLabel);
            this.Controls.Add(homunculusSFilter);
            this.Controls.Add(homunculusBaseLabel);
            this.Controls.Add(homunculusBaseFilter);
            this.Controls.Add(tabControl);
            this.Controls.Add(helpLabel);
            this.Controls.Add(helpTextBox);

            // Load saved filter settings
            LoadFilterSettings();
        }

        private void SearchBox_TextChanged(object sender, EventArgs e)
        {
            ApplyFilters();
        }

        private void Filter_Changed(object sender, EventArgs e)
        {
            ApplyFilters();
            SaveFilterSettings();
        }

        private void PropertyGrid_PropertyValueChanged(object s, PropertyValueChangedEventArgs e)
        {
            if (PropertyValueChanged != null)
                PropertyValueChanged(s, e);
        }

        private void PropertyGrid_SelectedGridItemChanged(object sender, SelectedGridItemChangedEventArgs e)
        {
            if (e.NewSelection != null && e.NewSelection.PropertyDescriptor != null)
            {
                var descriptor = e.NewSelection.PropertyDescriptor;
                var descriptionAttribute = descriptor.Attributes[typeof(DescriptionAttribute)] as DescriptionAttribute;
                
                if (descriptionAttribute != null && !string.IsNullOrEmpty(descriptionAttribute.Description))
                {
                    helpTextBox.Text = descriptionAttribute.Description;
                }
                else
                {
                    helpTextBox.Text = "No description available for this property.";
                }
            }
            else
            {
                helpTextBox.Text = "Select a property to see its description here.";
            }
        }

        private void LoadFilterSettings()
        {
            try
            {
                // Load saved filter settings
                string savedHomunculusS = Properties.Settings.Default.HomunculusSFilter;
                string savedHomunculusBase = Properties.Settings.Default.HomunculusBaseFilter;

                if (!string.IsNullOrEmpty(savedHomunculusS))
                {
                    for (int i = 0; i < homunculusSFilter.Items.Count; i++)
                    {
                        if (homunculusSFilter.Items[i].ToString() == savedHomunculusS)
                        {
                            homunculusSFilter.SelectedIndex = i;
                            break;
                        }
                    }
                }

                if (!string.IsNullOrEmpty(savedHomunculusBase))
                {
                    for (int i = 0; i < homunculusBaseFilter.Items.Count; i++)
                    {
                        if (homunculusBaseFilter.Items[i].ToString() == savedHomunculusBase)
                        {
                            homunculusBaseFilter.SelectedIndex = i;
                            break;
                        }
                    }
                }
            }
            catch
            {
                // If settings don't exist or are invalid, use defaults
                homunculusSFilter.SelectedIndex = 0;
                homunculusBaseFilter.SelectedIndex = 0;
            }
        }

        private void SaveFilterSettings()
        {
            try
            {
                Properties.Settings.Default.HomunculusSFilter = homunculusSFilter.SelectedItem != null ? homunculusSFilter.SelectedItem.ToString() : "";
                Properties.Settings.Default.HomunculusBaseFilter = homunculusBaseFilter.SelectedItem != null ? homunculusBaseFilter.SelectedItem.ToString() : "";
                Properties.Settings.Default.Save();
            }
            catch
            {
                // Ignore save errors
            }
        }

        private void RefreshAllTabs()
        {
            if (_hconf == null) return;

            categoryWrappers.Clear();
            foreach (TabPage tabPage in tabControl.TabPages)
            {
                PropertyGrid propertyGrid = tabPage.Controls[0] as PropertyGrid;
                if (propertyGrid != null)
                {
                    ApplyFilterToPropertyGrid(propertyGrid, tabPage.Text);
                }
            }
        }

        private void ApplyFilters()
        {
            if (_hconf == null) return;

            foreach (TabPage tabPage in tabControl.TabPages)
            {
                PropertyGrid propertyGrid = tabPage.Controls[0] as PropertyGrid;
                if (propertyGrid != null)
                {
                    ApplyFilterToPropertyGrid(propertyGrid, tabPage.Text);
                }
            }
        }

        private void ApplyFilterToPropertyGrid(PropertyGrid propertyGrid, string category)
        {
            string searchText = searchBox != null && searchBox.Text != null ? searchBox.Text.ToLower() : "";
            string homunculusSType = homunculusSFilter != null && homunculusSFilter.SelectedItem != null ? homunculusSFilter.SelectedItem.ToString() : "All";
            string homunculusBaseType = homunculusBaseFilter != null && homunculusBaseFilter.SelectedItem != null ? homunculusBaseFilter.SelectedItem.ToString() : "All";

            // Create or get the filtered wrapper object
            string key = category;
            if (!categoryWrappers.ContainsKey(key))
            {
                categoryWrappers[key] = new FilteredHomConfWrapper(_hconf, category);
            }

            var wrapper = categoryWrappers[key];
            wrapper.UpdateFilters(searchText, homunculusSType, homunculusBaseType);
            propertyGrid.SelectedObject = wrapper;
        }

        public new void Refresh()
        {
            RefreshAllTabs();
        }

        public void UpdateData()
        {
            RefreshAllTabs();
        }
    }

    // Custom wrapper class that implements filtering using TypeDescriptor
    [TypeDescriptionProvider(typeof(FilteredHomConfTypeDescriptionProvider))]
    internal class FilteredHomConfWrapper : ICustomTypeDescriptor
    {
        private HomConf _originalConf;
        private string _category;
        private string _searchText = "";
        private string _homunculusSType = "All";
        private string _homunculusBaseType = "All";
        private PropertyDescriptorCollection _filteredProperties;

        public FilteredHomConfWrapper(HomConf originalConf, string category)
        {
            _originalConf = originalConf;
            _category = category;
            UpdateFilteredProperties();
        }

        public void UpdateFilters(string searchText, string homunculusSType, string homunculusBaseType)
        {
            _searchText = searchText;
            _homunculusSType = homunculusSType;
            _homunculusBaseType = homunculusBaseType;
            UpdateFilteredProperties();
        }

        private void UpdateFilteredProperties()
        {
            if (_originalConf == null)
            {
                _filteredProperties = new PropertyDescriptorCollection(null);
                return;
            }

            List<PropertyDescriptor> filteredProps = new List<PropertyDescriptor>();
            PropertyDescriptorCollection originalProps = TypeDescriptor.GetProperties(_originalConf);

            foreach (PropertyDescriptor prop in originalProps)
            {
                // Check category
                if (prop.Category != _category)
                    continue;

                // Apply search filter
                if (!string.IsNullOrEmpty(_searchText))
                {
                    string propName = prop.Name.ToLower();
                    string displayName = prop.DisplayName.ToLower();
                    
                    if (!propName.Contains(_searchText) && !displayName.Contains(_searchText))
                        continue;
                }

                // Apply Homunculus type filters
                if (!ShouldShowProperty(prop.Name, _homunculusSType, _homunculusBaseType))
                    continue;

                // Create a wrapper descriptor that redirects to the original object
                filteredProps.Add(new WrappedPropertyDescriptor(prop, _originalConf));
            }

            _filteredProperties = new PropertyDescriptorCollection(filteredProps.ToArray());
        }

        private string GetTypeName(string filterValue)
        {
            return filterValue;
        }

        private bool ShouldShowProperty(string propertyName, string homunculusSType, string homunculusBaseType)
        {
            string propNameLower = propertyName.ToLower();

            // Check if property belongs to any specific homunculus type
            string[] homunculusSTypes = { "eira", "eleanor", "dieter", "bayeri", "sera" };
            string[] homunculusBaseTypes = { "amistr", "vanilmirth", "filir", "lif" };
            
            // Check for Homunculus S specific skills
            string[] seraSkills = { "paralyze", "poisonmist", "painkiller", "calllegion" };
            string[] eiraSkills = { "silentbreeze", "xenoslasher", "erasercutter", "overedboost", "regene" };
            string[] eleanorSkills = { "sonicclaw", "silvervein", "midnight", "tinderbreaker", "switchmode" };
            string[] bayeriSkills = { "stahlhorn", "hailege", "goldene", "steinwand", "angriffs" };
            string[] dieterSkills = { "lavaslide", "magmaflow", "granitic", "pyroclastic", "volcanic" };
            
            // Check for Base Homunculus specific skills
            string[] lifSkills = { "escape", "breeze" };
            string[] amistrSkills = { "bulwark", "castle", "bloodlust" };
            string[] filirSkills = { "flit", "accel", "moon", "speed" };
            string[] vanilmirthSkills = { "caprice", "chaotic", "selfdestruct" };

            // Check if this property belongs to a specific Homunculus S type
            string belongsToHomunculusS = null;
            if (propNameLower.Contains("sera") || seraSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusS = "sera";
            else if (propNameLower.Contains("eira") || eiraSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusS = "eira";
            else if (propNameLower.Contains("eleanor") || eleanorSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusS = "eleanor";
            else if (propNameLower.Contains("bayeri") || bayeriSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusS = "bayeri";
            else if (propNameLower.Contains("dieter") || dieterSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusS = "dieter";

            // Check if this property belongs to a specific Base Homunculus type
            string belongsToHomunculusBase = null;
            if (propNameLower.Contains("lif") || lifSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusBase = "lif";
            else if (propNameLower.Contains("amistr") || amistrSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusBase = "amistr";
            else if (propNameLower.Contains("filir") || filirSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusBase = "filir";
            else if (propNameLower.Contains("vanilmirth") || vanilmirthSkills.Any(skill => propNameLower.Contains(skill)))
                belongsToHomunculusBase = "vanilmirth";

            // Apply Homunculus S filter
            if (homunculusSType != "All Types")
            {
                string selectedType = GetTypeName(homunculusSType).ToLower();
                
                // If property belongs to a specific Homunculus S and it's not the selected one, hide it
                if (belongsToHomunculusS != null && belongsToHomunculusS != selectedType)
                    return false;
            }

            // Apply Base Homunculus filter
            if (homunculusBaseType != "All Types")
            {
                string selectedType = GetTypeName(homunculusBaseType).ToLower();
                
                // If property belongs to a specific Base Homunculus and it's not the selected one, hide it
                if (belongsToHomunculusBase != null && belongsToHomunculusBase != selectedType)
                    return false;
            }

            return true;
        }

        #region ICustomTypeDescriptor Implementation
        public PropertyDescriptorCollection GetProperties()
        {
            return _filteredProperties;
        }

        public PropertyDescriptorCollection GetProperties(Attribute[] attributes)
        {
            return _filteredProperties;
        }

        public string GetComponentName() 
        { 
            return TypeDescriptor.GetComponentName(_originalConf, true); 
        }
        
        public TypeConverter GetConverter() 
        { 
            return TypeDescriptor.GetConverter(_originalConf, true); 
        }
        
        public EventDescriptor GetDefaultEvent() 
        { 
            return TypeDescriptor.GetDefaultEvent(_originalConf, true); 
        }
        
        public PropertyDescriptor GetDefaultProperty() 
        { 
            return TypeDescriptor.GetDefaultProperty(_originalConf, true); 
        }
        
        public object GetEditor(Type editorBaseType) 
        { 
            return TypeDescriptor.GetEditor(_originalConf, editorBaseType, true); 
        }
        
        public EventDescriptorCollection GetEvents() 
        { 
            return TypeDescriptor.GetEvents(_originalConf, true); 
        }
        
        public EventDescriptorCollection GetEvents(Attribute[] attributes) 
        { 
            return TypeDescriptor.GetEvents(_originalConf, attributes, true); 
        }
        
        public object GetPropertyOwner(PropertyDescriptor pd) 
        { 
            return _originalConf; 
        }
        
        public AttributeCollection GetAttributes() 
        { 
            return TypeDescriptor.GetAttributes(_originalConf, true); 
        }
        
        public string GetClassName() 
        { 
            return TypeDescriptor.GetClassName(_originalConf, true); 
        }
        #endregion
    }

    internal class WrappedPropertyDescriptor : PropertyDescriptor
    {
        private PropertyDescriptor _baseDescriptor;
        private object _targetObject;

        public WrappedPropertyDescriptor(PropertyDescriptor baseDescriptor, object targetObject)
            : base(baseDescriptor)
        {
            _baseDescriptor = baseDescriptor;
            _targetObject = targetObject;
        }

        public override bool CanResetValue(object component) 
        { 
            return _baseDescriptor.CanResetValue(_targetObject); 
        }
        
        public override Type ComponentType 
        { 
            get { return _baseDescriptor.ComponentType; } 
        }
        
        public override object GetValue(object component) 
        { 
            return _baseDescriptor.GetValue(_targetObject); 
        }
        
        public override bool IsReadOnly 
        { 
            get { return _baseDescriptor.IsReadOnly; } 
        }
        
        public override Type PropertyType 
        { 
            get { return _baseDescriptor.PropertyType; } 
        }
        
        public override void ResetValue(object component) 
        { 
            _baseDescriptor.ResetValue(_targetObject); 
        }
        
        public override void SetValue(object component, object value) 
        { 
            _baseDescriptor.SetValue(_targetObject, value); 
        }
        
        public override bool ShouldSerializeValue(object component) 
        { 
            return _baseDescriptor.ShouldSerializeValue(_targetObject); 
        }
    }

    internal class FilteredHomConfTypeDescriptionProvider : TypeDescriptionProvider
    {
        public override ICustomTypeDescriptor GetTypeDescriptor(Type objectType, object instance)
        {
            return instance as ICustomTypeDescriptor ?? base.GetTypeDescriptor(objectType, instance);
        }
    }
}
