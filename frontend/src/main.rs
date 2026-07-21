use futures::StreamExt;
use gloo_net::http::Request;
use gloo_net::websocket::Message;
use gloo_net::websocket::futures::WebSocket;
use monaco::api::TextModel;
use monaco::yew::CodeEditor;
use serde::Deserialize;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future;

#[derive(Properties, PartialEq)]
struct VideosFetchProps {
    on_click: Callback<Video>,
    selected_video: Option<Video>,
}

#[component]
fn VideosFetcher(
    VideosFetchProps {
        on_click,
        selected_video,
    }: &VideosFetchProps,
) -> HtmlResult {
    let videos = use_future(|| async {
        Request::get("/api/data")
            .send()
            .await?
            .json::<Vec<Video>>()
            .await
    })?;

    match &*videos {
        Ok(videos) => Ok(html! {
            <>
                <div>
                    <h3>{ "Videos to watch" }</h3>
                    <VideosList videos={videos.clone()} on_click={on_click.clone()} />
                </div>
                if let Some(video) = selected_video {
                    <VideoDetails video={video.clone()} />
                }
            </>
        }),
        Err(err) => Ok(html! {
            <p>{format!("Error fetching videos: {err}")}</p>
        }),
    }
}

#[derive(Clone, PartialEq, Deserialize)]
struct StdOutData {
    timestamp: u64,
    data: AttrValue,
}

#[derive(Clone, Default, PartialEq)]
struct StdOutState {
    messages: Vec<StdOutData>,
}

enum StdOutAction {
    Append(StdOutData),
}

impl Reducible for StdOutState {
    type Action = StdOutAction;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            StdOutAction::Append(data) => {
                let state = Rc::make_mut(&mut self);
                state.messages.push(data);
            }
        };
        self
    }
}

#[component]
fn StdOutTerminal() -> Html {
    let messages = use_reducer(StdOutState::default);
    {
        let messages = messages.clone();
        use_effect_with((), move |_| {
            let mut ws = WebSocket::open("/api/stream").expect("Failed to connect");

            spawn_local(async move {
                while let Some(msg) = ws.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        if let Ok(item) = serde_json::from_str::<StdOutData>(&text) {
                            messages.dispatch(StdOutAction::Append(item));
                        }
                    }
                }
            });

            || ()
        });
    }
    html! {
        <>
            <h1>{"Std Out Stream"}</h1>
            <ul>
            for message in &messages.messages {
                <li>{format!("{}: {}", message.timestamp, message.data)}</li>
            }
            </ul>
        </>
    }
}

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: AttrValue,
    speaker: AttrValue,
    url: AttrValue,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[component]
fn VideosList(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_select = |video: &Video| {
        let on_click = on_click.clone();
        let video = video.clone();
        Callback::from(move |_| on_click.emit(video.clone()))
    };

    html! {
        for video in videos {
            <p key={video.id} onclick={on_select(video)}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[component]
fn VideoDetails(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ &*video.title }</h3>
            <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CodeCellProps {
    text_model: TextModel,
}

#[component]
fn CodeCell(CodeCellProps { text_model }: &CodeCellProps) -> Html {
    let contents = use_state_eq(String::new);
    let onclick = {
        let contents = contents.clone();
        let model = text_model.clone();
        Callback::from(move |_| {
            contents.set(model.get_value());
        })
    };
    html! {
        <>
            <CodeEditor classes={"code-cell"} model={text_model.clone()}/>
            <button {onclick}>{"Output"}</button>
            <p>{(*contents).clone()}</p>
        </>
    }
}

#[component]
fn App() -> Html {
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let text_model = use_state_eq(|| {
        TextModel::create("print('Hello, World!')\n1 + 1", Some("python"), None).unwrap()
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <Suspense fallback={html! {<p>{"Loading..."} </p>}} >
                <VideosFetcher
                    on_click={on_video_select}
                   selected_video={(*selected_video).clone()}
               />
            </Suspense>
            <CodeCell text_model={(*text_model).clone()}/>
            <StdOutTerminal/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
