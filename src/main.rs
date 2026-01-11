mod engine;
mod registries;
mod messages;
mod state;
mod results;
mod providers;
mod ui;
mod utils;

use iced::Size;

pub use messages::Message;
pub use state::DandanLauncher;
pub use results::ProviderResult;

pub use utils::PathUtils;
pub use utils::LauncherUtils;
pub use utils::IconsUtils;

fn main() -> iced::Result {
    iced::application(DandanLauncher::init, DandanLauncher::update, DandanLauncher::view)
        .title("dandan")
        .subscription(DandanLauncher::subscription)
        .resizable(false)
        .window_size(Size::from((600, 400)))
        .run()
}
