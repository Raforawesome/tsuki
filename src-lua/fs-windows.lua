local fs = {}

---@class File
---@field name string
fs.File = {}


---@class Path
---@field private relative boolean
---@field private raw string
---@field private stack string[]
---# tsuki.fs.Path
---A class to represent file paths and allow
---ergonomic path traversal.
fs.Path = {}

---@return string[] split_path An array of path segments.
---@param path string The raw path to split up.
function fs.Path.split_path(path)
    return {}
end

---@return Path new_path The newly created path from given string.
---@param from string The string to parse a path from.
---```lua
---local fs = require('fs')
---local Path = fs.Path
---local home_path = Path.new("/home/arch/projects/tsuki")
---```
function fs.Path.new(from)
    local new_path = setmetatable({
        relative = false, -- set class defaults
        raw = from
    }, {
        __index = fs.Path,          -- Inherit methods from Path
        __newindex = function() end -- Disable adding new behaviour
    })
    return new_path
end

---@return boolean exists Whether or not there is a directory or file at this path.
---Example:
---```lua
---local path = Path.new("/path/to/file")
---print(path:exists())
---```
function fs.Path:exists()
    return FS_INTERNAL.raw_path_exists(self.raw)
end

return fs
