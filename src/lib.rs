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

pub fn myers_unbounded(query: &str, text: &str) -> usize {
    let hsize = if text.len() % 64 == 0 {
        text.len() / 64
    } else {
        text.len() / 64 + 1
    };
    let vsize = if query.len() % 64 == 0 {
        query.len() / 64
    } else {
        query.len() / 64 + 1
    };

    let mut peq: Vec<u64> = vec![0; 256 * vsize];
    for (i, p) in query.bytes().enumerate() {
        peq[(i / 64) * 256 + p as usize] |= 1 << (i % 64);
    }

    let m = query.len();
    let mut score = m;

    let mut mhc: Vec<u64> = vec![0; hsize];
    let mut phc: Vec<u64> = vec![u64::max_value(); hsize];
    let last: u64 = 1 << (m % 64 - 1);

    for b in 0..vsize {
        let mut mv: u64 = 0;
        let mut pv: u64 = u64::max_value();
        score = m;

        for (i, t) in text.bytes().enumerate() {
            let mut eq: u64 = peq[b * 256 + t as usize];
            let pb: u64 = (phc[i / 64] >> (i % 64)) & 1;
            let mb: u64 = (mhc[i / 64] >> (i % 64)) & 1;

            eq |= mb;
            let xh: u64 = (((eq & pv).wrapping_add(pv)) ^ pv) | eq;
            let mut ph: u64 = mv | !(xh | pv);
            let mut mh: u64 = pv & xh;

            if ph & last != 0 {
                score += 1;
            }
            if mh & last != 0 {
                score -= 1;
            }

            if ((ph >> 63) ^ pb) > 0 {
                phc[i / 64] ^= (1 as u64) << (i % 64);
            }
            if ((mh >> 63) ^ mb) > 0 {
                mhc[i / 64] ^= (1 as u64) << (i % 64);
            }

            ph = (ph << 1) | pb;
            mh = (mh << 1) | mb;

            pv = mh | !((eq | mv) | ph);
            mv = ph & (eq | mv);
        }
    }
    score
}

pub fn trim(query: &str, text: &str) -> (usize, usize, usize) {
    let query_len = query.len();
    let text_len = text.len();

    let query_bytes: Vec<u8> = query.bytes().collect();
    let text_bytes: Vec<u8> = text.bytes().collect();

    let mut suffix = 0;
    while query_len > 0 && query_bytes[query_len - 1 - suffix] == text_bytes[text_len - 1 - suffix]
    {
        suffix += 1;
    }

    let mut prefix = 0;
    while prefix < query_len && query_bytes[prefix] == text_bytes[prefix] {
        prefix += 1;
    }

    let query_suffix = query_len - suffix;
    let text_suffix = text_len - suffix;

    (prefix, query_suffix, text_suffix)
}

pub fn lower_bound(query: &str, text: &str) -> usize {
    let qh = histogram(query);
    let th = histogram(text);
    let mut h_diff: usize = 0;
    for i in 0..256 {
        let qf = qh[i];
        let tf = th[i];
        let mut diff = 0;
        if qf < tf {
            diff = tf - qf;
        } else if tf < qf {
            diff = qf - tf;
        }
        h_diff += diff;
    }

    let mut l_diff = 0;
    if query.len() < text.len() {
        l_diff = text.len() - query.len();
    } else if text.len() < query.len() {
        l_diff = query.len() - text.len();
    }
    (h_diff + l_diff) / 2
}

fn histogram(string: &str) -> [usize; 256] {
    let mut hist: [usize; 256] = [0; 256];
    for b in string.bytes() {
        hist[b as usize] += 1;
    }
    hist
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

    #[test]
    fn big_ascii() {
        let query =
            String::from("kitten!kitten!kitten!kitten!kitten!kitten!kitten!kitten!kitten!kitten!");
        let text =
            String::from("sittingsittingsittingsittingsittingsittingsittingsittingsittingsitting");
        assert_eq!(myers_unbounded(&query, &text), 30);
    }

    #[test]
    fn trimmed_small_ascii() {
        let query = String::from("prE kitten! post");
        let text = String::from("pre sitting  Post");
        let (prefix, query_suffix, text_suffix) = trim(&query, &text);

        assert_eq!(prefix, 2);
        assert_eq!(query_suffix, 13);
        assert_eq!(text_suffix, 14);

        assert_eq!(
            myers_64(&query[prefix..query_suffix], &text[prefix..text_suffix]),
            6
        );
    }

    #[test]
    fn trimmed_big_ascii() {
        let query = String::from(
            "prE kitten!kitten!kitten!kitten!kitten!kitten!kitten!kitten!kitten!kitten! post",
        );
        let text = String::from(
            "pre sittingsittingsittingsittingsittingsittingsittingsittingsittingsitting  Post",
        );
        let (prefix, query_suffix, text_suffix) = trim(&query, &text);
        assert_eq!(prefix, 2);
        assert_eq!(query_suffix, 76);
        assert_eq!(text_suffix, 77);
        assert_eq!(
            myers_unbounded(&query[prefix..query_suffix], &text[prefix..text_suffix]),
            33
        );
    }

    #[test]
    fn same_histograms_lower_bound() {
        let query = String::from("abcdefghijklmnopqrstuvwxyz");
        let text = String::from("zyxwvutsrqponmlkjihgfedcba");
        assert_eq!(lower_bound(&query, &text), 0);
    }

    #[test]
    fn different_histograms_lower_bound() {
        let query = String::from("abcdefghijklm");
        let text = String::from("nopqrstuvwxyznopqrstuvwxyznopqrstuvwxyz");
        assert_eq!(lower_bound(&query, &text), 39);
    }

    #[test]
    fn random_histograms_lower_bound() {
        let query = String::from("abcdefghijklm");
        let text = String::from("anaocphqerfsgtdhuivjbwkxlym");
        assert_eq!(lower_bound(&query, &text), 14);
    }

    #[test]
    fn trimmed_lower_bound() {
        let query = String::from("abcdefghijklm");
        let text = String::from("anaocphqerfsgtdhuivjbwkxlym");
        let (prefix, query_suffix, text_suffix) = trim(&query, &text);
        assert_eq!(prefix, 1);
        assert_eq!(query_suffix, 12);
        assert_eq!(text_suffix, 26);
        assert_eq!(
            lower_bound(&query[prefix..query_suffix], &text[prefix..text_suffix]),
            14
        );
    }
}
