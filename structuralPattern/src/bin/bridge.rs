//! # 가교
//!
//! 구현에서 추상을 분리하여, 이들이 독립적으로 다양성을 가질 수 있도록 한다.
//! 큰 개념과 그 구체적 구현을 느슨하게 분리하는 것이라고 이해할 수 있다.
//!
//! ## 사용
//! * 추상적 개념과 이에 대한 구현 사이의 지속적인 종속 관계를 피하고 싶을 때.
//! * 추상적 개념에 해당하는 클래스를 사용하는 코드들은 구현 클래스가 변경되었다고 해서 다시 컴파일되지 않아야 할 때.
//!
//! ## 구성
//!
//! * Abstraction: Shape. 클라이언트가 사용하는 메서드를 정의한다. 클라이언트가 바라보는 상위 개념이라고 할 수 있다.
//! * RefinedAbstraction: Circle, Square. Shape을 구체화한 다양한 형태를 구현하는데, 이때 Implementor를 사용한다.
//! * Implementor: Renderer. 공통 연산을 정의한다.
//! * ConcreteImplementor: VectorRenderer, RasterRenderer 등 실제 구현을 수행한다.
//!
//! 실제 구현에 관련된 코드인 VectorRenderer, RasterRenderer는 자기들이 Shape(Circle, Square)에서 쓰인다는 사실을 모른다.
//! 이로써 구현이 클라이언트가 사용하는 인터페이스에 얽매이지 않게 된다.

// Implementor: 구현 측의 공통 인터페이스
trait Renderer {
    fn render_circle(&self, x: f64, y: f64, radius: f64);
    fn render_square(&self, x: f64, y: f64, side: f64);
}

// Concrete Implementor: 구체적 구현
struct VectorRenderer;

impl Renderer for VectorRenderer {
    fn render_circle(&self, x: f64, y: f64, radius: f64) {
        println!("벡터 원 그리기: 중심({:.1},{:.1}) 반지름({:.1})", x, y, radius);
    }
    fn render_square(&self, x: f64, y: f64, side: f64) {
        println!("벡터 사각형 그리기: 좌측상단({:.1},{:.1}) 변({:.1})", x, y, side);
    }
}

struct RasterRenderer;

impl Renderer for RasterRenderer {
    fn render_circle(&self, x: f64, y: f64, radius: f64) {
        println!("래스터 원 그리기: 중심({:.1},{:.1}) 반지름({:.1})", x, y, radius);
    }
    fn render_square(&self, x: f64, y: f64, side: f64) {
        println!("래스터 사각형 그리기: 좌측상단({:.1},{:.1}) 변({:.1})", x, y, side);
    }
}

// Abstraction: 클라이언트가 사용하는 추상화 인터페이스
trait Shape {
    fn draw(&self);
    fn resize(&mut self, factor: f64);
}

// Refined Abstraction: 구체적인 추상화
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
    renderer: Box<dyn Renderer>,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64, renderer: Box<dyn Renderer>) -> Self {
        Circle { x, y, radius, renderer }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        self.renderer.render_circle(self.x, self.y, self.radius);
    }

    fn resize(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
    renderer: Box<dyn Renderer>,
}

impl Square {
    fn new(x: f64, y: f64, side: f64, renderer: Box<dyn Renderer>) -> Self {
        Square { x, y, side, renderer }
    }
}

impl Shape for Square {
    fn draw(&self) {
        self.renderer.render_square(self.x, self.y, self.side);
    }

    fn resize(&mut self, factor: f64) {
        self.side *= factor;
    }
}

fn main() {
    let mut circle = Circle::new(0.0, 0.0, 5.0, Box::new(VectorRenderer));
    circle.draw();
    circle.resize(2.0);
    circle.draw();

    println!();

    let square = Square::new(1.0, 1.0, 4.0, Box::new(RasterRenderer));
    square.draw();
}
