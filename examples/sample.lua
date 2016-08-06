-- include config file
dofile "examples/config.lua"

code = {
    { "var", "extern", "void*", "func_ptr", "" },
    { "var", "", "int", "int_var", "1" },
}

function dump(o)
   if type(o) == 'table' then
      local s = '{ '
      for k,v in pairs(o) do
         if type(k) ~= 'number' then k = '"'..k..'"' end
         s = s .. '['..k..'] = ' .. dump(v) .. ','
      end
      return s .. '} '
   else
      return tostring(o)
   end
end

function get_code()
    return dump(code)
end