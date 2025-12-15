#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

fn print_suit(card: PokerSuit) {
    // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    println!("{:?}", card);
}
fn main() {
    // 任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举。
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);

    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };

    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
}