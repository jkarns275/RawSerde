-- A lua script to generate implementations for RawSerialize and RawDeserialize
-- for any tuple containing serializable values.

n = {"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"}
i = 2
while i < 27 do
  local z = 1
  local list = "<"
  local type = "("
  local serialize = ""
  local deserialize = ""
  local deserialize_return = "Ok(("
  local where_se = "where "
  local where_de = "where "
  while z <= i do
    where_se = where_se .. n[z] .. ": RawSerialize"
    where_de = where_de .. n[z] .. ": RawDeserialize"
    serialize = serialize
    .. "        let size_x;\n"
    .. "        check!(self." .. (z - 1) .. ".raw_serialize(to), size_x);\n"
    .. "        size += size_x;\n"
    deserialize = deserialize
    .. "        let _" .. n[z]:lower() .. ";\n"
    .. "        check!(" .. n[z] .. "::raw_deserialize(from), _" .. n[z]:lower() .. ");\n"
    deserialize_return = deserialize_return
    .. "_" .. n[z]:lower()
    list = list .. n[z]
    type = type .. n[z]
    if z ~= i then
      list = list .. ", "
      type = type .. ", "
      deserialize_return = deserialize_return .. ", "
      where_se = where_se .. ",\n    "
      where_de = where_de .. ",\n    "
    end
    z = z + 1
  end
  local se =
  "impl".. list .. "> RawSerialize for " .. type .. ") " .. where_se .. " {\n"
  .. "    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {\n"
  .. "        let mut size = 0;\n"
  .. serialize
  .. "        Ok(size)\n"
  .. "    }\n"
  .. "}\n";
  local de =
  "impl" .. list .. "> RawDeserialize for " .. type .. ") " .. where_de .. " {\n"
  .. "    fn raw_deserialize(from: &mut Read) -> Result<" .. type .. "), Error> {\n"
  .. deserialize
  .. "        " .. deserialize_return .. "))\n"
  .. "    }\n"
  .. "}\n";

  print(se)

  print(de)

  i = i + 1
end
