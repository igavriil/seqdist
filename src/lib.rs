pub fn myers_64(query: &str, text: &str) -> usize {
    let mut peq: [u64; 256] = [0; 256];
    for (i, p) in query.bytes().enumerate() {
        peq[p as usize] |= 1 << i;
    }

    let m = query.len();
    let mut score = m;

    let mut mv = 0;
    let mut pv = u64::max_value();
    let last: u64 = 1 << (m - 1);

    for t in text.bytes() {
        let eq: u64 = peq[t as usize];
        let xv = eq | mv;
        let xh = (((eq & pv).wrapping_add(pv)) ^ pv) | eq;

        let mut ph = mv | !(xh | pv);
        let mut mh = pv & xh;

        if ph & last != 0 {
            score += 1;
        }
        if mh & last != 0 {
            score -= 1;
        }

        ph = ph.wrapping_shl(1) | 1;
        mh = mh.wrapping_shl(1);
        pv = mh | !(xv | ph);
        mv = ph & xv;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_ascii() {
        let query = String::from("kitten!");
        let text = String::from("sitting");
        assert_eq!(myers_64(&query, &text), 3);
    }
}
