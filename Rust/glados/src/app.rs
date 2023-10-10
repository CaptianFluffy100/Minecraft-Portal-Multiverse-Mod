use crate::{error_template::{AppError, ErrorTemplate}, structs::{cob::GLaDOSError, portal::PortalVec}, pages::{server_list::page::ServerPage, portal_list::page::PortalPage, server_edit::page::ServerPageEdit, portal_edit::page::PortalPageEdit}};
use leptos::{*, html::Tr};
use leptos_meta::*;
use leptos_router::*;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use stylers::style;

use crate::structs::server::ServerVec;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <html class="h-full" style="height: 100%;">
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            // <Stylesheet id="leptos" href="/pkg/glados.css"/>
            <Stylesheet id="tailwind" href="https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css"/>
            <Stylesheet id="daisyui" href="https://cdn.jsdelivr.net/npm/daisyui@3.9.2/dist/full.css"/>

            <Script id="htmx" src="https://unpkg.com/htmx.org@1.9.6"/>

            // sets the document title
            <Title text="Welcome to GLaDOS"/>

            // content for this welcome page
            <body class="h-full" style="height: 100%;">
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! {
                        <ErrorTemplate outside_errors/>
                    }
                    .into_view()
                }>  
                    <main onload="get_server_list" class="flex max-h-screen flex-col items-center justify-between" style="height: 100%;">
                        <Routes>
                            <Route path="" view=HomePage/>
                            <Route path="/servers" view=ServerPage/>
                            <Route path="/portals" view=PortalPage/>
                            <Route path="/server/list" view=ServerPageEdit/>
                            <Route path="/portal/list" view=PortalPageEdit/>
                        </Routes>
                    </main>
                </Router>
            </body>
        </html>    
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <div class="navbar bg-base-100 h-full" style="height: 100%;">
            <div class="drawer h-full" style="height: 100%;">
              <input id="my-drawer" type="checkbox" class="drawer-toggle" />
              <div class="drawer-content" style="height: 100%;">
                // <div inner-html={page_data}/>
                <label for="my-drawer" class="btn btn-ghost drawer-button">GLaDOS</label>
                // {ServerPageDyn}
              </div> 
              {PopulateSideBar}
            </div>
        </div>
    }
}

pub async fn get_servers() -> Result<ServerVec> {
    let res = reqwasm::http::Request::get(&format!(
        "/api/servers",
    ))
    .send()
    .await?
    .json::<ServerVec>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

pub async fn get_portals() -> Result<PortalVec> {
    let res = reqwasm::http::Request::get(&format!(
        "/api/portals",
    ))
    .send()
    .await?
    .json::<PortalVec>()
    .await?;
    Ok(res)
    // Err(GLaDOSError::ERROR.into())
}

#[component]
pub fn PopulateSideBar() -> impl IntoView {

    view! {
        <div class="drawer-side h-full" style="height: 100%;">
          <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
          <ul class="menu p-4 w-80 h-full bg-base-200 text-base-content" style="height: 100%;">
            <li><a href="/">Home Page</a></li>
            <li class="w-full text-center underline align-center content-center font-black pt-6">List</li>
            <li><a href="/servers">Server List</a></li>
            <li><a href="/portals">Portal List</a></li>
            <li class="w-full text-center underline align-center content-center font-black pt-6">Add/Remove/Edit</li>
            <li><a href="/server/list">Servers</a></li>
            <li><a href="/portal/list">Portals</a></li>
          </ul>
        </div>
    }
}

