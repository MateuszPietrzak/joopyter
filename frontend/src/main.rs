use gloo_net::http::Request;
use gloo_net::websocket::Message;
use gloo_net::websocket::futures::WebSocket;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future;
use futures::StreamExt;

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

#[component]
fn StdOutTerminal() -> Html {
    let messages = use_state(Vec::<StdOutData>::new);
    {
        let messages = messages.clone();
        use_effect_with((), move |_| {
            let mut ws = WebSocket::open("/api/stream").expect("Failed to connect");

            spawn_local(async move {
                while let Some(msg) = ws.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        if let Ok(item) = serde_json::from_str::<StdOutData>(&text) {
                            messages.set({
                                let mut current = (*messages).clone();
                                current.push(item);
                                current
                            });
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
            for message in &*messages {
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

#[component]
fn App() -> Html {
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <Suspense fallback={html! {<p>{"Loading..."} </p>}} >
                <VideosFetcher
                    on_click={on_video_select}
                   selected_video={(*selected_video).clone()}
               />
            </Suspense>
            <StdOutTerminal/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
