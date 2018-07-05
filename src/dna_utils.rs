

pub fn rev_comp(i: &mut str) -> String {

    let mut ri = String::with_capacity(i.len());
    for lok in i.chars() {
        ri.push(replace_bases(lok));
    }
    ri.chars().rev().collect::<String>()
}

pub fn replace_bases(x: char) -> char {

    match x {

        'A' => 'T',
        'C' => 'G',
        'T' => 'A',
        'G' => 'C',
        'U' => 'A',
        'a' => 't',
        'c' => 'g',
        't' => 'a',
        'g' => 'c',
        'u' => 'a',
        _  => 'N'
    }
}