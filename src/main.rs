mod messages;
mod state;

use iced::Size;

pub use messages::Message;
pub use state::DandanLauncher;

fn main() -> iced::Result {
    iced::application(DandanLauncher::init, DandanLauncher::update, DandanLauncher::view)
        .title("dandan")
        .subscription(DandanLauncher::subscription)
        .resizable(false)
        .window_size(Size::from((600, 400)))
        .run()
}
