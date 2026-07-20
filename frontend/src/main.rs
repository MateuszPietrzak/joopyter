use gloo_net::http::Request;
use serde::Deserialize;
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
            </>
        }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
