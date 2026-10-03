//! 단일체
//! 객체 생성. 오직 한 개의 클래스 인스턴스만을 갖도록 보장하고, 이에 대한 전역적인 접근점을 제공한다.
//! 클래스 연산보다 유연하다는 특징이 있다.
//!
//! ## 사용
//! * 클래스의 인스턴스가 오직 하나여야 함을 보장하고, 잘 정의된 접근점으로 모든 사용자가 접근할 수 있도록 해야 할 때
//! * 유일한 인스턴스가 서브클래싱으로 확장되어야 하며, 사용자는 코드의 수정없이 확장된 서브클래스의 인스턴스를 사용할 수 있어야 할 때
//!
//! ## 구현
//! 단일체는 그 개념은 다른 패턴보다 직관적이고 이해하기 쉬우나, 구현은 각 언어와 경우에 따라 천차만별이다.
//! 흔히 static 예약어로 인스턴스 개수를 명시하고, 객체 생성 시 인스턴스 개수를 명시한 변수를 읽어 객체 생성 여부를 판별한다.
//! 그러나 이 방식은 다중 스레드 환경에서 경쟁 상태를 유발하거나 불안정할 우려가 있다.
use std::sync::OnceLock;

struct Config {
    app_name: String,
    max_connections: u32,
}

// 전역 정적 변수로 선언
static CONFIG: OnceLock<Config> = OnceLock::new();

fn config() -> &'static Config {
    CONFIG.get_or_init(|| Config {
        app_name: "MyApp".to_string(),
        max_connections: 100,
    })
}

pub fn main() {
    println!("App: {}", config().app_name);
    println!("Max connections: {}", config().max_connections);
}
