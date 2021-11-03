use core::cell::UnsafeCell;
use core::marker::PhantomData;
use objc::rc::{AutoreleasePool, Id, Owned, Shared};
use objc::runtime::{Class, Object, BOOL, NO, YES};
use objc::{class, msg_send, sel};

use super::menu::Menu;
use super::menubar::MenuBar;

/// Helper to make various functions on the global application object safe.
#[doc(alias = "NSApp")]
#[doc(alias = "NSApplication")]
#[repr(C)]
pub struct InitializedApplication {
    /// The application contains syncronization primitives that allows mutable
    /// access with an immutable reference, and hence need to be `UnsafeCell`.
    ///
    /// TODO: Verify this claim.
    _priv: UnsafeCell<[u8; 0]>,
}

unsafe impl objc::RefEncode for InitializedApplication {
    const ENCODING_REF: objc::Encoding<'static> = objc::Encoding::Object;
}

unsafe impl objc::Message for InitializedApplication {}
unsafe impl Sync for InitializedApplication {}

impl InitializedApplication {
    /// # Safety
    ///
    /// This must not be called before `applicationDidFinishLaunching`.
    ///
    /// In `winit`, this is at or after
    /// [`winit::event::StartCause::Init`] has been emitted.
    #[doc(alias = "sharedApplication")]
    pub unsafe fn new() -> &'static Self {
        msg_send![class!(NSApplication), sharedApplication]
    }

    #[doc(alias = "mainMenu")]
    pub fn menubar<'p>(&self, pool: &'p AutoreleasePool) -> Option<&'p Menu> {
        unsafe { msg_send![self, mainMenu] }
    }

    /// Setting the menubar to `null` does not work properly, so we don't allow
    /// that functionality here!
    #[doc(alias = "setMainMenu")]
    #[doc(alias = "setMainMenu:")]
    pub fn set_menubar(&self, menubar: MenuBar) -> Id<Menu, Shared> {
        let menu = menubar.into_raw();
        let _: () = unsafe { msg_send![self, setMainMenu: &*menu] };
        menu.into()
    }

    /// Returns the first menu set with [`set_window_menu`]
    #[doc(alias = "windowsMenu")]
    pub fn window_menu<'p>(&self, pool: &'p AutoreleasePool) -> Option<&'p Menu> {
        unsafe { msg_send![self, windowsMenu] }
    }

    /// Set the global window menu.
    ///
    /// The "Window: menu has items and keyboard shortcuts for entering
    /// fullscreen, managing tabs (e.g. "Show Next Tab") and a list of the
    /// application's windows.
    ///
    /// Should be called before [`set_menubar`], otherwise the window menu
    /// won't be properly populated.
    ///
    /// Un-setting the window menu (to `null`) does not work properly, so we
    /// don't expose that functionality here.
    ///
    /// Additionally, you can have luck setting the window menu more than once,
    /// though this is not recommended.
    #[doc(alias = "setWindowsMenu")]
    #[doc(alias = "setWindowsMenu:")]
    pub fn set_window_menu(&self, menu: &Menu) {
        // TODO: Is it safe to immutably set this?
        unsafe { msg_send![self, setWindowsMenu: menu] }
    }

    /// Returns the first menu set with [`set_services_menu`]
    #[doc(alias = "servicesMenu")]
    pub fn services_menu<'p>(&self, pool: &'p AutoreleasePool) -> Option<&'p Menu> {
        unsafe { msg_send![self, servicesMenu] }
    }

    /// Set the global services menu.
    ///
    /// The user can have a number of system configured services and
    /// corresponding keyboard shortcuts that can be accessed from this menu.
    ///
    /// Un-setting the services menu (to `null`) does not work properly, so we
    /// don't expose that functionality here.
    ///
    /// Additionally, you can sometimes have luck setting the services menu
    /// more than once, but this is really flaky.
    #[doc(alias = "setServicesMenu")]
    #[doc(alias = "setServicesMenu:")]
    pub fn set_services_menu(&self, menu: &Menu) {
        // TODO: Is it safe to immutably set this?
        // TODO: The menu should (must?) not contain any items!
        // TODO: Setting this and pressing the close button doesn't work in winit
        unsafe { msg_send![self, setServicesMenu: menu] }
    }

    // TODO: registerServicesMenuSendTypes

    /// Get the menu that is currently assigned as the help menu, or `None` if the system is configured to autodetect this.
    #[doc(alias = "helpMenu")]
    pub fn help_menu<'p>(&self, pool: &'p AutoreleasePool) -> Option<&'p Menu> {
        unsafe { msg_send![self, helpMenu] }
    }

    /// Set the global menu that should have the spotlight Help Search
    /// functionality at the top of it.
    ///
    /// If this is set to `None`, the system will place the search bar somewhere
    /// else, usually on an item named "Help" (unknown if localization applies).
    /// To prevent this, specify a menu that does not appear anywhere.
    #[doc(alias = "setHelpMenu")]
    #[doc(alias = "setHelpMenu:")]
    pub fn set_help_menu(&self, menu: Option<&Menu>) {
        // TODO: Is it safe to immutably set this?
        unsafe { msg_send![self, setHelpMenu: menu] }
    }

    // TODO: applicationDockMenu (the application delegate should implement this function)

    #[doc(alias = "menuBarVisible")]
    pub fn menubar_visible(&self) -> bool {
        let visible: BOOL = unsafe { msg_send![class!(NSMenu), menuBarVisible] };
        visible != NO
    }

    /// Hide or show the menubar for the entire application.
    /// This also hides or shows the yellow minimize button.
    ///
    /// Might silently fail to set the menubar visible if in fullscreen mode or similar.
    #[doc(alias = "setMenuBarVisible")]
    #[doc(alias = "setMenuBarVisible:")]
    pub fn set_menubar_visible(&self, visible: bool) {
        let visible: BOOL = if visible { YES } else { NO };
        unsafe { msg_send![class!(NSMenu), setMenuBarVisible: visible] }
    }

    // Only available on the global menu bar object
    // #[doc(alias = "menuBarHeight")]
    // pub fn global_height(&self) -> f64 {
    //     let height: CGFloat = unsafe { msg_send![self, menuBarHeight] };
    //     height
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use objc::rc::autoreleasepool;

    fn init_app() -> &'static InitializedApplication {
        unimplemented!()
    }

    fn create_menu() -> Id<Menu, Owned> {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn test_services_menu() {
        let app = init_app();
        let menu1 = create_menu();
        let menu2 = create_menu();

        autoreleasepool(|pool| {
            assert!(app.services_menu(pool).is_none());

            app.set_services_menu(&menu1);
            assert_eq!(app.services_menu(pool).unwrap(), &*menu1);

            app.set_services_menu(&menu2);
            assert_eq!(app.services_menu(pool).unwrap(), &*menu2);

            // At this point `menu1` still shows as a services menu...
        });
    }
}
