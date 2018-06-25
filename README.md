# rsearch
Port of vsearch into Rust

Rewriting vsearch with protein comparison and FOGSAA. I have a few ideas how to use this for annotation and other projects. Such as "overhang" extension. I need to a tool to try to fill in gaps in assemblies from PacBio to Illumina. My Illumina reads always drop coverage in HiSeq and I would like to "fill-in" these gaps by returning partial alignments and long overhangs. Many tools already do something similar, but not quite the way I wish to use it though. 

Right now, most of my thought is how to best read and write fastas. Rust Bio is ok, but the interface is wonky. Don't know how to erase records either (for filter). 

DUST is now working WindowMasker is almost done. TRF maybe, and something specific for Fungi (OcculterCut?). 

Of course, the licensing applies from vsearch. I intend no copyright or other infringment. All credit to Torbjørn Rognes (torognes). 


