//! # 원형
//! 객체 생성. 원형이 되는 인스턴스를 사용하여 생성할 객체의 종류를 명시하고, 이렇게 만든 견본을 복사해서 새로운 객체를 생성한다.
//!
//! ## 사용
//! * 인스턴스화할 클래스를 런타임에 저장할 때
//! * 제품 클래스 계통과 병렬적으로 만드는 팩토리 클래스를 피하고 싶을 때
//!
//! ## 구현
//! 1. 비싼 작업과 그 작업을 사용하는 비싼 제품이 있어야 한다.
//! 2. 비싼 작업과 그 작업을 사용하는 비싼 제품을 복제할 수 있도록 메서드를 추가한다.
//! 3. 프로그램이 실행될 때 비싼 제품을 미리 하나씩 만들어둔다.
//! 3. (선택) 저장소를 만들어서 만들어둔 비싼 제품들을 하나씩 저장한다.
//! 4. 사용자가 호출하면 비싼 제품을 복사하여 제공한다. 이러면 비싼 제품을 만드는 비용보다 싸게(빠르게) 객체를 얻을 수 있다.
//!
//! Shape은 비싼 작업을 가리킨다. ShapeClone과 Clone은 이 작업을 수행한 객체를 복사할 수 있게 한다.
//! Circle과 Rectangle은 Shape을 수행하는 비싼 제품이다.
//! ShapeRegistry는 제품들을 하나씩 미리 저장해두는 저장소이다. 사용자는 여기서 원하는 제품을 복사해서 빠르게 객체를 얻을 수 있다.
use std::collections::HashMap;

// 비싼 작업
trait Shape: ShapeClone {
    fn draw(&self);
    fn set_color(&mut self, color: &str);
}

// 비싼 작업을 복제할 수 있게 함.
trait ShapeClone {
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl<T: 'static + Shape + Clone> ShapeClone for T {
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
/// 비싼 작업을 요구하는 비싼 제품 1
struct Circle {
    radius: f64,
    color: String,
}

impl Shape for Circle {
    fn draw(&self) {
        println!("반지름 {:.1}인 {} 원 그리기", self.radius, self.color);
    }
    fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }
}

#[derive(Clone)]
/// 비싼 작업을 요구하는 비싼 제품 2
struct Rectangle {
    width: f64,
    height: f64,
    color: String,
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("{:.1}x{:.1} 크기의 {} 사각형 그리기", self.width, self.height, self.color);
    }
    fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }
}

// 프로토타입을 보관하고 필요 시 복제해 반환하는 저장소
struct ShapeRegistry {
    prototypes: HashMap<String, Box<dyn Shape>>,
}

impl ShapeRegistry {
    fn new() -> Self {
        let mut r = ShapeRegistry { prototypes: HashMap::new() };
        r.prototypes.insert("circle".to_string(), Box::new(Circle {
            radius: 5.0, color: "red".to_string(),
        }));
        r.prototypes.insert("rect".to_string(), Box::new(Rectangle {
            width: 10.0, height: 4.0, color: "blue".to_string(),
        }));
        r
    }

    fn create(&self, kind: &str) -> Option<Box<dyn Shape>> {
        self.prototypes.get(kind).map(|p| p.clone()) // 복제로 새 인스턴스 생성
    }
}

pub fn main() {
    let registry = ShapeRegistry::new();

    let mut c = registry.create("circle").unwrap();
    c.set_color("green");
    c.draw();

    let r = registry.create("rect").unwrap();
    r.draw();

    // 원본 프로토타입은 변경되지 않음
    let c2 = registry.create("circle").unwrap();
    c2.draw(); // 여전히 "red"
}
