# Easy Save Reversal
Easy Save is a framework to save and load data in unity games.  
Im analysing this using save files from the game "She will punish them".  
The general structure seems to be in small blocks, im going to refer to them as "Datablocks" from now on.  

## Analysis
### Datablock structure on an example
Block for "Gold" in SWPT: `7E 04 67 6F 6C 64 0A 00 00 00 FF 56 08 A8 E2 F4 01 00 00 7B`  
`7E`: Block start indicator  
`04`: String length  
`67 6F 6C 64`: String "gold"  
`0A`: newline ('\n')  
`00 00 00`: 3 zero filled bytes, some padding for whatever reason.  
`FF 56 08 A8 E2`: 5 indicator bytes for the value datatype, no idea why those are 5. This one indicates that a 32 bit unsigned integer is following.  
`F4 01 00 00`: Data, this is 500. Encoding is LITTLE ENDIAN!  
`7B`: Block terminator  

## List of indicator bytes
`7E`: Start of a datablock.  
`7B`: End of a datablock.  
`FF EE F1 E9 FD`: String type, followed by 1 byte for the string length.  
`FF 56 08 A8 E2`: u32 type.  
`FF 6B D7 3E 6E`: f32 type.  
`FF 9C 7C 4D AD`: 1 byte long bool.  
`FF 46 DC 66 EC`: Vector3 datatype, three f32 values back to back.  
`53`: Indicates an Array, that byte is infront of the 5 type indicator bytes, then followed by 1 padding byte and a u32 for array length.  
