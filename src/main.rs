// Gui components
use iced::{button, Application, Button, Column, Command, Element, Settings, Text};

// And of course the CDDA stuff itself
use cdda_editor::CDDASave;

// This thing's just a POC right now, so stuff is hard coded.
// Don't judge me!
fn main() { 
    CDDAEditor::run(Settings::default())
}

// A Struct to implement Application on that holds the GUI state
#[derive(Default)]
struct CDDAEditor {
    savefile: CDDASave,
    loaded: bool,
    some_state: String,
    a_button: button::State,
}

// An enum defining messages that can be passed from widgets
#[derive(Debug, Clone, Copy)]
enum Message {
    Pressed,
}

// Impl the Application trait
impl Application for CDDAEditor {
    type Message = Message;
    
    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }
    
    fn title(&self) -> String {
        String::from("CDDA Save File Editor")
    }
    
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Pressed => {
                if self.loaded == true { return Command::none(); }
                let loaded = match CDDASave::from_file("test.sav") {
                    Ok(x) => x,
                    Err(_e) => { panic!("Error loading save"); }
                };
                self.savefile = loaded;
                self.loaded = true;
                self.some_state = "Save Loaded".to_string();
            }
        }
        
        Command::none()
    }
    
    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Button::new(&mut self.a_button, Text::new("Load Save File"))
                    .on_press(Message::Pressed),
            )
            .push(
                Text::new(self.some_state.to_string()).size(50),
            )
            .into()
    }
}
