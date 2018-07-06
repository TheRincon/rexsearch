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

Currently, slightly faster(!) in DUST masking. The reason for the last post was I forgot to add "--release" to my spotcap intellij profile. Now I am 30% faster than vsearch with much lower cpu usage. Will test more thoroughly with rust benchmarks.

    vsearch --fastx_mask /Users/daniel/Desktop/gg.fasta --fastaout  --qmask dust   10.55s user 0.13s system 291% cpu 3.660 total
    ./rsearch /Users/daniel/Desktop/gg.fasta  7.23s user 0.14s system 99% cpu 7.434 total

This in fact means vsearch was probably using more cores and was slower. Let's see with more thorough testing. 

2 issues, vsearch --fastx_mask puts everything to uppercase (softmasking goes away). Maybe this is how it should be, but I don't think so. The other issue is, vsearch removes everything after " " in the fasta header. Again, maybe this is for a reason but I prefer to keep the file as is. Other than that, I am fairly sure I give more or less, the same output. 
