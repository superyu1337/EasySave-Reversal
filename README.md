# Easy Save Reversal
Easy Save is a framework to save and load data in unity games.
Im analysing this using save files from the game "She will punish them".
The general structure seems to be in small blocks, im going to refer to them as "Datablocks" from now on.  

## Analysis
### Datablock
Example Block: `7E 0A 70 6C 61 79 65 72 4E 61 6D 65 0B 00 00 00 FF EE F1 E9 FD 04 45 6C 6C 61 7B`
- `0x7E`: A string is following.
- `0x0A`: Length of the string that follows.
- 3rd to 10th byte: Actual string. In this case: `playername` - No null terminator!
- `0x0B`: Some sort of Checksum, indicator or terminator.
- 3 zero filled bytes
- 5 datatype indicator bytes?
- Actual data, seems to be variable length depending on the value's datatype.
- `0x7B`: Datablock terminator
   
 
Bool:
   - 5 indicator bytes: `FF 9C 7C 4D AD`


Block for "Gold" in SWPT: `7E 04 67 6F 6C 64 0A 00 00 00 FF 56 08 A8 E2 F4 01 00 00 7B`  
`7E`: Block start indicator
`04`: String length  
`67 6F 6C 64`: String "gold"  
`0A`: Some form of indicator or checksum, could also be a string terminator.  
`00 00 00`: 3 zero filled bytes, some padding for whatever reason.  
`FF 56 08 A8 E2`: 5 indicator bytes for the value datatype, no idea why those are 5. This one indicates that a 32 bit unsigned integer is following.  
`F4 01 00 00`: Data, this is 500. Encoding is LITTLE ENDIAN!  
`7B`: Block terminator  

## List of indicator bytes
`0x7E`: Start of a datablock.
`07B`: End of a datablock:
`FF EE F1 E9 FD`: String type, followed by 1 byte for the string length.
`FF 56 08 A8 E2`: u32 type.
`FF 9C 7C 4D AD`: 1 byte long bool.
