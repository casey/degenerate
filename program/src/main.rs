use degenerate::{Event, Field, Filter, Message, System};

fn main() {
  System::execute(|system: &System, event: Event| {
    if let Event::Frame(timestamp) = event {
      system.send(Message::Clear);
      system.send(Message::Render(Filter {
        field: Field::X,
        alpha: (timestamp / 5000.0).min(1.0),
        ..Filter::default()
      }));
    }
  });
}
