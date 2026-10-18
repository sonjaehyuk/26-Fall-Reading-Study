//! # 플라이급
//!
//! 공유를 통해 많은 수의 소립 객체들을 효과적으로 지원한다.
//! 플라이급 객체는 공유 가능한 객체이다. 본질적 상태는 플라이급 객체에 저장되어야 하는 것으로, 상황에 관계 없이 동일하게 동작할 수 있어야 한다.
//! 그렇기에 본질적 상태는 공유 가능한 것이다. 반면 부가적 상태는 상황에 따라 달라질 수 있는 것으로, 공유될 수 없다.
//!
//!
//! ## 사용
//! * 응용프로그램이 대량의 객체를 사용해야 할 때
//! * 객체의 수가 너무 많아져 저장 비용이 너무 높아질 때
//! * 대부분의 객체 상태를 부가적인 것으로 만들 수 있을 때
//!
//! ## 구성
//! * ConcreteFlyweight(CharacterType): 본질적 상태를 가지고 있는 플라이급 객체.
//! * FlyweightFactory(CharacterFactory): 플라이급 객체를 생성하고 관리하며, 공유하도록 도와주는 객체
//! * Client(Character): 플라이급 객체의 부가적 상태를 저장한 완전한 객체.
//!
//! FlyweightFactory에 객체 생성 요청이 왔을 때, ConcreteFlyweight를 새로 만들지 않고 중복(참조) 사용할 수 있게 만드는 게 핵심이다.
//! 여기서는 HashMap이라는 집합형을 사용했다. 

use std::collections::HashMap;

// 공유 가능한 본질적 상태
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CharacterType {
    symbol: char,
    font: String,
    size: u8,
}

impl CharacterType {
    fn render(&self, x: i32, y: i32) {
        println!("'{}' ({}, {}) 위치에 렌더링", self.symbol, x, y);
    }
}

// Flyweight Factory: 이미 생성된 Flyweight를 재사용
struct CharacterFactory {
    pool: HashMap<CharacterType, CharacterType>,
}

impl CharacterFactory {
    fn new() -> Self {
        CharacterFactory {
            pool: HashMap::new(),
        }
    }

    fn get_character(&mut self, symbol: char, font: &str, size: u8) -> CharacterType {
        let key = CharacterType {
            symbol,
            font: font.to_string(),
            size,
        };

        self.pool
            .entry(key.clone())
            .or_insert(key)
            .clone()
    }

    fn created_count(&self) -> usize {
        self.pool.len()
    }
}

// Client: Flyweight와 부가적 상태를 조합. 사용할 수 있는 완전한 객체.
struct Character {
    character_type: CharacterType,
    x: i32,
    y: i32,
}

impl Character {
    fn render(&self) {
        self.character_type.render(self.x, self.y);
    }
}

fn main() {
    let mut factory = CharacterFactory::new();

    let chars = vec![
        Character {
            character_type: factory.get_character('H', "Arial", 12),
            x: 0,
            y: 0,
        },
        Character {
            character_type: factory.get_character('e', "Arial", 12),
            x: 10,
            y: 0,
        },
        Character {
            character_type: factory.get_character('l', "Arial", 12),
            x: 20,
            y: 0,
        },
        Character {
            character_type: factory.get_character('l', "Arial", 12),
            x: 30,
            y: 0,
        },
        Character {
            character_type: factory.get_character('o', "Arial", 12),
            x: 40,
            y: 0,
        },
    ];

    for c in &chars {
        c.render();
    }

    println!("실제 생성된 Flyweight 개수: {}", factory.created_count());
}
