tys = "u8 i8 u16 i16 u32 i32 f32 u64 i64 f64 i128 u128"
lens = "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32"
size = {1, 1, 2, 2, 4, 4, 4, 8, 8, 8, 8, 16, 16}
i = 1
for ty in tys:gmatch("%w+") do
	for len in lens:gmatch("%d+") do
		print(
"\nimpl RawSerialize for [" .. ty .. "; " .. len .. "] ".. "{\n "
.. "    fn serialize(&self, to: &mut Write) -> Result<(), Error> {\n"
  .. "        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ " .. ty .. "; " .. len .. "], *const u8>(&(*self) as *const [" .. ty .. "; " .. len .. "]), ".. len .." * " .. size[i] .. ")};\n"
.. "        check!(to.write_all(y));\n"
.. "        Ok(())"
.. "    }\n"
.. "}\nimpl RawDeserialize for [" .. ty .. "; " .. len .. "] {\n"
.. "    fn deserialize(from: &mut Read) -> Result<[" .. ty .. "; " .. len .. "], Error> {\n"
.. "        unsafe { let mut buffer: [" .. ty .. "; " .. len .. "] = [0" .. ty .. "; " .. len .. "];\n"
  .. "        {\n            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ " .. ty .. "; " .. len .. "], *mut u8>((&mut buffer) as *mut [" .. ty .. "; " .. len .. "]), " .. tonumber(len) * size[i] .. ")));\n        }\n"
.. "        Ok(buffer)\n"
.. "        }\n    }\n"
.. "\n}") 
	end
i = i + 1
end
