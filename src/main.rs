use sdl2::messagebox::{
    show_message_box, ButtonData, ClickedButton, MessageBoxButtonFlag, MessageBoxColorScheme,
    MessageBoxFlag,
};

#[must_use]
pub fn confer_with_user(
    kind: MessageBoxFlag,
    title: &str,
    message: &str,
    window: &sdl2::video::Window,
    cancel_button_name: &str,
    confirmation_button_name: &str,
) -> bool {
    let cancel_button = ButtonData {
        flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
        button_id: 0,
        text: cancel_button_name,
    };
    let save_button = ButtonData {
        flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
        button_id: 1,
        text: confirmation_button_name,
    };
    let buttons = [save_button, cancel_button];
    let color_scheme = MessageBoxColorScheme {
        background: (255, 255, 255),
        text: (0, 0, 0),
        button_border: (255, 255, 0),
        button_background: (198, 198, 198),
        button_selected: (0, 0, 0),
    };
    let message_box_answer = show_message_box(kind, &buttons, title, message, window, color_scheme)
        .expect("Displaying a fancy message box failed.");

    match message_box_answer {
        ClickedButton::CustomButton(b) if b.button_id == buttons[0].button_id => true,
        ClickedButton::CustomButton(_) | ClickedButton::CloseButton => false,
    }
}
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("APP NAME", 400, 400)
        .position_centered()
        .build()
        .unwrap();
    if confer_with_user(
        MessageBoxFlag::WARNING,
        "Are You Sure?",
        "Are you sure you want this dialog to look like this?",
        &window,
        "Cancel",
        "Confirm",
    ) {
        println!("User hit Confirm.");
    } else {
        println!("User hit Cancel.");
    }
}
