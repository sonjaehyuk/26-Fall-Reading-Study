//! # 복합체
//!
//! 부분과 전체의 계층을 표현하기 위해 객체들을 모아 트리 구조로 구성한다.
//! 사용자로 하여금 개별 객체와 복합 객체를 모두 동일하게 다룰 수 있도록 하는 패턴이다.
//! 쉽게 말해 배열의 원소가 몇 개이든(1개이든 다수이든) 동일하게 다룰 수 있도록 한다는 것이다.
//!
//! ## 사용
//! * 사용자가 객체의 합성으로 생긴 복합 객체와 개개의 객체 사이의 차이를 알지 않고도 자기 일을 할 수 있도록 만들고 싶을 때.
//!
//! ## 구성
//!
//! * Component(FileSystemComponent): 집합 관계에 정의될 모든 객체에 대한 인터페이스를 정의한다.
//! * Leaf(File): 가장 말단의 객체. 사실상 원본 객체를 뜻하는 것이라고 볼 수 있다.
//! * Composite(Folder): 자식이 있는 구성요소에 대한 행동을 정의한다.
//!
//! File이든 Folder이든 상관없이 동일한 방식으로 처리할 수 있다.

// Component: Leaf와 Composite가 공유하는 공통 인터페이스
trait FileSystemComponent {
    fn get_size(&self) -> usize;
    fn display(&self, indent: usize);
}

// Leaf: 트리 구조의 말단 개체
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        File {
            name: name.to_string(),
            size,
        }
    }
}

impl FileSystemComponent for File {
    fn get_size(&self) -> usize {
        self.size
    }

    fn display(&self, indent: usize) {
        println!("{}[파일] {} ({} bytes)", "  ".repeat(indent), self.name, self.size);
    }
}

// Composite: 여러 Component를 포함하는 그룹
struct Folder {
    name: String,
    children: Vec<Box<dyn FileSystemComponent>>,
}

impl Folder {
    fn new(name: &str) -> Self {
        Folder {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    fn add(&mut self, component: Box<dyn FileSystemComponent>) {
        self.children.push(component);
    }
}

impl FileSystemComponent for Folder {
    fn get_size(&self) -> usize {
        self.children.iter().map(|c| c.get_size()).sum()
    }

    fn display(&self, indent: usize) {
        println!("{}[폴더] {}", "  ".repeat(indent), self.name);
        for child in &self.children {
            child.display(indent + 1);
        }
    }
}

fn main() {
    let mut root = Folder::new("root");

    let file1 = Box::new(File::new("a.txt", 100));
    let file2 = Box::new(File::new("b.txt", 200));

    let mut sub_folder = Folder::new("sub");
    let file3 = Box::new(File::new("c.txt", 300));

    sub_folder.add(file3);

    root.add(file1);
    root.add(file2);
    root.add(Box::new(sub_folder));

    root.display(0);
    println!("전체 크기: {} bytes", root.get_size());
}
