use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use server_fn::codec::{MultipartData, MultipartFormData};

use web_sys::{FormData, HtmlFormElement, SubmitEvent};
use wasm_bindgen::JsCast;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/avatar-generator.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="container">
            <Header/>
            <Sidebar/>
            <div class="main">
                <h1>Welcome to Leptos!!</h1>
                <button on:click=on_click>Click Me:  {count}</button>
            </div>
            <Footer/>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <h1>Avatar Creator</h1>
            <h2>Create your own avatar using AI</h2>
            <ProfileMenu/>
        </header>
    }
}

#[component]
fn ProfileMenu() -> impl IntoView {
    view! {
        <nav>
            <details class="profile">
                <summary>"👤"</summary>
                <ul class="menu">
                    <li><a href="/profile">Profile</a></li>
                    <li><a href="/settings">Logout</a></li>
                </ul>
            </details>
        </nav>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <aside>
            <p>Leptos</p>
            <FileUpload/>
        </aside>
    }
}


#[component]
pub fn FileUpload() -> impl IntoView {
    /// A simple file upload function, which does just returns the length of the file.
    ///
    /// On the server, this uses the `multer` crate, which provides a streaming API.
    #[server(
        input = MultipartFormData,
    )]
    pub async fn file_length(
        data: MultipartData,
    ) -> Result<usize, ServerFnError> {
        // `.into_inner()` returns the inner `multer` stream
        // it is `None` if we call this on the client, but always `Some(_)` on the server, so is safe to
        // unwrap
        let mut data = data.into_inner().unwrap();

        // this will just measure the total number of bytes uploaded
        let mut count = 0;
        while let Ok(Some(mut field)) = data.next_field().await {
            println!("\n[NEXT FIELD]\n");
            let name = field.name().unwrap_or_default().to_string();
            println!("  [NAME] {name}");
            while let Ok(Some(chunk)) = field.chunk().await {
                let len = chunk.len();
                count += len;
                println!("      [CHUNK] {len}");
                // in a real server function, you'd do something like saving the file here
            }
        }

        Ok(count)
    }

    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        // `MultipartData` implements `From<FormData>`
        file_length(data.into())
    });

    view! {
        <h3>File Upload</h3>
        <p>Uploading files is fairly easy using multipart form data.</p>
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch(form_data);
        }>
            <input class="file_upload" type="file" name="file_to_upload"/>
            <input type="submit"/>
        </form>
        <p>
            {move || if upload_action.input().get().is_none() && upload_action.value().get().is_none() {
                "Upload a file.".to_string()
            } else if upload_action.pending().get() {
                "Uploading...".to_string()
            } else if let Some(Ok(value)) = upload_action.value().get() {
                value.to_string()
            } else {
                format!("{:?}", upload_action.value().get())
            }}
        </p>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <p>Leptos</p>
        </footer>
    }
}
