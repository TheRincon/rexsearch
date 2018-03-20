mod abundance;
mod dust;

fn main() {

    // let x = abundance::get_abundance("sdfsdf;gdfgdf;size=5734895707834588");
    // println!("{:?}", x);
    let mut tt: [char; 1020] = ['C', 'C', 'A', 'C', 'T', 'G', 'C', 'A', 'C', 'T', 'C', 'A', 'C', 'C', 'G', 'C', 'A', 'C', 'C', 'C', 'G', 'G', 'C', 'C', 'A', 'A', 'T', 'T', 'T', 'T', 'T', 'G', 'T', 'G', 'T', 'T', 'T', 'T', 'T', 'A', 'G', 'T', 'A', 'G', 'A', 'G', 'A', 'C', 'T', 'A', 'A', 'A', 'T', 'A', 'C', 'C', 'A', 'T', 'A', 'T', 'A', 'G', 'T', 'G', 'A', 'A', 'C', 'A', 'C', 'C', 'T', 'A', 'A', 'G', 'A', 'C', 'G', 'G', 'G', 'G', 'G', 'G', 'C', 'C', 'T', 'T', 'G', 'G', 'A', 'T', 'C', 'C', 'A', 'G', 'G', 'G', 'C', 'G', 'A', 'T', 'T', 'C', 'A', 'G', 'A', 'G', 'G', 'G', 'C', 'C', 'C', 'C', 'G', 'G', 'T', 'C', 'G', 'G', 'A', 'G', 'C', 'T', 'G', 'T', 'C', 'G', 'G', 'A', 'G', 'A', 'T', 'T', 'G', 'A', 'G', 'C', 'G', 'C', 'G', 'C', 'G', 'C', 'G', 'G', 'T', 'C', 'C', 'C', 'G', 'G', 'G', 'A', 'T', 'C', 'T', 'C', 'C', 'G', 'A', 'C', 'G', 'A', 'G', 'G', 'C', 'C', 'C', 'T', 'G', 'G', 'A', 'C', 'C', 'C', 'C', 'C', 'G', 'G', 'G', 'C', 'G', 'G', 'C', 'G', 'A', 'A', 'G', 'C', 'T', 'G', 'C', 'G', 'G', 'C', 'G', 'C', 'G', 'G', 'C', 'G', 'C', 'C', 'C', 'C', 'C', 'T', 'G', 'G', 'A', 'G', 'G', 'C', 'C', 'G', 'C', 'G', 'G', 'G', 'A', 'C', 'C', 'C', 'C', 'T', 'G', 'G', 'C', 'C', 'G', 'G', 'T', 'C', 'C', 'G', 'C', 'G', 'C', 'A', 'G', 'G', 'C', 'G', 'C', 'A', 'G', 'C', 'G', 'G', 'G', 'G', 'T', 'C', 'G', 'C', 'A', 'G', 'G', 'G', 'C', 'G', 'C', 'G', 'G', 'C', 'G', 'G', 'G', 'T', 'T', 'C', 'C', 'A', 'G', 'C', 'G', 'C', 'G', 'G', 'G', 'G', 'A', 'T', 'G', 'G', 'C', 'G', 'C', 'T', 'G', 'T', 'C', 'C', 'G', 'C', 'G', 'G', 'A', 'G', 'G', 'A', 'C', 'C', 'G', 'G', 'G', 'C', 'G', 'C', 'T', 'G', 'G', 'T', 'G', 'C', 'G', 'C', 'G', 'C', 'C', 'C', 'T', 'G', 'T', 'G', 'G', 'A', 'A', 'G', 'A', 'A', 'G', 'C', 'T', 'G', 'G', 'G', 'C', 'A', 'G', 'C', 'A', 'A', 'C', 'G', 'T', 'C', 'G', 'G', 'C', 'G', 'T', 'C', 'T', 'A', 'C', 'A', 'C', 'G', 'A', 'C', 'A', 'G', 'A', 'G', 'G', 'C', 'C', 'C', 'T', 'G', 'G', 'A', 'A', 'A', 'G', 'G', 'T', 'G', 'C', 'G', 'G', 'C', 'A', 'G', 'G', 'C', 'T', 'G', 'G', 'G', 'C', 'G', 'C', 'C', 'C', 'C', 'C', 'G', 'C', 'C', 'C', 'C', 'C', 'A', 'G', 'G', 'G', 'G', 'C', 'C', 'C', 'T', 'C', 'C', 'C', 'T', 'C', 'C', 'C', 'C', 'A', 'A', 'G', 'C', 'C', 'C', 'C', 'C', 'C', 'G', 'G', 'A', 'C', 'G', 'C', 'G', 'C', 'C', 'T', 'C', 'A', 'C', 'C', 'C', 'A', 'C', 'G', 'T', 'T', 'C', 'C', 'T', 'C', 'T', 'C', 'G', 'C', 'A', 'G', 'G', 'A', 'C', 'C', 'T', 'T', 'C', 'C', 'T', 'G', 'G', 'C', 'T', 'T', 'T', 'C', 'C', 'C', 'C', 'G', 'C', 'C', 'A', 'C', 'G', 'A', 'A', 'G', 'A', 'C', 'C', 'T', 'A', 'C', 'T', 'T', 'C', 'T', 'C', 'C', 'C', 'A', 'C', 'C', 'T', 'G', 'G', 'A', 'C', 'C', 'T', 'G', 'A', 'G', 'C', 'C', 'C', 'C', 'G', 'G', 'C', 'T', 'C', 'C', 'T', 'C', 'A', 'C', 'A', 'A', 'G', 'T', 'C', 'A', 'G', 'A', 'G', 'C', 'C', 'C', 'A', 'C', 'G', 'G', 'C', 'C', 'A', 'G', 'A', 'A', 'G', 'G', 'T', 'G', 'G', 'C', 'G', 'G', 'A', 'C', 'G', 'C', 'G', 'C', 'T', 'G', 'A', 'G', 'C', 'C', 'T', 'C', 'G', 'C', 'C', 'G', 'T', 'G', 'G', 'A', 'G', 'C', 'G', 'C', 'C', 'T', 'G', 'G', 'A', 'C', 'G', 'A', 'C', 'C', 'T', 'A', 'C', 'C', 'C', 'C', 'A', 'C', 'G', 'C', 'G', 'C', 'T', 'G', 'T', 'C', 'C', 'G', 'C', 'G', 'C', 'T', 'G', 'A', 'G', 'C', 'C', 'A', 'C', 'C', 'T', 'G', 'C', 'A', 'C', 'G', 'C', 'G', 'T', 'G', 'C', 'C', 'A', 'G', 'C', 'T', 'G', 'C', 'G', 'A', 'G', 'T', 'G', 'G', 'A', 'C', 'C', 'C', 'G', 'G', 'C', 'C', 'A', 'G', 'C', 'T', 'T', 'C', 'C', 'A', 'G', 'G', 'T', 'G', 'A', 'G', 'C', 'G', 'G', 'C', 'T', 'G', 'C', 'C', 'G', 'T', 'G', 'C', 'T', 'G', 'G', 'G', 'C', 'C', 'C', 'C', 'T', 'G', 'T', 'C', 'C', 'C', 'C', 'G', 'G', 'G', 'A', 'G', 'G', 'G', 'C', 'C', 'C', 'C', 'G', 'G', 'C', 'G', 'G', 'G', 'G', 'T', 'G', 'G', 'G', 'T', 'G', 'C', 'G', 'G', 'G', 'G', 'G', 'G', 'C', 'G', 'T', 'G', 'C', 'G', 'G', 'G', 'G', 'C', 'G', 'G', 'G', 'T', 'G', 'C', 'A', 'G', 'G', 'C', 'G', 'A', 'G', 'T', 'G', 'A', 'G', 'C', 'C', 'T', 'T', 'G', 'A', 'G', 'C', 'G', 'C', 'T', 'C', 'G', 'C', 'C', 'G', 'C', 'A', 'G', 'C', 'T', 'C', 'C', 'T', 'G', 'G', 'G', 'C', 'C', 'A', 'C', 'T', 'G', 'C', 'C', 'T', 'G', 'C', 'T', 'G', 'G', 'T', 'A', 'A', 'C', 'C', 'C', 'T', 'C', 'G', 'C', 'C', 'C', 'G', 'G', 'C', 'A', 'C', 'T', 'A', 'C', 'C', 'C', 'C', 'G', 'G', 'A', 'G', 'A', 'C', 'T', 'T', 'C', 'A', 'G', 'C', 'C', 'C', 'C', 'G', 'C', 'G', 'C', 'T', 'G', 'C', 'A', 'G', 'G', 'C', 'G', 'T', 'C', 'G', 'C', 'T', 'G', 'G', 'A', 'C', 'A', 'A', 'G', 'T', 'T', 'C', 'C', 'T', 'G', 'A', 'G', 'C', 'C', 'A', 'C', 'G', 'T', 'T', 'A', 'T', 'C', 'T', 'C', 'G', 'G', 'C', 'G', 'C', 'T', 'G', 'G', 'T', 'T', 'T', 'C', 'C', 'G', 'A', 'G', 'T', 'A', 'C', 'C', 'G', 'C', 'T', 'G', 'A', 'A', 'C', 'T', 'G', 'T', 'G', 'G', 'G', 'T', 'G', 'G', 'G', 'T', 'G', 'G', 'C', 'C', 'G', 'C', 'G', 'G', 'G', 'A', 'T', 'C', 'C', 'C', 'C', 'A', 'G', 'G', 'C', 'G', 'A', 'C', 'C', 'T', 'T', 'C', 'C', 'C', 'C', 'G', 'T', 'G', 'T', 'T', 'T', 'G', 'A', 'G', 'T', 'A', 'A', 'A', 'G', 'C', 'C', 'T', 'C', 'T', 'C', 'C', 'C', 'A', 'G', 'G', 'A', 'G', 'C', 'A', 'G', 'C', 'C', 'T', 'T', 'C', 'T', 'T', 'G', 'C', 'C', 'G', 'T', 'G', 'C', 'T', 'C', 'T', 'C', 'T', 'C', 'G', 'A', 'G', 'G', 'T', 'C', 'A', 'G', 'G', 'A', 'C', 'G', 'C', 'G', 'A', 'G', 'A', 'G', 'G', 'A', 'A', 'G', 'G', 'C', 'G', 'C'];
    /* let mut test_string = String::from("CCACTGCACTCACCGCACCCGGCCAATTTTTGTGTTTTTAGTAGAGACTAAATACCATATAGTGAACACCTAAGA
CGGGGGGCCTTGGATCCAGGGCGATTCAGAGGGCCCCGGTCGGAGCTGTCGGAGATTGAGCGCGCGCGGTCCCGG
GATCTCCGACGAGGCCCTGGACCCCCGGGCGGCGAAGCTGCGGCGCGGCGCCCCCTGGAGGCCGCGGGACCCCTG
GCCGGTCCGCGCAGGCGCAGCGGGGTCGCAGGGCGCGGCGGGTTCCAGCGCGGGGATGGCGCTGTCCGCGGAGGA
CCGGGCGCTGGTGCGCGCCCTGTGGAAGAAGCTGGGCAGCAACGTCGGCGTCTACACGACAGAGGCCCTGGAAAG
GTGCGGCAGGCTGGGCGCCCCCGCCCCCAGGGGCCCTCCCTCCCCAAGCCCCCCGGACGCGCCTCACCCACGTTC
CTCTCGCAGGACCTTCCTGGCTTTCCCCGCCACGAAGACCTACTTCTCCCACCTGGACCTGAGCCCCGGCTCCTC
ACAAGTCAGAGCCCACGGCCAGAAGGTGGCGGACGCGCTGAGCCTCGCCGTGGAGCGCCTGGACGACCTACCCCA
CGCGCTGTCCGCGCTGAGCCACCTGCACGCGTGCCAGCTGCGAGTGGACCCGGCCAGCTTCCAGGTGAGCGGCTG
CCGTGCTGGGCCCCTGTCCCCGGGAGGGCCCCGGCGGGGTGGGTGCGGGGGGCGTGCGGGGCGGGTGCAGGCGAG
TGAGCCTTGAGCGCTCGCCGCAGCTCCTGGGCCACTGCCTGCTGGTAACCCTCGCCCGGCACTACCCCGGAGACT
TCAGCCCCGCGCTGCAGGCGTCGCTGGACAAGTTCCTGAGCCACGTTATCTCGGCGCTGGTTTCCGAGTACCGCT
GAACTGTGGGTGGGTGGCCGCGGGATCCCCAGGCGACCTTCCCCGTGTTTGAGTAAAGCCTCTCCCAGGAGCAGC
CTTCTTGCCGTGCTCTCTCGAGGTCAGGACGCGAGAGGAAGGCGC").replace("\n", ""); */
    // println!("teststring len is {:?}", test_string.chars().count());
    let y = dust::dust(&mut tt, 1020, true);
    // let gog = y.iter().cloned().collect::<String>();
    let bob = tt.iter().cloned().collect::<String>();
    println!("{:?}", bob);
}
