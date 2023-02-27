fn main() {
    let vec1 = vec!['a','b','c','d','e'];
    let vec2 =  vec!['x','x','c','x','x'];
    let allignment : Vec<usize> = vec![2];

    
    let(parts1, parts2) = vec1.into_iter().zip(vec2).enumerate().fold(
        (Vec::new(),Vec::new()), |(mut parts1,mut parts2),(idx, parts)| {
        if idx == 0 || allignment.contains(&idx) {
            parts1.push(Vec::new());
            parts2.push(Vec::new());
        } 
        if  !allignment.contains(&idx) {
            parts1.last_mut().unwrap().push(parts.0);
            parts2.last_mut().unwrap().push(parts.1);

        }
        (parts1,parts2)

    });
    println!("{:?}{:?}",parts1,parts2);
}
