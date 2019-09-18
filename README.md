# Indrex
Indrex is a little tool I wrote to explore REXPaint files. It requires a 
REXPaint image (.xp) and a font image which was used to create the xp file.


It displays the xp file and the font file, as well as the character under the
cursor, and provides information about that character such as the ascii character,
the position in the font file, and the character's hex representation.


The tool will also highlight uses of a character in the xp file, to help
understand the makeup of the file, and it will highlight the character in
the font map to identify which character is in which square.


Overall this is a fairly specialized tool, but it could possibly be useful for
others. It hard-codes path and file name information, and probably
has assumptions about the particular files I use it on, so user beware.
