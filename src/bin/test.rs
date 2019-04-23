use event_sourcing::*;

struct Post;

#[derive(Debug)]
struct PostData {
    title: String,
    body: Option<String>,
    generation: u64,
}

impl PostData {
    fn new(title: &str, body: Option<String>) -> Self {
        Self {
            title: String::from(title),
            body: body,
            generation: 1,
        }
    }
}

enum PostCommand {
    UpdateTitle(String),
}

#[derive(Clone)]
enum PostEvent {
    TitleUpdated(String),
}

impl Event for PostEvent {
    fn event_type(&self) -> &str {
        match self {
            PostEvent::TitleUpdated(_) => "title_updated",
        }
    }
}

impl AggregateState for PostData {
    fn generation(&self) -> u64 {
        self.generation
    }
}

impl Aggregate for Post {
    type State = PostData;
    type Command = PostCommand;
    type Event = PostEvent;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        let new_state = match evt {
            PostEvent::TitleUpdated(new_title) => PostData {
                title: new_title,
                body: state.body.clone(),
                generation: state.generation + 1,
            },
        };
        Ok(new_state)
    }

    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        let mut events = vec![];
        match cmd {
            PostCommand::UpdateTitle(title) => events.push(PostEvent::TitleUpdated(title)),
        }
        Ok(events)
    }
}

fn main() {
    let post_data = PostData::new("title", None);

    println!("{:?}", post_data);

    let cmd = PostCommand::UpdateTitle(String::from("the new title"));

    let events = Post::handle_command(&post_data, cmd).unwrap();
    let new_post_data = events.into_iter().fold(post_data, |acc, event| {
        Post::apply_event(&acc, event).unwrap()
    });

    println!("{:?}", new_post_data);
}
