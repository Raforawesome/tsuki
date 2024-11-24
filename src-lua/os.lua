local sys = {}

---@return string os_name The name of the current OS.
---Possible return values:
---- linux
---- windows
---- macos
---- android
---- ios
---
--- and every other value listed [here](https://doc.rust-lang.org/stable/std/env/consts/constant.OS.html),
--- although it is very unlikely these will ever be used.
function sys.os_name()
    return OS_INTERNAL.GET_OS_NAME()
end

---@return string arch_name The name of the current CPU architecture.
---Possible return values:
---- x86
---- x86_64
---- arm
---- aarch64
---
--- and every other value listed [here](https://doc.rust-lang.org/stable/std/env/consts/constant.ARCH.html),
--- although it is very unlikely these will ever be used.
function sys.arch_name()
    return OS_INTERNAL.GET_ARCH_NAME()
end

---@return string os_name The name of the current OS family.
---Possible return values:
---- unix
---- windows
---- wasm
function sys.os_family_name()
    return OS_INTERNAL.GET_OS_FAMILY_NAME()
end

return sys
