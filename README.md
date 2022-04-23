# cdda-editor
A Parser, Viewer, and Editor for CDDA SAV files

## Updates as of 23-Apr-2022
Significantly limited the scope of this project as it had gotten more than a little out of control 
and frankly I just didn't want to work on it anymore.  Also I stopped playing CDDA for like 2 years 
and just got back into it.

## Current functionality
It can parse a save file from CDDA v0.F-3.  That's easier said than done with the absolutely ridiculous 
mess they call a data structure in that thing.  Seriously how do they get away with such a huge mess 
in a strongly-typed language?  I haven't seen that sort of mixed-type rescursive hash nonsense since 
the last time I wrote something in Perl.

Oh also it panics on the slightest unexpected thing and there are half a dozen fields that I don't
have populated in any of my save files so I have no idea what they look like so hooray for Serde_Json's 
generic 'Value' type.

Oh and finally it's currently only really able to parse save files with very limited mods enabled.  Like, 
Magiclysm, and that's about it.

## Planned v1 features
- A basic CLI
- Output vital data about a given player save like skill levels or mutations
- Function to clear out active enemies and re-save
- Function to modify skill levels
- Function to clean up massive extraneous data like the insanely huge known_traps hash

## Planned v2 features
- Some sort of gui with, I dunno, maybe the egui crate?  That one looks reasonable.
- Functionality to actually add, remove, and modify mutations and spells and the like
- Maybe tackle the clothing inventory?  It's another super ugly mixed hash that also appears recursive.
- Functionality to modify other things.  What other things?  Who knows!

## A desperate cry for help
If you have a save file with any of the fields marked 'need further research' populated then by all means
make a branch and a PR or something because that'd be really helpful.

## Okay bye
That's it.  Try it or don't, I'unno, this is mostly just for fun/insomnia.
