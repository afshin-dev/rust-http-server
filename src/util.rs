pub fn string_slc<'a>(s: &'a str ,start: usize, end: usize) -> &'a str {
    // function always return str 
    // empty slice of str must be considered 

    if start > s.len()  || end > s.len() {
        return "" ;
    }

    if start >= end {
        return "" ;
    }

    &s[start..end]

}