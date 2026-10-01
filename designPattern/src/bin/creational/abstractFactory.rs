//! # 추상팩토리
//! 객체 생성. 상세화된 서브클래스를 정의하지 않고도 서로 관련성이 있거나 독립적인 여러 객체의 군을 생성하기 위한 인터페이스
//!
//! ## 언제 쓰는가
//! * 여러 제품군 중 하나를 선택해서 설정해야 하고 그걸 다른 것으로 대체할 수 있을 때
//! * 관련된 제품 객체들이 함께 사용되도록 설계되었고 외부에서도 지켜져야 할 때
//! * 구현이 아닌 인터페이스를 노출시키고 싶을 때
//!
//! ## 어떻게 만드는가
//!
//! 1. 만들고자 하는 제품(객체)을 정의한다. button, checkbox 등.
//! 2. 각 상황에서 그 제품을 구현하는 방법들을 정의한다. 예를 들어, 윈도우에서 구현하는 방법, 맥에서 구현하는 방법 같이 목적은 같지만 세부 방법이 다른 것들에 주로 사용한다.
//! 3. 제품을 구현하는 방법들을 추상화한다. 예를 들어, 세부 방법은 다르지만 목적이 같은 게 있다면(button을 만들어라, checkbox를 만들어라), 그 목적을 메서드로 정의한다. 그걸 추상 팩토리라고 부른다.
//! 4. 추상 팩토리를 이용해서 만들고자 하는 제품들을 만들 수 있도록, 실제 팩토리를 작성한다.
//! 5. 최종적으로 각 상황의 구현은 신경쓸 필요 없이 추상 팩토리만 알아도 모든 객체를 만들 수 있게 된다.
//!
//! 아래 예제는 윈도우와 mac에서 GUI 처리를 다르게 한다고 가정하고 만든 예제임.
//! trait으로 Button과 Checkbox가 있는데, WindowsButton과 WindowsCheckbox는 윈도우용, MacButton과 MacCheckbox는 맥용임.
//! 추상 팩토리는 GUIFactory로, GUIFactory를 구현하기만 했다면 윈도우와 맥 모두 동일하게 제품을 만들어낼 수 있게 됨.
//! 윈도우와 맥의 제품을 만들어내는 GUIFactory 구현체가 WindowsFactory, MacFactory임.

trait Button {
    fn render(&self);
    fn on_click(&self);
}

trait Checkbox {
    fn render(&self);
    fn toggle(&self);
}

// Windows용 제품
struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) { println!("[Windows] 버튼 렌더링"); }
    fn on_click(&self) { println!("[Windows] 버튼 클릭 이벤트"); }
}
struct WindowsCheckbox;
impl Checkbox for WindowsCheckbox {
    fn render(&self) { println!("[Windows] 체크박스 렌더링"); }
    fn toggle(&self) { println!("[Windows] 체크박스 토글"); }
}

// macOS용 제품
struct MacButton;
impl Button for MacButton {
    fn render(&self) { println!("[macOS] 버튼 렌더링"); }
    fn on_click(&self) { println!("[macOS] 버튼 클릭 이벤트"); }
}
struct MacCheckbox;
impl Checkbox for MacCheckbox {
    fn render(&self) { println!("[macOS] 체크박스 렌더링"); }
    fn toggle(&self) { println!("[macOS] 체크박스 토글"); }
}

// 추상 팩토리
trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

// 구체 팩토리
struct WindowsFactory;
impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}

struct MacFactory;
impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}

fn app(factory: &dyn GUIFactory) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();

    button.render();
    button.on_click();
    checkbox.render();
    checkbox.toggle();
}

pub fn main() {
    print!("당신의 OS는 무엇입니까? ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("입력 실패");
    match input.trim().to_lowercase().as_str() {
        "macos" => {
            println!("macOS용 객체를 생성합니다.");
            app(Box::new(MacFactory).as_ref());
        },
        "windows" => {
            println!("Windows용 객체를 생성합니다.");
            app(Box::new(WindowsFactory).as_ref());
        },
        _ => {
            println!("잘못된 입력입니다.")
        }
    }
}