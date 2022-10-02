includes("**/xmake.lua")

function string.contains(src, sub)
  return string.match(src, sub) ~= nil
end

function getpathname(path)
  return string.match(path, ".+[\\/](.+)")
end

function getname(file)
  return string.match(file, ".*[\\/](.+)..+")
end

add_includedirs("$(projectdir)/src/include")

for _, dir in ipairs(os.dirs(os.projectdir() .. "/src/*")) do
  local name = getpathname(dir)
  if name ~= "include" then
    local sourcefiles = os.files(os.projectdir() .. "/src/" .. name .. "/*.c")
    if #sourcefiles > 0 then
      target(name)
        set_kind("binary")
        add_files("$(projectdir)/src/" .. name .. "/*.c")
        add_includedirs("$(projectdir)/src/" .. name)
      target_end()
      for _, testpath in ipairs(os.files(os.projectdir() .. "/src/" .. name .. "/tests/*")) do
        local test = getname(testpath)
        target(name .. "-" .. test)
          set_kind("binary")
          add_files(testpath)
          add_includedirs("$(projectdir)/src/" .. name)
      end
    end
  end
end