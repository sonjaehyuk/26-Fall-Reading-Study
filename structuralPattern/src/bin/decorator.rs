//! # 장식자
//!
//! 객체에 동적으로 새로운 책임을 추가할 수 있게 한다.
//! 상속 없이도 유연하게 책임을 확장할 수 있도록 하는 구조 패턴이다.
//! 적응자와 함께 둘 다 기존 객체를 내부에 감싸는 형태로 구현되는데,
//! 적응자는 인터페이스 호환을 목표로 한다면 장식자는 새로운 기능과 책임의 추가/제거를 목표로 한다.
//!
//! ## 사용
//! * 다른 객체에 영향을 주지 않고 개개의 객체에 새로운 책임을 추가할 때
//! * 책임이 제거될 수 있을 때
//!
//! ## 구성
//!
//! * Component(Coffee): 동적으로 추가할 서비스를 가질 가능성이 있는 객체들의 인터페이스. 쉽게 말해 기본 인터페이스다.
//! * Concrete Component(SimpleCoffee): 원본 객체
//! * Decorator(CoffeeDecorator, MilkDecorator, SugarDecorator): Component를 감싸며 동일한 인터페이스를 구현한다.
//!
//! Decorator가 자기 속성은 유지하면서 Coffee를 구현하는 방식으로 장식자 패턴을 만든다.
//! 장식자 패턴을 통해 아래와 같은 책임(기능)을 유연하게 조합할 수 있다.
//! ```
//! 기본 커피: $2
//! 기본 커피 + 우유: $2.5
//! 기본 커피 + 우유 + 설탕: $2.8
//! ```


// Component: 기본 인터페이스
trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

// Concrete Component: 기본 구현체
struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        2.0
    }

    fn description(&self) -> String {
        "기본 커피".to_string()
    }
}

// Base Decorator: Component를 감싸는 추상 데코레이터
struct CoffeeDecorator {
    coffee: Box<dyn Coffee>,
}

// Concrete Decorator: 우유 추가
struct MilkDecorator {
    decorator: CoffeeDecorator,
}

impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator {
            decorator: CoffeeDecorator { coffee },
        }
    }
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.decorator.coffee.cost() + 0.5
    }

    fn description(&self) -> String {
        format!("{} + 우유", self.decorator.coffee.description())
    }
}

// Concrete Decorator: 설탕 추가
struct SugarDecorator {
    decorator: CoffeeDecorator,
}

impl SugarDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        SugarDecorator {
            decorator: CoffeeDecorator { coffee },
        }
    }
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> f64 {
        self.decorator.coffee.cost() + 0.3
    }

    fn description(&self) -> String {
        format!("{} + 설탕", self.decorator.coffee.description())
    }
}

fn main() {
    let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee);
    println!("{}: ${}", coffee.description(), coffee.cost());

    let milk_coffee: Box<dyn Coffee> = Box::new(MilkDecorator::new(Box::new(SimpleCoffee)));
    println!("{}: ${}", milk_coffee.description(), milk_coffee.cost());

    let milk_sugar_coffee: Box<dyn Coffee> = Box::new(SugarDecorator::new(Box::new(MilkDecorator::new(Box::new(SimpleCoffee)))));
    println!("{}: ${}", milk_sugar_coffee.description(), milk_sugar_coffee.cost());
}
