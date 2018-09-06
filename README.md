# rsearch
Port of [vsearch](https://github.com/torognes/vsearch) into Rust

The licensing applies from vsearch, since it is mostly a port. I intend no copyright or other infringment. All credit to Torbjørn Rognes (torognes), also took some code from rust::bio. Same applies, all credit to Johannes. And of course, [usearch](https://www.drive5.com/usearch/).

Rewriting vsearch with protein comparison and FOGSAA. I have a few ideas how to use this for annotation and other projects. Such as "overhang" extension. I need to a tool to try to fill in gaps in assemblies from PacBio to Illumina. My Illumina reads always drop coverage in HiSeq and I would like to "fill-in" these gaps by returning partial alignments and long overhangs. Many tools already do something similar, but not quite the way I wish to use it though. Still getting to the point of the original intent. Now, I am also thinking about making several smaller, more modular tools, as part of package perhaps. 

## Comments

RSEARCH is nearing VSEARCH speed. It is 30% faster for one core, but overall 50% slower because I only use 1 core. This is also for simple tasks such as msking or filtering reads, not alignment. 

Candidate tools:

rfilter &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; fast filtering <br>
rsearch &nbsp;&nbsp;&nbsp; for sequnce matches <br>
rsort   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; sort by length and abundance <br>
rsample &nbsp;&nbsp;&nbsp;    subsample and shuffle reads <br>
ralign  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;     alignment and splicing tools <br>

## FILTER

Added capability to change softmask to hardmask (mask::fast_mask). Now have max/minlen filter (untested), max/minqual filter (untested), any_n filter (tested, filters sequences with any 'N' in seq), count_n (untested, LESS than provided, not equal to), and DUST preserving 80 char lines, and without (tested). DUST has soft- and hardmasking. 

    vsearch --fastx_mask /Users/daniel/Desktop/gg.fasta --fastaout  --qmask dust   10.55s user 0.13s system 291% cpu 3.660 total
    ./rsearch /Users/daniel/Desktop/gg.fasta  7.23s user 0.14s system 99% cpu 7.434 total

This in fact means vsearch was probably using more cores and was slower. Let's see with more thorough testing. 

## ALIGNMENT

Next Phase is back to alignment:

Smith-Waterman &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Not Started <br>
FOGSAA       &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Not Started <br>
Needlemann-Wunsch  &nbsp;&nbsp;     In progress <br>



## CLUSTERING

This seems to be where vsearch and usearch shine. Not really going to focus on this for now. 


## DEREPLICATION

Now this could be something for rsearch to implement, rather simple, just look at "size=" tag on fasta header, then derep and eventually rerep. Saves space and time. 



2 issues, vsearch --fastx_mask puts everything to uppercase (softmasking goes away). Maybe this is how it should be, but I don't think so. The other issue is, vsearch removes everything after " " in the fasta header. Again, maybe this is for a reason but I prefer to keep the file as is. Other than that, I am fairly sure I give more or less, the same output. 
