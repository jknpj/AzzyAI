using System;
using System.ComponentModel;
using System.Globalization;
using System.IO;
using System.Text.RegularExpressions;

namespace AzzyAIConfig
{
    enum ServerTypeOptions
    {
        Official = 0,
        Private = 1
    }

    enum BossAggroOptions
    {
        Tactics = 0,
        ReactToMVPs = 1,
        ReactToBosses = 2
    }

    partial class HomConf
    {
        private static readonly string[] AdditionalOptionNames = {
            "ServerType", "AutoDetectServerType", "CastTimeRatio", "SphereTrackInterval", "UseProvokeOwner",
            "UseEiraTwisterCutter", "EiraTwisterCutterLevel",
            "UseEiraAbsoluteZephyr", "EiraAbsoluteZephyrLevel",
            "UseEleanorBlazingAndFurious", "EleanorBlazingAndFuriousLevel",
            "UseEleanorTheOneFighterRises", "EleanorTheOneFighterRisesLevel",
            "UseSeraToxinOfMandara", "SeraToxinOfMandaraLevel",
            "UseSeraNeedleStinger", "SeraNeedleStingerLevel",
            "UseBayeriGlanzenSpies", "BayeriGlanzenSpiesLevel",
            "UseBayeriHeiligePferd", "BayeriHeiligePferdLevel",
            "UseBayeriGoldeneTone", "BayeriGoldeneToneLevel",
            "UseEiraLightOfRegene", "EiraLightOfRegeneLevel", "EiraLightOfRegeneOwnerHP",
            "UseEleanorTinderBreaker", "EleanorTinderBreakerLevel",
            "UseEleanorCBC", "EleanorCBCLevel", "UseEleanorEQC", "EleanorEQCLevel",
            "AllowBloodlust", "AllowSBR44",
            "UseMobInfo", "SkillElementMinRate", "PlantSkills", "BossAggro", "TargetPriority", "KiteByRange"
        };

        [Category("Basic Options"), DefaultValue(ServerTypeOptions.Official),
        Description("Official: Gravity servers. Private: rAthena/Hercules servers. This changes actor identification and skill compatibility, not the selected homunculus filter.")]
        public ServerTypeOptions ServerType { get; set; }

        [Category("Basic Options"), DefaultValue(true),
        Description("Automatically switch to private-server actor identification when rAthena/Hercules monster IDs are detected.")]
        public bool AutoDetectServerType { get; set; }

        private double _castTimeRatio;
        [Category("AutoSkill Options"), DefaultValue(0.5),
        Description("Remaining variable cast time after reductions: 0 = no variable cast time, 1 = full variable cast time.")]
        public double CastTimeRatio
        {
            get { return _castTimeRatio; }
            set
            {
                if (double.IsNaN(value) || value < 0 || value > 1)
                    throw new ArgumentOutOfRangeException("value", "CastTimeRatio must be between 0 and 1.");
                _castTimeRatio = value;
            }
        }

        private int _sphereTrackInterval;
        [Category("AutoSkill Options"), DefaultValue(800),
        Description("Advanced Eleanor option: minimum milliseconds between fighting-style attack attempts counted toward the sphere estimate. This is not the skill cooldown or the real sphere count. Use a shorter interval only for a homunculus that actually attacks faster; too short can cause refused skills. Positive integer; default 800 ms. An H_Extra.lua override takes precedence.")]
        public int SphereTrackInterval
        {
            get { return _sphereTrackInterval; }
            set
            {
                if (value < 1)
                    throw new ArgumentOutOfRangeException("value", "SphereTrackInterval must be positive.");
                _sphereTrackInterval = value;
            }
        }

        [Category("Autobuff Options"), DefaultValue(UseProvokeOwnerOptions.Never),
        Description("Use Amistr Castling to protect the owner in the selected AI states.")]
        public UseProvokeOwnerOptions UseProvokeOwner { get; set; }

        [Category("AutoSkill Options"), DefaultValue(false), Description("Enable Eira's Twister Cutter attack.")]
        public bool UseEiraTwisterCutter { get; set; }
        [Category("AutoSkill Options"), DefaultValue(10), Description("Twister Cutter level, 1-10. The AI also checks the learned level.")]
        public int EiraTwisterCutterLevel { get; set; }
        [Category("AutoSkill Options"), DefaultValue(false), Description("Enable Eira's Absolute Zephyr area attack.")]
        public bool UseEiraAbsoluteZephyr { get; set; }
        [Category("AutoSkill Options"), DefaultValue(10), Description("Absolute Zephyr level, 1-10.")]
        public int EiraAbsoluteZephyrLevel { get; set; }

        [Category("AutoSkill Options"), DefaultValue(false), Description("Enable Eleanor's Blazing And Furious attack.")]
        public bool UseEleanorBlazingAndFurious { get; set; }
        [Category("AutoSkill Options"), DefaultValue(10), Description("Blazing And Furious level, 1-10.")]
        public int EleanorBlazingAndFuriousLevel { get; set; }
        [Category("AutoSkill Options"), DefaultValue(false), Description("Enable Eleanor's The One Fighter Rises attack.")]
        public bool UseEleanorTheOneFighterRises { get; set; }
        [Category("AutoSkill Options"), DefaultValue(10), Description("The One Fighter Rises level, 1-10.")]
        public int EleanorTheOneFighterRisesLevel { get; set; }

        [Category("AutoSkill Options"), DefaultValue(false), Description("Enable Sera's Toxin Of Mandara area attack. Targeting follows the selected server type.")]
        public bool UseSeraToxinOfMandara { get; set; }
        [Category("AutoSkill Options"), DefaultValue(10), Description("Toxin Of Mandara level, 1-10.")]
        public int SeraToxinOfMandaraLevel { get; set; }
        [Category("AutoSkill Options"), DefaultValue(false), Description("Enable Sera's Needle Stinger attack.")]
        public bool UseSeraNeedleStinger { get; set; }
        [Category("AutoSkill Options"), DefaultValue(10), Description("Needle Stinger level, 1-10.")]
        public int SeraNeedleStingerLevel { get; set; }

        [Category("AutoSkill Options"), DefaultValue(false), Description("Enable Bayeri's Glanzen Spies attack.")]
        public bool UseBayeriGlanzenSpies { get; set; }
        [Category("AutoSkill Options"), DefaultValue(10), Description("Glanzen Spies level, 1-10.")]
        public int BayeriGlanzenSpiesLevel { get; set; }
        [Category("AutoSkill Options"), DefaultValue(false), Description("Enable Bayeri's Heilige Pferd area attack.")]
        public bool UseBayeriHeiligePferd { get; set; }
        [Category("AutoSkill Options"), DefaultValue(10), Description("Heilige Pferd level, 1-10.")]
        public int BayeriHeiligePferdLevel { get; set; }
        [Category("Autobuff Options"), DefaultValue(UseDefensiveBuffOptions.Never),
        Description("Use Bayeri's Goldene Tone buff in the selected AI states.")]
        public UseDefensiveBuffOptions UseBayeriGoldeneTone { get; set; }
        [Category("Autobuff Options"), DefaultValue(10), Description("Goldene Tone level, 1-10.")]
        public int BayeriGoldeneToneLevel { get; set; }

        [Category("Autobuff Options"), DefaultValue(UseDefensiveBuffOptions.Never),
        Description("Keep Eira's Light of Regenerate active in the selected AI states. If the owner dies while it is active, the Eira sacrifices itself to resurrect them. The skill needs Loyal intimacy and lowers it on every cast.")]
        public UseDefensiveBuffOptions UseEiraLightOfRegene { get; set; }
        [Category("Autobuff Options"), DefaultValue(5), Description("Light of Regenerate level, 1-5. Higher levels last longer and restore more HP.")]
        public int EiraLightOfRegeneLevel { get; set; }
        [Category("Autobuff Options"), DefaultValue(100),
        Description("Only cast Light of Regenerate while the owner's HP percent is at or below this value (0-100). 100 keeps it up all the time.")]
        public int EiraLightOfRegeneOwnerHP { get; set; }

        [Category("AutoSkill Options"), DefaultValue(true), Description("Enable Eleanor's Tinder Breaker in grappling mode.")]
        public bool UseEleanorTinderBreaker { get; set; }
        [Category("AutoSkill Options"), DefaultValue(5), Description("Tinder Breaker level, 1-5.")]
        public int EleanorTinderBreakerLevel { get; set; }
        [Category("AutoSkill Options"), DefaultValue(true), Description("Enable Eleanor's CBC grappling combo follow-up.")]
        public bool UseEleanorCBC { get; set; }
        [Category("AutoSkill Options"), DefaultValue(5), Description("CBC level, 1-5.")]
        public int EleanorCBCLevel { get; set; }
        [Category("AutoSkill Options"), DefaultValue(true), Description("Enable Eleanor's EQC grappling combo follow-up.")]
        public bool UseEleanorEQC { get; set; }
        [Category("AutoSkill Options"), DefaultValue(5), Description("EQC level, 1-5.")]
        public int EleanorEQCLevel { get; set; }

        [Category("Autobuff Options"), DefaultValue(false), Description("Allow the inherited Amistr Blood Lust buff.")]
        public bool AllowBloodlust { get; set; }
        [Category("AutoSkill Options"), DefaultValue(false), Description("Allow inherited Filir SBR44, which consumes intimacy.")]
        public bool AllowSBR44 { get; set; }

        [Category("Basic Options"), DefaultValue(true),
        Description("Use the monster database (MobInfo.lua: element, race, size, boss/MVP and plant of every monster) to pick skills and to treat bosses and plants. Set to false for the old behavior.")]
        public bool UseMobInfo { get; set; }
        [Category("AutoSkill Options"), DefaultValue(50),
        Description("Attack skills are left out against a target whose element would cut their damage below this percent (0-200). Immune targets (0% or less) are always skipped. Example: fire skills do 90% to Water 1 monsters and 0% to Fire 2 monsters.")]
        public int SkillElementMinRate { get; set; }
        [Category("AutoSkill Options"), DefaultValue(false),
        Description("Use skills on plants and mushrooms too. They take 1 damage from everything, so by default skills are saved for other monsters and plants don't count for area skills.")]
        public bool PlantSkills { get; set; }
        [Category("Basic Options"), DefaultValue(BossAggroOptions.ReactToMVPs),
        Description("Bosses the homunculus never starts a fight with, whatever the tactics say: it still defends you and itself when they attack. Tactics = the tactics decide. ReactToMVPs = MVPs. ReactToBosses = MVPs and mini-bosses.")]
        public BossAggroOptions BossAggro { get; set; }
        [Category("Basic Options"), DefaultValue(false),
        Description("Choose the next target by more than distance. Among targets with the same tactics priority: monsters attacking you or a friend first, plants last, targets your skills can hurt (element), the center of the biggest cluster when an area skill is available, then the closest.")]
        public bool TargetPriority { get; set; }
        [Category("Kiting Options"), DefaultValue(false),
        Description("Kite by the monster's real attack range (monster database). Ranged monsters make the homunculus kite earlier and with a longer step, a monster that never chases only matters inside its range, and none is kited when its range reaches KiteBounds (there would be no way out). Melee monsters keep KiteThreshold. Needs UseMobInfo.")]
        public bool KiteByRange { get; set; }

        private void ResetAdditionalOptions()
        {
            var properties = TypeDescriptor.GetProperties(this);
            foreach (string name in AdditionalOptionNames)
                properties[name].SetValue(this, ((DefaultValueAttribute)properties[name].Attributes[typeof(DefaultValueAttribute)]).Value);
        }

        private void LoadAdditionalOptions(string fileName)
        {
            ResetAdditionalOptions();
            string defaultsFile = Path.Combine(Path.GetDirectoryName(Path.GetFullPath(_file)), "Defaults.lua");
            if (File.Exists(defaultsFile))
                ReadAdditionalOptions(File.ReadAllText(defaultsFile));
            ReadAdditionalOptions(File.ReadAllText(fileName));
        }

        private void ReadAdditionalOptions(string text)
        {
            var properties = TypeDescriptor.GetProperties(this);
            foreach (string name in AdditionalOptionNames)
            {
                MatchCollection matches = Regex.Matches(text, @"(?m)^[ \t]*" + name + @"[ \t]*=[ \t]*([A-Za-z_]+|[-+]?(?:\d+(?:\.\d*)?|\.\d+))[ \t]*(?=--|\r?$)");
                if (matches.Count == 0) continue;
                string token = matches[matches.Count - 1].Groups[1].Value;
                var property = properties[name];
                object value;
                if (property.PropertyType == typeof(ServerTypeOptions))
                {
                    if (token == "SERVER_OFFICIAL" || token == "0") value = ServerTypeOptions.Official;
                    else if (token == "SERVER_PRIVATE" || token == "1") value = ServerTypeOptions.Private;
                    else throw new FormatException("Invalid ServerType: " + token);
                }
                else if (property.PropertyType == typeof(bool))
                    value = int.Parse(token, CultureInfo.InvariantCulture) != 0;
                else if (property.PropertyType.IsEnum)
                    value = Enum.ToObject(property.PropertyType, int.Parse(token, CultureInfo.InvariantCulture));
                else
                    value = Convert.ChangeType(token, property.PropertyType, CultureInfo.InvariantCulture);
                property.SetValue(this, value);
            }
        }

        private string SaveAdditionalOptions(string text)
        {
            var properties = TypeDescriptor.GetProperties(this);
            foreach (string name in AdditionalOptionNames)
            {
                var property = properties[name];
                object value = property.GetValue(this);
                if (name.EndsWith("Level", StringComparison.Ordinal))
                {
                    int maximum = name == "EleanorTinderBreakerLevel" || name == "EleanorCBCLevel" ||
                        name == "EleanorEQCLevel" || name == "EiraLightOfRegeneLevel" ? 5 : 10;
                    if ((int)value < 1 || (int)value > maximum)
                        throw new InvalidOperationException(name + " must be between 1 and " + maximum + ".");
                }
                else if (name == "EiraLightOfRegeneOwnerHP" && ((int)value < 0 || (int)value > 100))
                    throw new InvalidOperationException(name + " must be between 0 and 100.");
                else if (name == "SkillElementMinRate" && ((int)value < 0 || (int)value > 200))
                    throw new InvalidOperationException(name + " must be between 0 and 200.");
                string token = property.PropertyType == typeof(ServerTypeOptions)
                    ? ((ServerTypeOptions)value == ServerTypeOptions.Private ? "SERVER_PRIVATE" : "SERVER_OFFICIAL")
                    : property.PropertyType == typeof(double) ? ((double)value).ToString(CultureInfo.InvariantCulture)
                    : Convert.ToInt32(value, CultureInfo.InvariantCulture).ToString(CultureInfo.InvariantCulture);
                string pattern = @"(?m)^([ \t]*" + name + @"[ \t]*=[ \t]*)[^ \t\r\n]+";
                if (Regex.IsMatch(text, pattern))
                    text = Regex.Replace(text, pattern, match => match.Groups[1].Value + token);
                else
                    text += Environment.NewLine + name.PadRight(25) + "= " + token;
            }
            return text;
        }
    }
}
