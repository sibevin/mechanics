use super::*;

const PAGE_CODE: &str = "loading";
const PAGE_NAME: &str = "Loading";
const PAGE_ICON: &str = "wrench";

pub struct Page;

impl PageBase for Page {
    fn code(&self) -> &str {
        PAGE_CODE
    }
    fn name(&self) -> &str {
        PAGE_NAME
    }
    fn icon(&self) -> &str {
        PAGE_ICON
    }
    fn state(&self) -> PageState {
        PageState::Loading
    }
    fn build(&self, _app: &mut App) {}
}
