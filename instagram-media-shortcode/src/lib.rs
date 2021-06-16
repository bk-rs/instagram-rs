const PRIVATE_LEN: usize = 28;

/// Ref https://gist.github.com/sclark39/9daf13eea9c0b381667b61e3d2e7bc11
pub fn ig_id_to_shortcode(ig_id: u64) -> String {
    if ig_id == 0 {
        return "".to_owned();
    }

    let mut bytes = ig_id.to_be_bytes().to_vec();
    bytes.insert(0, 0);

    let b64 = base64::encode_config(bytes, base64::STANDARD);

    let s = String::from_utf8_lossy(b64.as_bytes());

    let mut s = s.replacen("+", "-", s.len()).replacen("/", "_", s.len());

    if let Some(offset) = s.find(|c: char| c != 'A') {
        return s.drain(offset..).collect();
    }

    s
}

pub fn shortcode_to_ig_id(shortcode: impl AsRef<str>) -> Result<u64, String> {
    let s = shortcode.as_ref();
    let s = private_shortcode_to_public_shortcode(s);

    if s.is_empty() {
        return Ok(0);
    }
    if s.len() > 11 {
        return Err("invalid".to_owned());
    }

    let mut s = s.replacen("-", "+", s.len()).replacen("_", "/", s.len());

    s = format!(
        "{}{}",
        String::from_utf8(vec![b'A'; 12 - s.len()]).map_err(|err| err.to_string())?,
        s
    );

    let bytes = base64::decode_config(s, base64::STANDARD).map_err(|err| err.to_string())?;

    if bytes.len() != 9 {
        return Err("unreachable".to_owned());
    }
    if bytes[0] != 0 {
        return Err("unreachable".to_owned());
    }

    let mut buf = [0; 8];
    buf.copy_from_slice(&bytes[1..9]);
    Ok(u64::from_be_bytes(buf))
}

pub fn is_private_shortcode(shortcode: &str) -> bool {
    shortcode.len() >= (1 + PRIVATE_LEN) && shortcode.len() <= (11 + PRIVATE_LEN)
}

pub fn private_shortcode_to_public_shortcode(shortcode: &str) -> String {
    if !is_private_shortcode(shortcode) {
        return shortcode.to_owned();
    }

    shortcode[..shortcode.len() - PRIVATE_LEN].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use once_cell::sync::Lazy;

    const DATA: Lazy<Vec<(u64, &str)>> = Lazy::new(|| {
        vec![
            // 4
            (1032176, "D7_w"),
            // 5
            (437530289, "aFC6x"),
            (420278081, "ZDO9B"),
            (420261136, "ZDK0Q"),
            (161130246, "JmqcG"),
            (157121833, "JXX0p"),
            (155561006, "JRawu"),
            (155229606, "JQJ2m"),
            (155224654, "JQIpO"),
            (155218313, "JQHGJ"),
            (150746580, "I_DXU"),
            (559987755, "hYLwr"),
            (528045353, "feVUp"),
            (425469818, "ZXCd6"),
            (373272202, "WP66K"),
            (343935491, "UgAoD"),
            (343154763, "UdCBL"),
            (327029923, "TfhSj"),
            (421437451, "ZHqAL"),
            (88295815, "FQ0mH"),
            (580738842, "inV8a"),
            (522693295, "fJ6qv"),
            (350448475, "U42tb"),
            (346274608, "Uo7sw"),
            (662648618, "nfzcq"),
            (56102025, "DWAyJ"),
            (55183240, "DSgeI"),
            (55183040, "DSgbA"),
            (453992411, "bD1_b"),
            (412285824, "YkvuA"),
            (576766613, "iYMKV"),
            (389533852, "XN9Cc"),
            (389511682, "XN3oC"),
            (389498898, "XN0gS"),
            (364504671, "VueZf"),
            (343580036, "Uep2E"),
            (34661594, "CEOTa"),
            (670573370, "n-CM6"),
            (542511091, "gVg_z"),
            (505175937, "eHF-B"),
            (486620242, "dATxS"),
            (477486396, "cdd08"),
            (448508280, "au7F4"),
            (448419901, "aulg9"),
            (392828132, "XahTk"),
            (388774646, "XLDr2"),
            (375537043, "WYj2T"),
            (186055513, "LFvtZ"),
            (517287514, "e1S5a"),
            (205158695, "MOnkn"),
            (205156153, "MOm85"),
            (158516524, "JcsUs"),
            (158514954, "Jcr8K"),
            (421293888, "ZHG9A"),
            (420629224, "ZEkro"),
            (408610238, "YWuW-"),
            (408610027, "YWuTr"),
            (389795628, "XO88s"),
            (631289058, "loLTi"),
            (631245030, "loAjm"),
            (195974056, "LrlOo"),
            // 10
            (1056959166906215431, "6rEsMliawH"),
            (1030438191905746762, "5M2hN3ia9K"),
            (945953217714958218, "0gs3CMCa-K"),
            (941569566340394384, "0RIIgtCa2Q"),
            (918287674304474896, "y-acZEia8Q"),
            (792631091109932805, "r__ccfia8F"),
            (785608381888572952, "rnCqk3Ca4Y"),
            (737125878412717825, "o6zBS5ia8B"),
            (672707381708828604, "lV78ysia-8"),
            (575962744747569076, "f-OxCUia-0"),
            (998489928793395766, "3bWUzNiRI2"),
            (971466260125127128, "17V2qiCRHY"),
            (945239587474182796, "0eKmWPCRKM"),
            (1113859099227413796, "91OPMDSV0k"),
            (1064554064761151286, "7GDkgaSV82"),
            (1058108616349409145, "6vKC4zSV95"),
            (971676493963418760, "18Fp-DSVyI"),
            (931153845668110088, "zsH306SV8I"),
            (926017417632111845, "zZ3-9hyVzl"),
            (916730896118799512, "y44eSZyVyY"),
            (890765760308797273, "xcor1RSV9Z"),
            (804902569546571820, "srlp9Yn2As"),
            (642780934423724522, "jrndqwuEHq"),
            (573677374600134678, "f2HIhguEAW"),
            (481320595824853627, "at_oghOEJ7"),
            (461030486135292251, "Zl6MjUuEFb"),
            (1150374260838089202, "_280uYAcHy"),
            (1150373370169246163, "_28nw4AcHT"),
            (826217883029987454, "t3UMmBE-B-"),
            (1026764615814849015, "4_zPphlaH3"),
            (1152359739175960764, "_-ARPLC4C8"),
            (1002912076455215874, "3rDzhKEAMC"),
            (938690556103491699, "0G5hZekABz"),
            (911455800488100484, "ymJDiwkAKE"),
            (883387299387146368, "xCbBGDEACA"),
            (874905352922988792, "wkSch1EAD4"),
            (856712187615380443, "vjpzIRkAPb"),
            (837007080481096321, "udpYFKkAKB"),
            (828241399736435134, "t-gSozkAG-"),
            (828113788020458268, "t-DRpIkAMc"),
            (802913203390513491, "skhU3xkAFT"),
            (747096048141336602, "peN-XCkAAa"),
            (735469007897887473, "o06Sp_EALx"),
            (709515365185487604, "nYtHcokAL0"),
            (703795331490382707, "nEYiAIEANz"),
            (686212635604026350, "mF6sGhkAPu"),
            (676101811602456942, "lh_wP4kAFu"),
            (668886213982159619, "lIXHe7kAMD"),
            (628370532316741737, "i4a6t-EABp"),
            (1147843002754376347, "_t9SElF6Kb"),
            (1141234139317903554, "_Wemc3l6DC"),
            (1132617808259555618, "-33eVPl6Ei"),
            (1119600924291474116, "-JnxvWl6LE"),
            (1117426851591463129, "-B5c0Xl6DZ"),
            (1115165929728614906, "953YEzF6H6"),
            (911029360259179143, "ykoGBWMyKH"),
            (885763770712007166, "xK3XTcMyH-"),
            (885006557441630724, "xILMZoMyIE"),
            (874156497510277596, "whoLP4MyHc"),
            (760936528178520561, "qPY72isyHx"),
            // 11
            (2461113606337266768, "CInockRnQRQ"),
            (2449694306716095966, "CH_D_4wHBXe"),
            (2449691244203188128, "CH_DTUkHZ-g"),
            (2441630167741500342, "CHiabOFHAu2"),
            (2428491133486075319, "CGzu9G3n423"),
            (2426198591176285913, "CGrlsOdnZ7Z"),
            (2426181710167934711, "CGrh2kzHAb3"),
            (2426180384877090703, "CGrhjShnpeP"),
            (2421199852795806283, "CGZ1HAkH0JL"),
            (2340880666446425630, "CB8epBJHm4e"),
            (2322199054406563691, "CA6G7yHprdr"),
            (2321460500915047925, "CA3fAalJE31"),
            (2319840786010950658, "CAxuudhpUwC"),
            (2315356651803905682, "CAhzJuHHA6S"),
            (2314705107739747249, "CAffAganCOx"),
            (2309080135403302958, "CALgCYsn_Au"),
            (2308343129375611074, "CAI4diWJZjC"),
            (2304455909480731442, "B_7EnCtJhMy"),
            (2302986725882281126, "B_12jm2pzym"),
            (2302523163745433029, "B_0NJ4-J7XF"),
            (2302320000501077746, "B_ze9edJNby"),
            (2301123769791771529, "B_vO-BypXOJ"),
            (2299300796650797594, "B_oweRwJLoa"),
            (2298603616998951714, "B_mR8-mposi"),
            (2298603475063690985, "B_mR66aplbp"),
            (2293859936987735269, "B_VbXWUp2Tl"),
            (2293858690356666551, "B_VbFNTpuC3"),
            (2467671103476258061, "CI-7ctZMtUN"),
            (2467669113127949992, "CI-6_vvMdao"),
            (2467667287179597563, "CI-6lLMMRb7"),
            (2466107465267174685, "CI5X6x1MF0d"),
            (2466098133402359166, "CI5Vy-2s5V-"),
            (2464053791254878857, "CIyE943M5qJ"),
            (2463221975887161529, "CIvH1YbsKi5"),
            (2463221533690049840, "CIvHu8msDUw"),
            (2463221214319196575, "CIvHqTKs7mf"),
            (2463068413978740082, "CIuk6wwsAVy"),
            (2461123390928552353, "CInqq84s4Gh"),
            (2461102206941679598, "CInl2rwsE_u"),
            (2461099204884249243, "CInlK_4Hzqb"),
            (2461080120441510381, "CIng1SGsRnt"),
            (2461078715475639808, "CIngg1oMz4A"),
            (2450830923490466966, "CIDGb1nMRyW"),
            (2449464344395797338, "CH-PtfpslNa"),
            (2447937019665510932, "CH40b_psPIU"),
            (2466184768853985934, "CI5pfsaBUqO"),
            (2466184429014696586, "CI5pav6BUKK"),
            (2463683419536981352, "CIwwwRUhX1o"),
            (2445881258379995952, "CHxhAuwhmsw"),
            (2444388979173625646, "CHsNtNchg8u"),
            (2443605102897325711, "CHpbeTxhuaP"),
            (2441881379907341121, "CHjTi1pBItB"),
            (2431441284796680439, "CG-NvdYhoj3"),
            (2430643572031422463, "CG7YXNehP__"),
            (2430622790328925114, "CG7TozAhju6"),
            (2430616100187510308, "CG7SHcVBZYk"),
            (2408667696326977954, "CFtToIVhymi"),
            (2405381158151386983, "CFhoWtJB_dn"),
            (2405281636880466661, "CFhRuevB8Ll"),
            (2405213559518090888, "CFhCP0wB1aI"),
        ]
    });

    #[test]
    fn test_ig_id_and_shortcode_converter() -> Result<(), Box<dyn error::Error>> {
        for (ig_id, shortcode) in DATA.iter() {
            assert_eq!(ig_id_to_shortcode(*ig_id), *shortcode);

            assert_eq!(shortcode_to_ig_id(shortcode.to_string())?, *ig_id);
        }

        //
        assert_eq!(ig_id_to_shortcode(u64::MIN), "");
        assert_eq!(shortcode_to_ig_id("")?, u64::MIN);
        assert_eq!(ig_id_to_shortcode(1), "B");
        assert_eq!(shortcode_to_ig_id("B")?, 1);
        assert_eq!(ig_id_to_shortcode(u64::MAX), "P__________");
        assert_eq!(shortcode_to_ig_id("P__________")?, u64::MAX);

        //
        assert_eq!(ig_id_to_shortcode(1724590456043472323), "Bfu-eHrgAXD");
        assert_eq!(
            shortcode_to_ig_id("Bfu-eHrgAXDlJ8ifgkj-lm4H6_UHy5GCAzsBU80")?,
            1724590456043472323
        );

        //
        assert_eq!(ig_id_to_shortcode(2448037011284432259), "CH5LLEGnhWD");
        assert_eq!(
            shortcode_to_ig_id("CH5LLEGnhWDZpMs--h6rwCecLT3So9_ZOwTKCk0")?,
            2448037011284432259
        );

        //
        assert_eq!(
            shortcode_to_ig_id(String::from_utf8(vec![b'X'; 12]).unwrap()).err(),
            Some("invalid".to_owned())
        );
        assert_eq!(
            shortcode_to_ig_id(String::from_utf8(vec![b'X'; 28]).unwrap()).err(),
            Some("invalid".to_owned())
        );

        Ok(())
    }

    #[test]
    fn test_is_private_shortcode() {
        for (_, shortcode) in DATA.iter() {
            assert_eq!(is_private_shortcode(*shortcode), false);
            assert_eq!(
                is_private_shortcode(
                    format!(
                        "{}{}",
                        shortcode,
                        String::from_utf8(vec![b'X'; 28]).unwrap()
                    )
                    .as_str()
                ),
                true
            );
        }

        assert_eq!(
            is_private_shortcode("Bfu-eHrgAXDlJ8ifgkj-lm4H6_UHy5GCAzsBU80"),
            true
        );
    }

    #[test]
    fn test_private_shortcode_to_public_shortcode() {
        for (_, shortcode) in DATA.iter() {
            assert_eq!(
                private_shortcode_to_public_shortcode(*shortcode),
                *shortcode
            );

            assert_eq!(
                private_shortcode_to_public_shortcode(
                    format!(
                        "{}{}",
                        shortcode,
                        String::from_utf8(vec![b'X'; 28]).unwrap()
                    )
                    .as_str()
                ),
                *shortcode
            );
        }

        assert_eq!(
            private_shortcode_to_public_shortcode("Bfu-eHrgAXDlJ8ifgkj-lm4H6_UHy5GCAzsBU80"),
            "Bfu-eHrgAXD"
        );
    }
}
