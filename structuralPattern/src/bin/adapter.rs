//! # 적응자 패턴
//!
//! 인터페이스를 사용자가 기대하는 인터페이스 형태로 적응시킨다.
//! 적응자의 또다른 이름은 Wrapper인데, 적응자에 해당하는 인스턴스 연산을 호출하고 적응자는
//! 해당 요청을 수행하기 위해 적응 대상자의 연산을 호출한다는 점을 고려하면 적절한 이름이다.
//!
//! ## 사용
//!
//! * 기존 클래스를 사용하고 싶은데 인터페이스가 맞지 않을 때
//! * 이미 만든 것을 재사용하고자 하나 이 재사용 가능한 라이브러리를 수정할 수 없을 때
//!
//! ## 구성
//!
//! - **Target** (`MediaPlayer`): 사용자가 기대하고 쓰는 실제 인터페이스
//! - **Adaptee** (`AdvancedMediaPlayer`): 이미 존재하는 레거시? 인터페이스
//! - **Adapter** (`MediaAdapter`): Adaptee를 Target으로 변환하는 역할 수행
//!     - Adapter는 Target과 Adaptee를 구현함으로써 변환 역할을 수행할 수 있다.
//! - **Client** (`AudioPlayer`): Target을 쓰는 구현체

// Target 인터페이스: 클라이언트가 기대하는 인터페이스
trait MediaPlayer {
    fn play(&self, audio_type: &str, file_name: &str);
}

// Adaptee: 이미 존재하며 인터페이스가 호환되지 않는 클래스/트레이트
trait AdvancedMediaPlayer {
    fn play_vlc(&self, file_name: &str);
    fn play_mp4(&self, file_name: &str);
}

struct VlcPlayer;
impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self, file_name: &str) {
        println!("VLC 재생 중: {}", file_name);
    }
    fn play_mp4(&self, _file_name: &str) {
        // VLC는 MP4를 지원하지 않음
    }
}

struct Mp4Player;
impl AdvancedMediaPlayer for Mp4Player {
    fn play_vlc(&self, _file_name: &str) {
        // MP4 플레이어는 VLC를 지원하지 않음
    }
    fn play_mp4(&self, file_name: &str) {
        println!("MP4 재생 중: {}", file_name);
    }
}

// Adapter: MediaPlayer 인터페이스를 AdvancedMediaPlayer 구현체에 연결
struct MediaAdapter {
    advanced_player: Box<dyn AdvancedMediaPlayer>,
}

impl MediaAdapter {
    fn new(audio_type: &str) -> Option<Self> {
        match audio_type {
            "vlc" => Some(Self {
                advanced_player: Box::new(VlcPlayer),
            }),
            "mp4" => Some(Self {
                advanced_player: Box::new(Mp4Player),
            }),
            _ => None,
        }
    }
}

impl MediaPlayer for MediaAdapter {
    fn play(&self, audio_type: &str, file_name: &str) {
        match audio_type {
            "vlc" => self.advanced_player.play_vlc(file_name),
            "mp4" => self.advanced_player.play_mp4(file_name),
            _ => println!("지원하지 않는 형식: {}", audio_type),
        }
    }
}

// Client: MediaPlayer 인터페이스만 사용
struct AudioPlayer;

impl MediaPlayer for AudioPlayer {
    fn play(&self, audio_type: &str, file_name: &str) {
        if audio_type == "mp3" {
            println!("MP3 직접 재생 중: {}", file_name);
        } else if let Some(adapter) = MediaAdapter::new(audio_type) {
            // Adapter를 통해 vlc/mp4도 MediaPlayer 인터페이스로 재생
            adapter.play(audio_type, file_name);
        } else {
            println!("오류: '{}' 형식은 재생할 수 없습니다.", audio_type);
        }
    }
}

fn main() {
    let player = AudioPlayer;

    player.play("mp3", "song.mp3");
    player.play("mp4", "movie.mp4");
    player.play("vlc", "concert.vlc");
    player.play("avi", "clip.avi");
}
