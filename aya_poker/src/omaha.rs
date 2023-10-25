use aya_base::{Hand, CARDS};

use crate::{ace_five_rank, insert_cards, poker_rank, AceFiveHandRank, PokerHandRank};

/// Returns the rank of the best 5-card poker hand that can be made with
/// two hole cards and three board cards.
///
/// If there are fewer than 2 hole cards or fewer than 3 board cards, it
/// returns a ranking of Invalid (0).
///
/// # Panics
///
/// Panics if the same card appears in both the hole and board cards.
///
/// # Examples
/// ```
/// use aya_poker::omaha_rank;
///
/// let hole_cards = "Jd 7s 4d 2c".parse()?;
/// let board_cards = "4s 6c Jc 2d Js".parse()?;
/// let rank = omaha_rank(&hole_cards, &board_cards);
/// # Ok::<(), aya_poker::base::ParseError>(())
/// ```
pub fn omaha_rank(hole: &Hand, board: &Hand) -> PokerHandRank {
    let mut buffer = [CARDS[0]; 7];
    let hole_cards = insert_cards(hole, &mut buffer);

    let mut buffer = [CARDS[0]; 7];
    let community_cards = insert_cards(board, &mut buffer);

    let mut max_rank = PokerHandRank(0);
    for i1 in 0..(community_cards.len() - 2) {
        for i2 in (i1 + 1)..(community_cards.len() - 1) {
            for i3 in (i2 + 1)..community_cards.len() {
                let mut community_cards_hand = Hand::new();
                community_cards_hand.insert_unchecked(&community_cards[i1]);
                community_cards_hand.insert_unchecked(&community_cards[i2]);
                community_cards_hand.insert_unchecked(&community_cards[i3]);

                for j1 in 0..(hole_cards.len() - 1) {
                    for j2 in (j1 + 1)..hole_cards.len() {
                        let mut hand = community_cards_hand;
                        hand.insert_unchecked(&hole_cards[j1]);
                        hand.insert_unchecked(&hole_cards[j2]);
                        let rank = poker_rank(&hand);
                        max_rank = max_rank.max(rank);
                    }
                }
            }
        }
    }

    max_rank
}

/// Returns the rank of the best 5-card ace-five lowball poker hand that can
/// be made with precisely two hole cards and three cards from the board.
///
/// If there are fewer than 2 hole cards or fewer than 3 board cards, it
/// returns a ranking of Invalid (0).
///
/// # Panics
///
/// Panics if the same card appears in both the hole and board cards.
///
/// # Examples
/// ```
/// use aya_poker::omaha_lo_rank;
///
/// let hole_cards = "Ks Jd 6h Jc".parse()?;
/// let board_cards = "Jh Td Kd As Js".parse()?;
/// let rank = omaha_lo_rank(&hole_cards, &board_cards);
/// # Ok::<(), aya_poker::base::ParseError>(())
/// ```
pub fn omaha_lo_rank(hole: &Hand, board: &Hand) -> AceFiveHandRank {
    let mut buffer = [CARDS[0]; 7];
    let hole_cards = insert_cards(hole, &mut buffer);

    let mut buffer = [CARDS[0]; 7];
    let community_cards = insert_cards(board, &mut buffer);

    let mut max_lo_rank = AceFiveHandRank(0);
    for i1 in 0..(community_cards.len() - 2) {
        for i2 in (i1 + 1)..(community_cards.len() - 1) {
            for i3 in (i2 + 1)..community_cards.len() {
                let mut community_cards_hand = Hand::new();
                community_cards_hand.insert_unchecked(&community_cards[i1]);
                community_cards_hand.insert_unchecked(&community_cards[i2]);
                community_cards_hand.insert_unchecked(&community_cards[i3]);

                for j1 in 0..(hole_cards.len() - 1) {
                    for j2 in (j1 + 1)..hole_cards.len() {
                        let mut hand = community_cards_hand;
                        hand.insert_unchecked(&hole_cards[j1]);
                        hand.insert_unchecked(&hole_cards[j2]);
                        let lo_rank = ace_five_rank(&hand);
                        max_lo_rank = max_lo_rank.max(lo_rank);
                    }
                }
            }
        }
    }

    max_lo_rank
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::ParseError;

    #[test]
    fn omaha_rank_ordering() -> Result<(), ParseError> {
        let hands = [
            // High cards
            ("9c Ts", "Qd 8h 3d 7c 2h"),          // Q T 9 8 7
            ("As 5h 9d 7c", "6s 8d 3h Js 2d"),    // A J 9 8 6
            ("9d Th 7d As", "2h 5s 3h Jh 4d"),    // A T J 5 4
            ("Js 4d 2c 3s 6d", "Ks 7h 8h Qc As"), // A K Q J 6
            ("Js 4c Kd 6h", "Ac Qh 9s 5c 2h"),    // A K Q J 9
            // Pairs
            ("Th 9h Jc 7s", "2c 6s As 2s 5c"),    // 2 A J T
            ("Tc 2s 6s 9d", "3h 7s Jh Ks 3d"),    // 3 K T 9
            ("4c Js 8d 9d", "3h 2d 6c 3c Kc"),    // 3 K J 9
            ("3s 2h Qh Ah", "Td 7h 6c 3c 8c"),    // 3 A T 8
            ("5d 2h 5s 5c", "Kd Td Jc 4d 9c"),    // 5 K J T
            ("2d 4h Js Ts", "7s 5h As 9c 5d"),    // 5 A J T
            ("8s 6s", "9h Qs Jd 6d Ts"),          // 6 Q J 8
            ("6s 9s Kd Qd", "5s 4h 3h 8d 6d"),    // 6 K 8 5
            ("8c 7h 5h 9h", "4d 7c Td 2s Qd"),    // 7 Q T 9
            ("8d 2c Qs 5h 6d", "Ts Jc 7h 7s 4s"), // 7 Q J 8
            ("3h Qs 8c 2s", "Ah 7d Td 7h 4s"),    // 7 A Q 8
            ("8h 8d 6d Th", "3s 7c Ac 4c Kh"),    // 8 A K 7
            ("7h 8s Ts 3c", "Ks Jc 9h 9c 4h"),    // 9 K T 8
            ("2c Qh 7d 5s", "6h Js 3s 8s Jc"),    // J Q 8 7
            ("5h Jh Kh 2d", "9h Js Td 4d 6h"),    // J K T 9
            ("2h Kd 7s 6d", "Qc Jc Jh Ah 8s"),    // J A K 7
            ("Qd 2h Qc Jd", "4c 2c 7c 3d 5d"),    // Q 7 5 4
            ("8c Qs 6d Jh", "7c Qh Ts 2h 4h"),    // Q J T 7
            ("4h 2c 3c Ks Js", "Qd Tc 5d 8h Qc"), // Q K J T
            ("7d 2h 8s Qh", "4c 3h 9s Qs Ah"),    // Q A 9 8
            ("Qh 7c Qs 4d 4h", "5h 9d 8c Kc Ac"), // Q A K 9
            ("Qh 8s 6h 2c Kc", "4s 9d As Qd Jc"), // Q A K J
            ("Td 5d Ks 5s", "3h Kh 6c 8c 2s"),    // K T 8 6
            ("Qh 5c 4h Jc Th", "As Kh 2s 7h Ks"), // K A Q J
            ("5c 4s Ad Th", "Qs 7s Ah 8d 2d"),    // A Q T 8
            ("Qs Js As 4c", "Ac 7s 2c 9d Th"),    // A Q T 9
            ("Qs Jc 8s 7h", "Ad Ac 2s 5s 4c"),    // A Q J 5
            ("Ad 7c Ah 8c", "Jh 4s 9c 2c Qd"),    // A Q J 9
            // Two pairs
            ("2d 4s 8d 9c", "4d Jh Tc 2c Kc"),    // 4 2 K
            ("8c 4d 5s Td", "3s 3d 4c 6h Js"),    // 4 3 J
            ("4s 5s Jc Qh", "Kc 5c Ts 2h 2c"),    // 5 2 Q
            ("4s Qd 6c Ks 4d", "5s 2s Jh 6h 2c"), // 6 2 K
            ("5s 7h Jh 5c", "3d 4c Kc 7s 3c"),    // 7 3 J
            ("6s 7c Ad Ac", "6d 4s 7s Kh 3c"),    // 7 6 K
            ("9c 2c 8c 4h", "Kd 9h 3c 3d 6c"),    // 9 3 K
            ("8h 9s", "Th 3d Ts 9c 8c"),          // T 9 8
            ("6d 2d Qh Kc", "8d Jc 2h Jh As"),    // J 2 A
            ("Qs Js 3d 5h", "Jd 3c 2d 7c 9s"),    // J 3 9
            ("Jc 7d Ac 8s", "7h 3c 3s Js 5d"),    // J 7 5
            ("3s Ts Jc Qs", "7h 5c Td 3c Js"),    // J T 7
            ("9c Td Tc 7c", "9h Js Jh 7d 3d"),    // J T 9
            ("Ah Qh 2c Td Kd", "Jd 6c Tc Jc 7h"), // J T A
            ("Ts 3c Qs Ac", "Jd Qd 3d 2d 8c"),    // Q 3 J
            ("As Kh Qh 4s", "Qc Jd 8s 7d 4h"),    // Q 4 J
            ("3s 4d Tc 7h", "Qc 3h Jc Qh Th"),    // Q T 7
            ("3s 4d Td Ts", "6c Ac 9h Qh Qd"),    // Q T A
            ("Ks 8s 5h Js", "Kh 9s 8d Qc 7h"),    // K 8 Q
            ("Td 6d 4h 5c Ts", "Ks Kh 8c 8d 7d"), // K T 8
            ("8d 5h 5s 4h", "3s Ts 4d Ad Ac"),    // A 4 8
            ("Ac 4h As Kh", "7h 7d 3c Td 5s"),    // A 7 T
            ("3d 8c 6h Ts", "As 5h 8s Js Ad"),    // A 8 T
            ("As Qd Ad 8s", "3h 9h 7h 9s 4h"),    // A 9 7
            ("Jc 9s Ts Kd", "Td As 4d 2s Ac"),    // A T K
            // Sets
            ("Jd 8h 7h 8c 7s", "5h 6c 7d Kc 2h"), // 7 K 6
            ("Jc 2s 6d Jd 3c", "9c 4h 7d Ks Js"), // J K 7
            ("Qh Qc", "Qs Kc 3s 4d 5h"),          // Q K 5
            ("Qd As 7h 8c", "Qc Qs Kc Js 6d"),    // Q A K
            ("Ah 6c As Qd", "Ad Tc 3s 9c 5c"),    // A T 9
            ("Ad 4d Tc 6h", "5h Jd Ah 9d Ac"),    // A J T
            // Straights
            ("Js Kh Ah 3c Jd", "5d 2d Qd As 4h"), // 5
            ("Js 5s Ad 6s", "2s 2d 4c Jc 3h"),    // 6
            ("8d Jd Qh 7h 6c", "Qc 3h 4c 5d Kh"), // 7
            ("4c Qd Ts 5c 7s", "As 6s 4d Ac 8h"), // 8
            ("6h 5s", "7s 5c 8d Th 9h"),          // 9
            ("Tc 4s Jh 7d", "6d 9h 8h Kc 4d"),    // T
            ("2d 7s Tc 6h", "2c Jd 9s Th 8h"),    // J
            ("Tc Js 7h 3h 4s", "9d 6h 9h Qd 8s"), // Q
            ("3h Kc Th 9h", "5d Js Qs Ks 4h"),    // K
            // Flushes
            ("4s 6d 3h 8s", "9c 7s 2s Kd 5s"),    // 8 7 5 4 2
            ("4d 9c Qc 6d", "2d 5c 3c 8d 7d"),    // 8 7 6 4 2
            ("2c 7c Ad 5d Td", "4c 3c 2s 6h 9c"), // 9 7 4 3 2
            ("8c Qd 4h Tc", "2c Ad 7c 9c Kh"),    // T 9 8 7 2
            ("6s 7s 5h 3s", "Qs 4s 5d 5s 9d"),    // Q 7 6 5 4
            ("Td 4d", "7d Ts 8d Kd 7c"),          // K T 8 7 4
            ("6d 5c 3c 4h", "2s 2c Kc Ac 9s"),    // A K 5 3 2
            ("Ac Th 6c 2c", "Kc Td 7c Qd Tc"),    // A K T 7 6
            ("9h 4c 5s Kh 6c", "5d Ah 2d Qh 3h"), // A K Q 9 3
            // Full houses
            ("2d 7d Jh 5s", "Tc 2h 5d Kc 2c"),    // 2 5
            ("3c 9s Jd Ts", "3d 9c 3h Kc 7s"),    // 3 9
            ("8d Js Kh 3c 4s", "3d 4c 6h 4d 7c"), // 4 3
            ("8h 4d", "5c 4h 4c Th 8c"),          // 4 8
            ("9s 7c 9c 3s Kc", "9h As Kd 8c Ks"), // 9 K
            ("Ah 9h 9d Ac", "Kd As 2h 2d Ts"),    // A 2
            ("6h Ac Kh 9c 8d", "2h Ts Kc As Ad"), // A K
            // Quads
            ("Qc 3c 3d 9d", "3s 7h 8s 9c 3h"),    // 3 Q
            ("Ac Qh Jh 4d 4s", "5d 5h 4c 7s 4h"), // 4 7
            ("Kd 9c", "9h 5d 9s 9d 4h"),          // 9 K
            ("9h Tc Kc Qh", "6d Ts Kd Kh Ks"),    // K Q
            ("Kh Ac 7s 8c Th", "Ah 5c Ad As Ks"), // A K
            // Straight flushes
            ("Qh 6s Kd 4s", "2d 3s 5s 9h 2s"),    // 6
            ("Qd 6c Kh Js 5c", "7c 4c 2s 3c 2c"), // 7
            ("9c 5h 6s 7d 6h", "7h 9d Jh 8h 4h"), // 8
            ("5d 8d", "7s 9d 7d 6s 6d"),          // 9
            ("Qs As 7h 6s 8h", "6h Th 7d 9h 5h"), // T
            // Royal flush
            ("Th Jc Ac Qd", "Tc Qc Ad Kc 9c"), // A
        ]
        .map(|(h, b)| (h.parse(), b.parse()));

        for (&(h1, b1), &(h2, b2)) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = omaha_rank(&h1?, &b1?);
            let r2 = omaha_rank(&h2?, &b2?);

            assert!(
                r1 < r2,
                "[{:?}], [{:?}] is ranked {:?}, which is larger than [{:?}], [{:?}] ({:?}).",
                h1,
                b1,
                r1,
                h2,
                b2,
                r2
            );
        }

        Ok(())
    }

    #[test]
    fn omaha_rank_lo_ordering() -> Result<(), ParseError> {
        let hands = [
            // Full houses
            ("Qc Qs", "Kc Kh Kd Qd Qh"), // Q K
            ("Kh Kd", "Jc Ks Js Kc Jd"), // J K
            // Two pairs
            ("Kc Ks Qh Qs", "Kh Qc Kd Qd Jh"), // K Q J
            ("Jc Qs", "Js Qc Jh Qd Kc"),       // Q J K
            ("Ts 5d Qc 5h", "Tc 5s Td Qs Qh"), // T 5 Q
            ("9s 9d 8d 8s", "Td 9h 8h 9c Th"), // 9 8 T
            ("7c 7s", "7d 9h 9s 9c Kh"),       // 9 7 K
            ("Ah 8s Ac Qd", "As 8d Qc 8h 8c"), // 8 A Q
            ("7d Ad", "Qc 7c Ac 7h As"),       // 7 A Q
            ("4d 8s 4h 4c", "5h 5d 8c 5c 8h"), // 5 4 8
            ("3c 5s 5c 9s", "3d 3h 5d 5h 9h"), // 5 3 9
            ("Ac 2h Ah 2d", "As 3c 3s 2s 2c"), // 2 A 3
            // Pairs
            ("9s Qh Qd Kh", "Kc Qs 2s 9d Qc"), // 9 K Q 2
            ("4h 9h 7h 9s", "8s 8h 8c 8d Th"), // 8 T 7 4
            ("6c 6s 6d 6h", "Tc Ac 4s 8c 5h"), // 6 5 4 A
            ("Tc Td 7c 7s", "Qd Qs 5h 7d 5c"), // 5 Q T 7
            ("8s 2s Jh 7c", "5d Ac 5h 5s 5c"), // 5 7 2 A
            ("4h 4d 6h Ts", "Ks 6c 4c Kh 6d"), // 4 K T 6
            ("7d Qd 5d 9s", "Tc 4c 4h 4s Td"), // 4 T 7 5
            ("9c 4s", "4h 7c 6h 9h 6c"),       // 4 9 7 6
            ("Ah 9s 5s 7h", "4d 4h 4c 7s 7d"), // 4 7 5 A
            ("3d 3h 3s Jc", "Jh Js Qc 6h Jd"), // 3 Q J 6
            ("9s 3h 6s 6h", "9d 3c Qc 6d 6c"), // 3 Q 9 6
            ("Th Tc 7s 2s", "6d 2c 7h Td 6h"), // 2 T 7 6
            ("4s 4c 2s 4d", "6h 6d 2d 7d 7c"), // 2 7 6 4
            ("7h 9d 4h 9h", "Kd Ks Ah Ad Kh"), // A K 7 4
            ("Ad Qh Ac Qs", "3h 8s Qc Ah 8c"), // A Q 8 3
            ("7s 5s 2c 9d", "Ac Jd Jc Jh Ah"), // A J 5 2
            ("4d Qc Js Ts", "2d 2c Ad Ac 2h"), // A T 4 2
            ("9d 4s 3s 8d", "Ad As Ac 9s Ah"), // A 9 4 3
            // High cards
            ("Ks Qd", "Jc Ts 9c Jd Th"),          // K Q J T 9
            ("8d Qh Td Jc", "Tc Ah Ks Kd Kc"),    // K Q T 8 A
            ("9h 7h 2s Kh", "7d Ks Ac 2h 7s"),    // K 9 7 2 A
            ("Kh Qd Jd 6s", "Ks Qs 5d 5s 4c"),    // Q J 6 5 4
            ("6h Qs", "3h 3d 9d Tc 6d"),          // Q T 9 6 3
            ("Td 4c Ad Kh", "9c Tc Qh Qc 9s"),    // Q T 9 4 A
            ("Qd 3c", "Tc Js 7h Th 4h"),          // Q T 7 3 4
            ("8h Qd Qh Kh", "Ks 3c Kc 2c 9s"),    // Q 9 8 3 2
            ("Ts 4h 9c 3h", "Kd 2s 2c Qc 4d"),    // Q 9 4 3 2
            ("4h 5s 5d Td Jh", "5c Kh Jc 8d 8s"), // J T 8 5 4
            ("Td Jc Ts 6s", "5d Ks Jh Kh 4h"),    // J T 6 5 4
            ("Ah Jh Ad Th", "Ts 2s 2h 3h 2c"),    // J T 3 2 A
            ("Jc Th 4s 9h", "Jh Ks 8h As Js"),    // J 9 8 4 A
            ("6h Qh 7s 4s", "Js 4c Kh 6d 9h"),    // J 9 7 6 4
            ("Ah Js Ac 3d", "Kc Jh 8d 2h 3c"),    // J 8 3 2 A
            ("7s 5c 3d 5s", "Js 5d Jd Kd 6s"),    // J 7 6 5 3
            ("8d 2c 2s 7h", "5c Kc 5s Jd 4d"),    // J 7 5 4 2
            ("3d 3c 7h Qh", "7d 5d 3s Jh Ad"),    // J 7 5 3 A
            ("Kc 8d 2c Ac Qh", "Qd Kd 7s 4s Jd"), // J 7 4 2 A
            ("6s 9c Kc 4c", "Qh 5s 2s Js Jh"),    // J 6 5 4 2
            ("2d As 4h Jd", "4d Qs Jc Jh 6d"),    // J 6 4 2 A
            ("Th 6d 8h Tc", "9c 7d 6s 9s 9h"),    // T 9 8 7 6
            ("Ad 8d 9h As", "9c Th 4h Qc 8c"),    // T 9 8 4 A
            ("Th Qc 8h 7c", "4h 3d 4c Ts 3s"),    // T 8 7 4 3
            ("Ts 5s 8s 5h", "3h 6s Jh Tc Kd"),    // T 8 6 5 3
            ("5h 8d Qd Ts", "6c 2c 8c 2d Qh"),    // T 8 6 5 2
            ("7d 6d 3s Js", "8c Jc 8d 2d Ts"),    // T 8 6 3 2
            ("4h 4d Qs 6h 5c", "8c Td 2d Kh Tc"), // T 8 5 4 2
            ("Th 2h", "4h 8s As 9h Ah"),          // T 8 4 2 A
            ("6d Ah 2d 2h Jd", "8d 3d Kh Td Kc"), // T 8 3 2 A
            ("5h Jd 4s 7s", "Th 6d Js 5d Qd"),    // T 7 6 5 4
            ("5c 8s 4c 9c 6d", "Ad Qh 7h Th 7d"), // T 7 5 4 A
            ("Ac Jc As 7s", "2c 4s Js 4h Th"),    // T 7 4 2 A
            ("2d 4c 7d Kd", "5h Td 5c 6s Jc"),    // T 6 5 4 2
            ("2d 3h 7s 5d 4d", "6h 5s Kc Js Ts"), // T 6 5 3 2
            ("3c Ah", "Ks 2s 5s Th Ts"),          // T 5 3 2 A
            ("Qh 9c 8h Kc", "2s 5c 9h Th 7d"),    // 9 8 7 5 2
            ("8h Js Jh 7s", "9s Ac Ah Tc 5s"),    // 9 8 7 5 A
            ("9c 8d 4d 9d", "9h 8c 8s 6d 5d"),    // 9 8 6 5 4
            ("7c Js 6h Tc 2h", "8h 5d Kh 9d 8d"), // 9 8 6 5 2
            ("4s 9c Tc 8s", "8d 4d 5c 9s 3d"),    // 9 8 5 4 3
            ("As Ks 8s Kc", "5c 5d 4h Qc 9c"),    // 9 8 5 4 A
            ("9h 4d Qc Td", "Ac Qh 8s Tc 2h"),    // 9 8 4 2 A
            ("9s 6c Jh 4h", "9h 5d 7d 7s Ts"),    // 9 7 6 5 4
            ("9c 4h Td 4d", "6h Th Kh 3c 7d"),    // 9 7 6 4 3
            ("4d 2h 6c 7c Kc", "Qh Jc 4s 9s 6s"), // 9 7 6 4 2
            ("2h 6s 7s 3s", "Kc 9h 2d 3d Qs"),    // 9 7 6 3 2
            ("5c 4c Jc Qh 2d", "7d 2c 4s 9h 2h"), // 9 7 5 4 2
            ("Qs 5d Jd 9d", "Kh As Jc 2d 7s"),    // 9 7 5 2 A
            ("9d 4h Jh 7h 2h", "Jc 2c As Ac 9s"), // 9 7 4 2 A
            ("3s 6c 6h 9d", "Th Kd 5h 4d 9h"),    // 9 6 5 4 3
            ("Js 9s 3s 5d", "Qd Jd 6c 2d 3c"),    // 9 6 5 3 2
            ("2s 7s Jd Ah", "6h 3s Qd 9s Kh"),    // 9 6 3 2 A
            ("7s Js Qh Qd 8d", "6h Qc 2s 5h Ks"), // 8 7 6 5 2
            ("3d Jh 4s Qc", "7h 8c 6s Js Kh"),    // 8 7 6 4 3
            ("2d Js 4d Ks", "5d 8h Kc 6h 5s"),    // 8 6 5 4 2
            ("8d 2h 9c 2d", "6s As 5d Qh Kd"),    // 8 6 5 2 A
            ("3c Ks 6c 2h", "4c 3s 2s 6h 8d"),    // 8 6 4 3 2
            ("7h 3d 4h 4d", "6s Qc 8d 9c Ah"),    // 8 6 4 3 A
            ("3d 7c Ac 2s", "Ad As 8c Ks 6h"),    // 8 6 3 2 A
            ("Ac 6h Qs Jh 6d", "7s 2s 9d 5c Qd"), // 7 6 5 2 A
            ("Qs 7d 4h 4c", "6s 9s 3d 7s Ah"),    // 7 6 4 3 A
            ("6s 4h Jd 7s Th", "2d Qs Qd Ad 7c"), // 7 6 4 2 A
            ("Jh 8s Ad 4d", "Ac 5s Ts 3d 7d"),    // 7 5 4 3 A
            ("9s Ac 2c 4c", "2h 4s Qs 5h 7c"),    // 7 5 4 2 A
            ("4d Th 8h 6d Td", "Jd 3d 2h Qs 5s"), // 6 5 4 3 2
            ("Ah 6c 4d 9c 3c", "7d 4h 5h 9s 6h"), // 6 5 4 3 A
            ("2s 7s 5h Qs", "6d Ac 4s 6c 6s"),    // 6 5 4 2 A
            ("6c 9c 2d 6s", "Ah 3s Qd 8h 5d"),    // 6 5 3 2 A
            ("Qs Kh 4d 3d", "Tc Kd 5c As 2d"),    // 5 4 3 2 A
        ]
        .map(|(h, b)| (h.parse(), b.parse()));

        for (&(h1, b1), &(h2, b2)) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = omaha_lo_rank(&h1?, &b1?);
            let r2 = omaha_lo_rank(&h2?, &b2?);

            assert!(
                r1 < r2,
                "[{:?}], [{:?}] is ranked {:?}, which is larger than [{:?}], [{:?}] ({:?}).",
                h1,
                b1,
                r1,
                h2,
                b2,
                r2
            );
        }

        Ok(())
    }
}
