//! # 프록시
//! 다른 객체에 대한 접근을 제어하기 위한 대리자 객체를 둔다.
//!
//! ## 종류
//! * 원격지 프록시: 서로 다른 주소 공간에 존재하는 객체를 가리키는 대표 객체.
//! * 가상 프록시: 요청이 있을 때만 필요한 고비용 객체를 생성하는 객체.
//! * 보호용 프록시: 원래 객체에 대한 실제 접근을 제어해서 보호하는 객체.
//! * 스마트 참조자: 참조 횟수 관리, 동시 접근 관리 등을 하는 객체
//!
//! ## 구성
//! * Subject(Image): 실제 객체와 프록시 객체의 공통 인터페이스. 실제 객체를 쓰는 자리를 프록시가 쓸 수 있도록 하기 위함이다.
//! * RealSubject(RealImage): 실제 객체이다.
//! * Proxy(Proxy): 실제 대상에 대한 접근을 제어하고 실제 대상의 생성과 삭제를 책임진다.
//!
//! 이 예제에서는 가상 프록시를 사용하여 실제 객체가 꼭 필요한 요청이 처음 들어올 때 객체를 만든다.
//! 이후 이 객체를 재활용하여 다음 번 요청은 더 빠르게 제공할 수 있다.

use std::cell::RefCell;

// Subject: RealSubject와 Proxy가 공유하는 인터페이스
trait Image {
    fn display(&self);
}

// RealSubject: 실제로 무거운 작업을 수행하는 객체
struct RealImage {
    filename: String,
}

impl RealImage {
    fn new(filename: &str) -> Self {
        println!("{} 파일을 디스크에서 로드 중...", filename);
        RealImage {
            filename: filename.to_string(),
        }
    }
}

impl Image for RealImage {
    fn display(&self) {
        println!("{} 이미지 표시", self.filename);
    }
}

// Proxy: RealSubject에 대한 대리자
struct ProxyImage {
    filename: String,
    real_image: RefCell<Option<RealImage>>,
}

impl ProxyImage {
    fn new(filename: &str) -> Self {
        println!("{} 프록시 생성 (아직 로드하지 않음)", filename);
        ProxyImage {
            filename: filename.to_string(),
            real_image: RefCell::new(None),
        }
    }
}

impl Image for ProxyImage {
    fn display(&self) {
        // 실제 이미지가 아직 생성되지 않았다면 지금 생성
        if self.real_image.borrow().is_none() {
            *self.real_image.borrow_mut() = Some(RealImage::new(&self.filename));
        }

        // 실제 객체에 위임
        self.real_image.borrow().as_ref().unwrap().display();
    }
}

fn main() {
    let image: Box<dyn Image> = Box::new(ProxyImage::new("vacation.jpg"));

    println!("이미지가 아직 표시되지 않음");
    println!("이제 이미지 표시 시작");
    image.display();
    println!("다시 표시");
    image.display();
}
