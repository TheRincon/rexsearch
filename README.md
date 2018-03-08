# rsearch
Port of vsearch into Rust

Rewriting vsearch with protein and FOGSAA. I have a few ideas how to use this for annotation and other projects. Such as "overhang" branching. I need to a tool to try to fill in gaps in assemblies from PacBio to Illumina. My Illumina reads always drop coverage in HiSeq and I would like to "fill-in" these gaps by returning partial alignments and overhangs. Many tools already do this, but not quite the way I wish to use it though. 
Right now it has Needleman-Wunsch, but I am looking into FOGSAA. Eventually want to add Smith-Waterman like SWIPE, and SIMD to things I can optimize. 

Of course, the licensing applies from vsearch. I intend no copyright or other infringment. All credit to Torbjørn Rognes (torognes). 
