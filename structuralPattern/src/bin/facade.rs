//! # 퍼사드 패턴
//! 한 서브시스템 내의 인터페이스 집합에 대한 획일화된 하나의 인터페이스를 제공하는 패턴.
//! 복잡한 서브시스템(하위 객체)에 대해 단순화된 통합 인터페이스를 제공하는 구조 패턴이다.
//!
//! ## 사용
//! * 복잡한 서브시스템에 대한 단순한 인터페이스 제공이 필요할 때
//! * 서브시스템을 계층화시킬 때
//!
//! ## 구성
//! * 퍼사드(ComputerFacade): 단순하고 일관된 통합 인터페이스를 제공하며, 사용자의 요청을 알맞는 하위 객체로 보내는 역할을 한다.
//! * 서브시스템(Cpu, Memory, HardDrive, GraphicsCard): 실제 작업을 수행하는 여러 객체들. 퍼사드의 존재를 몰라도 된다.
//!
//! 복잡함을 단순화하는 것이 주 목적이다.
//! 가상의 컴퓨터를 켜기 위한 여러 작업들이 있는데, 우리의 목적은 컴퓨터를 키는 것이므로 그 목적을 수행하는 퍼사드를 만드는 것이다.
//! 퍼사드는 컴퓨터를 켜기 위한 여러 작업들을 실제로 호출하는 역할을 수행한다.

struct Cpu;

impl Cpu {
    fn freeze(&self) {
        println!("CPU: 동작 정지");
    }

    fn jump(&self, position: u64) {
        println!("CPU: 부팅 위치 0x{:X}로 점프", position);
    }

    fn execute(&self) {
        println!("CPU: 명령어 실행 시작");
    }
}

struct Memory;

impl Memory {
    fn load(&self, position: u64, data: &str) {
        println!("Memory: 위치 0x{:X}에 '{}' 적재", position, data);
    }
}

struct HardDrive;

impl HardDrive {
    fn read_boot_sector(&self) -> String {
        "부팅 섹터 데이터".to_string()
    }
}

struct GraphicsCard;

impl GraphicsCard {
    fn init(&self) {
        println!("GraphicsCard: 초기화 완료");
    }
}

// Facade
struct ComputerFacade {
    cpu: Cpu,
    memory: Memory,
    hard_drive: HardDrive,
    graphics_card: GraphicsCard,
}

impl ComputerFacade {
    fn new() -> Self {
        ComputerFacade {
            cpu: Cpu,
            memory: Memory,
            hard_drive: HardDrive,
            graphics_card: GraphicsCard,
        }
    }

    fn power_on(&self) {
        println!("컴퓨터 전원 ON");

        self.cpu.freeze();
        let boot_data = self.hard_drive.read_boot_sector();
        self.memory.load(0x0000, &boot_data);
        self.cpu.jump(0x0000);
        self.cpu.execute();
        self.graphics_card.init();

        println!("컴퓨터 사용 준비 완료");
    }
}

fn main() {
    let computer = ComputerFacade::new();
    computer.power_on();
}
