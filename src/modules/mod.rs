pub mod clock;
pub mod sysinfo;
pub mod window;
pub mod workspaces;

use gtk::glib::IsA;
use gtk::Widget;

pub trait Module<W>
where
    W: IsA<Widget>,
{
    fn into_widget(self) -> W;
}
