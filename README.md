# rsearch
Port of vsearch into Rust

Rewriting vsearch with protein comparison and FOGSAA. I have a few ideas how to use this for annotation and other projects. Such as "overhang" extension. I need to a tool to try to fill in gaps in assemblies from PacBio to Illumina. My Illumina reads always drop coverage in HiSeq and I would like to "fill-in" these gaps by returning partial alignments and long overhangs. Many tools already do something similar, but not quite the way I wish to use it though. Still getting to the point of the original intent. Now I am alo thinking about making several smaller tools. 

Candidate tools:

rfilter      fast filtering
rsearch      for sequnce matches
rsort        sort by length and bundance
rsample      subsample and shuffle reads
ralign       alignment and splicing tools 

Don't need to erase records, just skip when writing them according to a predicate. Also added a mthod for returning the string, not just as_bytes. Will also be moving DUST and WM to methods as well. 

Filter of Quality scores is what I need to focus on now. Should be simple if I can use 12u8 or similar (which it looks like I can).

TRF maybe, and something specific for Fungi (OcculterCut?). 

Of course, the licensing applies from vsearch. I intend no copyright or other infringment. All credit to Torbjørn Rognes (torognes), also took some code from rust::bio. Same applies, all credit to Johannes. 

Currently, I am ~10x slower than vsearch in both filtering and masking. SIMD will help I think and some optimization could probably get me in the same ballpark. 

2 issues, vsearch --fastx_mask puts everything to uppercase (softmasking goes away). Maybe this is how it should be, but I don't think so. The other issue is, vsearch removes everything after " " in the fasta header. Again, maybe this is for a reason but I prefer to keep the file as is. Other than that, I am fairly sure I give more or less, the same output. 
