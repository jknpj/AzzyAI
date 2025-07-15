using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Drawing;
using System.Linq;
using System.Reflection;
using System.Windows.Forms;

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
            homunculusSFilter.Items.AddRange(new string[] { "All", "Eira", "Eleanor", "Dieter", "Bayeri", "Sera" });
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
            homunculusBaseFilter.Items.AddRange(new string[] { "All", "Amistr", "Vanilmirth", "Filir", "Lif" });
            homunculusBaseFilter.SelectedIndex = 0;
            homunculusBaseFilter.SelectedIndexChanged += Filter_Changed;

            // Create tab control
            tabControl = new TabControl
            {
                Location = new Point(10, 40),
                Size = new Size(this.Width - 20, this.Height - 50),
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
                
                tabPage.Controls.Add(propertyGrid);
                tabControl.TabPages.Add(tabPage);
            }

            // Add controls to the user control
            this.Controls.Add(searchLabel);
            this.Controls.Add(searchBox);
            this.Controls.Add(homunculusSLabel);
            this.Controls.Add(homunculusSFilter);
            this.Controls.Add(homunculusBaseLabel);
            this.Controls.Add(homunculusBaseFilter);
            this.Controls.Add(tabControl);
        }

        private void SearchBox_TextChanged(object sender, EventArgs e)
        {
            ApplyFilters();
        }

        private void Filter_Changed(object sender, EventArgs e)
        {
            ApplyFilters();
        }

        private void PropertyGrid_PropertyValueChanged(object s, PropertyValueChangedEventArgs e)
        {
            if (PropertyValueChanged != null)
                PropertyValueChanged(s, e);
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

        public void Refresh()
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

        private bool ShouldShowProperty(string propertyName, string homunculusSType, string homunculusBaseType)
        {
            string propNameLower = propertyName.ToLower();

            // Filter by Homunculus S type
            if (homunculusSType != "All")
            {
                string homunculusSTypeLower = homunculusSType.ToLower();
                
                // If property contains a specific Homunculus S name and it doesn't match the filter, hide it
                string[] homunculusSTypes = { "eira", "eleanor", "dieter", "bayeri", "sera" };
                foreach (string type in homunculusSTypes)
                {
                    if (propNameLower.Contains(type) && type != homunculusSTypeLower)
                        return false;
                }
                
                // If filter is set to a specific type, only show properties that contain that type or are general
                if (homunculusSTypes.Any(type => propNameLower.Contains(type)))
                {
                    return propNameLower.Contains(homunculusSTypeLower);
                }
            }

            // Filter by Homunculus Base type
            if (homunculusBaseType != "All")
            {
                string homunculusBaseTypeLower = homunculusBaseType.ToLower();
                
                // If property contains a specific Homunculus Base name and it doesn't match the filter, hide it
                string[] homunculusBaseTypes = { "amistr", "vanilmirth", "filir", "lif" };
                foreach (string type in homunculusBaseTypes)
                {
                    if (propNameLower.Contains(type) && type != homunculusBaseTypeLower)
                        return false;
                }
                
                // If filter is set to a specific type, only show properties that contain that type or are general
                if (homunculusBaseTypes.Any(type => propNameLower.Contains(type)))
                {
                    return propNameLower.Contains(homunculusBaseTypeLower);
                }
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
