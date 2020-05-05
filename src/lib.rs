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
}
