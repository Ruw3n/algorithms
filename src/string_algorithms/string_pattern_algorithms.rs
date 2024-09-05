pub fn kmp_algorithm(text: &str, pattern: &str) ->i32 {
    let prefix = calculate_prefix_func(pattern);
    let mut s = 0;
    let mut  v = 0;
    let p:Vec<char> = pattern.chars().collect() ;
    let t:Vec<char> = text.chars().collect();
    while v< text.len() {
        if p[v-s] == t[v] {
            v+=1;
            if v-s == pattern.len(){return s as i32;}
        } else {
            if s!=v {
                s = v-prefix[v-s];
            } else {
                s+=1;
                v+=1;
            }
        }
    }
-1
}
fn calculate_prefix_func(pattern: &str) -> Vec<usize> {
    let m = pattern.len();
    let p: Vec<char> = pattern.chars().collect();
    let mut prefix: Vec<usize> = vec![];
    let mut i =0;
    while i<m {
        prefix.push(0);
        i+=1;
    }
    for i in 1..m {
        let mut k = prefix[i - 1];
        while p[k + i] != p[i] && k > 0 {
            k = prefix[k];
        }
        if p[k + 1] != p[i] {
            prefix[i] = 0;
        } else {
            prefix[i] = k + 1
        }
    }
    prefix
}
