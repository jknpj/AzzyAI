# azzy-config

Format-preserving reader/writer for `H_Config.lua`, `M_Config.lua` and `Types/<Name>.lua`.

**Why:** a native GUI (or any tool) that rewrites a player's configuration must not reformat,
reorder or lose anything it did not change. This is the Rust equivalent of
`webgui/luaConfig.js`: parse into lines (blank, comment, `Name = value [-- comment]`, other), edit a
value in place, write everything else back byte for byte - including spacing, trailing comments,
tab alignment and the file's line endings.

```
azzy-config list USER_AI/H_Config.lua          # name=value per line
azzy-config get USER_AI/H_Config.lua LavaSlideMode
azzy-config sections USER_AI/H_Config.lua      # comment section -> options
azzy-config diff a.lua b.lua                   # what differs between two files
azzy-config set H_Config.lua LavaSlideMode=1   # in place, touching only that line
```

`cargo test -p azzy-config` checks the properties that matter:

- **round-trip byte-identical** for the shipped `H_Config.lua`, `M_Config.lua` and
  `Types/Amistr.lua` (including the file that has no trailing newline);
- `set` changes only the value token, keeping the trailing comment;
- an unknown option is not appended (a typo must not invent a key);
- section grouping follows the `-- section` comments;
- a quoted value with dashes (`"Default Configuration LATAM"`) is not mistaken for a comment.
