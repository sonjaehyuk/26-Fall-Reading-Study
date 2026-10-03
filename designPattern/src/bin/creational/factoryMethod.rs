//! # 팩토리 메서드
//!
//! 클래스 생성. 객체를 생성하기 위한 인터페이스를 정의하지만, 어떤 클래스의 인스턴스를 생성할 지에 대한 결정은 서브클래스가 내린다.
//!
//! ## 사용
//! * 어떤 클래스가 자신이 생성해야 하는 객체의 클래스를 예측할 수 없을 때
//! * 생성할 객체를 기술하는 책임을 자신의 서브클래스가 지정했으면 할 때
//!
//! ## 구현
//!
//! 1. 구현해야 하는 제품(Notification)과 그 상세 구현 방법들을 정의한다.
//! 2. 추상 생성자를 만든다. 추상 생성자에는 팩토리와 템플릿 메서드가 있다. 팩토리 메서드는 상세 구현 방법으로 제품을 구현하도록 지시한다. 템플릿 메서드는 구현에 상관없이 제품을 사용하도록 지시한다.
//! 3. 추상 생성자를 구현하는 구체 생성자를 만든다.
//! 4. 사용자는 추상 생성자를 이용해 프로그래밍한다. 그러면 구체 생성자 중 어떤 것을 선택해도 코드를 변경할 필요가 없다.

/// 추상 제품
trait Notification {
    fn send(&self, message: &str);
}

/// 구체 제품 1
struct EmailNotification;
impl Notification for EmailNotification {
    fn send(&self, message: &str) {
        println!("[Email] 전송: {}", message);
    }
}
/// 구체 제품 2
struct SmsNotification;
impl Notification for SmsNotification {
    fn send(&self, message: &str) {
        println!("[SMS] 전송: {}", message);
    }
}
/// 구체 제품 3
struct PushNotification;
impl Notification for PushNotification {
    fn send(&self, message: &str) {
        println!("[Push] 전송: {}", message);
    }
}

trait NotificationCreator {
    /// 팩토리 메서드: 어떤 제품을 만들지는 구현체가 결정
    fn create_notification(&self) -> impl Notification;

    /// 템플릿 메서드: 생성 로직을 포함한 공통 비즈니스 로직
    fn notify_user(&self, message: &str) {
        let notification = self.create_notification(); // 생성을 하위에 위임
        println!("알림 채널 준비 완료, 발송 시작...");
        notification.send(message);
    }
}

struct EmailCreator;
impl NotificationCreator for EmailCreator {
    fn create_notification(&self) -> impl Notification {
        EmailNotification {}
    }
}

struct SmsCreator;
impl NotificationCreator for SmsCreator {
    fn create_notification(&self) -> impl Notification {
        SmsNotification {}
    }
}

struct PushCreator;
impl NotificationCreator for PushCreator {
    fn create_notification(&self) -> impl Notification {
        PushNotification {}
    }
}

enum CreatorList {
    EmailCreator(EmailCreator),
    SmsCreator(SmsCreator),
    PushCreator(PushCreator),
}

impl CreatorList {
    fn notify_user(&self, message: &str) {
        match self {
            CreatorList::EmailCreator(creator) => creator.notify_user(message),
            CreatorList::SmsCreator(creator) => creator.notify_user(message),
            CreatorList::PushCreator(creator) => creator.notify_user(message),
        }
    }
}

pub fn main() {
    // 설정이나 런타임 조건에 따라 생성자를 선택
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("입력 실패");

    let creator: CreatorList = match input.trim().to_lowercase().as_str() {
        "sms" => CreatorList::SmsCreator(SmsCreator {}),
        "push" => CreatorList::PushCreator(PushCreator {}),
        "email" => CreatorList::EmailCreator(EmailCreator {}),
        _ => panic!("유효하지 않은 입력"),
    };

    // 사용자는 어떤 채널인지 몰라도 됨
    creator.notify_user("주문이 완료되었습니다.");
}
