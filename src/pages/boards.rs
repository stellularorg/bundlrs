use actix_web::HttpRequest;
use actix_web::{get, web, HttpResponse, Responder};

use yew::prelude::*;
use yew::ServerRenderer;

use crate::components::navigation::Footer;
use crate::db::bundlesdb::{Board, BoardMetadata, BoardPostLog, Log, UserState};
use crate::db::{self, bundlesdb};
use crate::utility::{self, format_html};

#[derive(Default, Properties, PartialEq, serde::Deserialize)]
struct NewProps {
    pub auth_state: Option<bool>,
}

#[derive(Default, Properties, PartialEq, serde::Deserialize)]
struct Props {
    pub board: Board<String>,
    pub posts: Vec<Log>,
    pub auth_state: Option<bool>,
}

#[derive(Default, Properties, PartialEq, serde::Deserialize)]
struct ViewPostProps {
    pub board: Board<String>,
    pub post: Log,
    pub auth_state: Option<bool>,
    pub user: Option<UserState>,
}

#[derive(Default, Properties, PartialEq, serde::Deserialize)]
struct SettingsProps {
    pub board: Board<String>,
    pub auth_state: Option<bool>,
}

#[function_component]
fn CreateNew(props: &NewProps) -> Html {
    return html! {
        <div class="flex flex-column g-4" style="height: 100dvh;">
            <main class="small flex flex-column g-4 align-center">
                <div class="card secondary round border" style="width: 25rem;" id="forms">
                    <div id="error" class="mdnote note-error full" style="display: none;" />
                    <form class="full flex flex-column g-4" action="/api/board/new" id="create-board">
                        <label for="_name"><b>{"Name"}</b></label>

                        <input
                            type="text"
                            name="_name"
                            id="_name"
                            placeholder="Name"
                            class="full round"
                            minlength={4}
                            maxlength={32}
                            required={true}
                        />

                        <hr />

                        <button class="bundles-primary full round">
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-plus"><path d="M5 12h14"/><path d="M12 5v14"/></svg>
                            {"Create Board"}
                        </button>
                    </form>
                </div>

                <script type="module">
                    {"import \"/static/js/NewBoard.js\";"}
                </script>

                <Footer auth_state={props.auth_state} />
            </main>
        </div>
    };
}

fn build_new_renderer_with_props(props: NewProps) -> ServerRenderer<CreateNew> {
    ServerRenderer::<CreateNew>::with_props(|| props)
}

#[get("/b/new")]
/// Available at "/b/new"
pub async fn new_request(
    req: HttpRequest,
    data: web::Data<db::bundlesdb::AppData>,
) -> impl Responder {
    // verify auth status
    let token_cookie = req.cookie("__Secure-Token");
    let mut set_cookie: &str = "";

    let token_user = if token_cookie.is_some() {
        Option::Some(
            data.db
                .get_user_by_hashed(token_cookie.as_ref().unwrap().value().to_string()) // if the user is returned, that means the ID is valid
                .await,
        )
    } else {
        Option::None
    };

    if token_user.is_some() {
        // make sure user exists, refresh token if not
        if token_user.as_ref().unwrap().success == false {
            set_cookie = "__Secure-Token=refresh; SameSite=Strict; Secure; Path=/; HostOnly=true; HttpOnly=true; Max-Age=0";
        }
    } else {
        // you must have an account to create boards
        return HttpResponse::NotFound().body(
            "You must have an account to create a board.
You can login at: /d/auth/login
You can create an account at: /d/auth/register",
        );
    }

    // ...
    let renderer = build_new_renderer_with_props(NewProps {
        auth_state: if req.cookie("__Secure-Token").is_some() {
            Option::Some(true)
        } else {
            Option::Some(false)
        },
    });

    return HttpResponse::Ok()
        .append_header(("Set-Cookie", set_cookie))
        .append_header(("Content-Type", "text/html"))
        .body(format_html(
            renderer.render().await,
            "<title>New Board - ::SITE_NAME::</title>",
        ));
}

#[function_component]
fn ViewBoard(props: &Props) -> Html {
    let board_m = serde_json::from_str::<BoardMetadata>(&props.board.metadata).unwrap();

    let can_post_from_anonymous =
        board_m.allow_anonymous.is_none() || board_m.allow_anonymous.unwrap() != String::from("no");

    // ...
    return html! {
        <div class="flex flex-column g-4" style="height: 100dvh;">
            <div style="display: none;" id="board-name">{&props.board.name}</div>

            <div class="toolbar flex justify-space-between">
                // left
                <div class="flex">
                    <a class="button" href="/" style="border-left: 0">
                        <b>{"::SITE_NAME::"}</b>
                    </a>

                    <a class="button" href={format!("/b/{}", props.board.name)} style="border-left: 0">
                        {props.board.name.clone()}
                    </a>
                </div>

                // right
                <div class="flex">
                    <a class="button" href={format!("/b/{}/manage", props.board.name)} title="Manage Board">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-file-cog"><path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/><circle cx="6" cy="14" r="3"/><path d="M6 10v1"/><path d="M6 17v1"/><path d="M10 14H9"/><path d="M3 14H2"/><path d="m9 11-.88.88"/><path d="M3.88 16.12 3 17"/><path d="m9 17-.88-.88"/><path d="M3.88 11.88 3 11"/></svg>
                    </a>
                </div>
            </div>

            <div class="toolbar-layout-wrapper">
                <main class="small flex flex-column g-4 align-center">
                    <div class="full" id="about">
                        {if board_m.about.is_some() {
                            let content = Html::from_html_unchecked(AttrValue::from(
                                crate::markdown::render::parse_markdown(&board_m.about.unwrap())
                            ));

                            html! {{content}}
                        } else {
                            html! {}
                        }}
                    </div>

                    {if (props.auth_state.is_some() && props.auth_state.unwrap() == true) || (can_post_from_anonymous == true) {
                        // ^ signed in OR can_post_from_anonymous is true
                        html! {
                            <div class="full">
                                <div class="card round secondary flex flex-column g-4" id="post">
                                    <div id="error" class="mdnote note-error full" style="display: none;" />

                                    <form id="create-post" class="flex flex-column g-4">
                                        <div class="full flex justify-space-between align-center g-4">
                                            <b>{"Create Post"}</b>

                                            <button class="bundles-primary round">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-plus"><path d="M5 12h14"/><path d="M12 5v14"/></svg>
                                                {"Send"}
                                            </button>
                                        </div>

                                        <textarea
                                            type="text"
                                            name="content"
                                            id="content"
                                            placeholder="Content"
                                            class="full round"
                                            minlength={2}
                                            maxlength={1_000}
                                            required={true}
                                        ></textarea>
                                    </form>
                                </div>

                                <hr style="var(--u-08) 0 var(--u-04) 0" />
                            </div>
                    }} else {
                        html! {}
                    }}

                    {for props.posts.iter().map(|p| {
                        let post = serde_json::from_str::<BoardPostLog>(&p.content).unwrap();
                        let content = Html::from_html_unchecked(AttrValue::from(post.content_html.clone()));

                        html! {
                            <div class="card secondary round full flex flex-column g-4">
                                <div class="flex justify-space-between align-center g-4">
                                    <span class="chip mention round" style="width: max-content;">
                                        {if post.author != "Anonymous" {
                                            html! {<a href={format!("/~{}", &post.author)} style="color: inherit;">{&post.author}</a>}
                                        } else {
                                            html! {<span>{"Anonymous"}</span>}
                                        }}
                                    </span>

                                    <div class="flex g-4 flex-wrap">
                                        <a
                                            class="button round"
                                            href={format!("/b/{}/posts/{}", post.board, p.id)}
                                            style="color: var(--text-color);"
                                            target="_blank"
                                            title="open/manage"
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-up-right-from-square"><path d="M21 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h6"/><path d="m21 3-9 9"/><path d="M15 3h6v6"/></svg>
                                        </a>
                                    </div>
                                </div>

                                <div>{content}</div>
                            </div>
                        }
                    })}

                    <script type="module">
                        {"import \"/static/js/BoardView.js\";"}
                    </script>

                    <Footer auth_state={props.auth_state} />
                </main>
            </div>
        </div>
    };
}

fn build_view_renderer_with_props(props: Props) -> ServerRenderer<ViewBoard> {
    ServerRenderer::<ViewBoard>::with_props(|| props)
}

#[get("/b/{name:.*}")]
/// Available at "/b/{name}"
pub async fn view_board_request(
    req: HttpRequest,
    data: web::Data<db::bundlesdb::AppData>,
) -> impl Responder {
    // verify auth status
    let token_cookie = req.cookie("__Secure-Token");
    let mut set_cookie: &str = "";

    let token_user = if token_cookie.is_some() {
        Option::Some(
            data.db
                .get_user_by_hashed(token_cookie.as_ref().unwrap().value().to_string()) // if the user is returned, that means the ID is valid
                .await,
        )
    } else {
        Option::None
    };

    if token_user.is_some() {
        // make sure user exists, refresh token if not
        if token_user.as_ref().unwrap().success == false {
            set_cookie = "__Secure-Token=refresh; SameSite=Strict; Secure; Path=/; HostOnly=true; HttpOnly=true; Max-Age=0";
        }
    }

    // get board
    let name: String = req.match_info().get("name").unwrap().to_string();

    let board: bundlesdb::DefaultReturn<Option<Board<String>>> =
        data.db.get_board_by_name(name.clone()).await;

    if board.success == false {
        let renderer = ServerRenderer::<crate::pages::errors::_404Page>::new();
        return HttpResponse::NotFound()
            .append_header(("Content-Type", "text/html"))
            .body(utility::format_html(
                renderer.render().await,
                "<title>404: Not Found</title>",
            ));
    }

    // check if board is private
    // if it is, only the owner and users with the "staff" role can view it
    let metadata =
        serde_json::from_str::<bundlesdb::BoardMetadata>(&board.payload.as_ref().unwrap().metadata)
            .unwrap();

    if metadata.is_private == String::from("yes") {
        // anonymous
        if token_user.is_none() {
            return HttpResponse::NotFound()
                .body("You do not have permission to view this paste's contents.");
        }

        // not owner
        let user = token_user.unwrap().payload.unwrap();

        if (user.username != metadata.owner) && (user.role != String::from("staff")) {
            return HttpResponse::NotFound()
                .body("You do not have permission to view this board's contents.");
        }
    }

    // ...
    let posts: bundlesdb::DefaultReturn<Option<Vec<Log>>> =
        data.db.get_board_posts(name.clone()).await;

    // ...
    let renderer = build_view_renderer_with_props(Props {
        board: board.payload.unwrap(),
        posts: posts.payload.unwrap(),
        auth_state: if req.cookie("__Secure-Token").is_some() {
            Option::Some(true)
        } else {
            Option::Some(false)
        },
    });

    let render = renderer.render();
    return HttpResponse::Ok()
        .append_header(("Set-Cookie", set_cookie))
        .append_header(("Content-Type", "text/html"))
        .body(format_html(
            render.await,
            &format!("<title>{}</title>", &name),
        ));
}

#[function_component]
fn ViewBoardPost(props: &ViewPostProps) -> Html {
    let p = &props.post;
    let post = serde_json::from_str::<BoardPostLog>(&p.content).unwrap();
    let board = serde_json::from_str::<BoardMetadata>(&props.board.metadata).unwrap();
    let content = Html::from_html_unchecked(AttrValue::from(post.content_html.clone()));

    // check if we can delete this post
    // must be authenticated AND board owner OR staff OR post author
    let can_delete: bool = props.auth_state.is_some()
        && props.user.is_some()
        && props.user.as_ref().unwrap().username != String::from("Anonymous")
        && ((props.user.as_ref().unwrap().username == board.owner)
            | (props.user.as_ref().unwrap().role == String::from("staff"))
            | (props.user.as_ref().unwrap().username == post.author));

    // ...
    return html! {
        <div class="flex flex-column g-4" style="height: 100dvh;">
            <div style="display: none;" id="board-name">{&props.board.name}</div>
            <div style="display: none;" id="post-id">{&props.post.id}</div>

            <div class="toolbar flex justify-space-between">
                // left
                <div class="flex">
                    <a class="button" href="/" style="border-left: 0">
                        <b>{"::SITE_NAME::"}</b>
                    </a>

                    <a class="button" href={format!("/b/{}", props.board.name)} style="border-left: 0">
                        {props.board.name.clone()}
                    </a>
                </div>
            </div>

            <div class="toolbar-layout-wrapper">
                <main class="small flex flex-column g-4">
                    <div id="error" class="mdnote note-error full" style="display: none;" />
                    <div id="success" class="mdnote note-note full" style="display: none;" />

                    <div class="card secondary round full flex flex-column g-4">
                        <span class="chip mention round" style="width: max-content;">
                            {if post.author != "Anonymous" {
                                html! {<a href={format!("/~{}", &post.author)} style="color: inherit;">{&post.author}</a>}
                            } else {
                                html! {<span>{"Anonymous"}</span>}
                            }}
                        </span>

                        <div>{content}</div>
                    </div>

                    {if can_delete {
                        html! {
                            <button class="bundles-primary round" id="delete-post" data-endpoint={format!("/api/board/{}/posts/{}", &post.board, &p.id)}>{"Delete"}</button>
                        }
                    } else {
                        html! {}
                    }}

                    <Footer auth_state={props.auth_state} />
                </main>
            </div>

            <script type="module">
                {"import \"/static/js/ManageBoardPost.js\";"}
            </script>
        </div>
    };
}

fn build_view_post_renderer_with_props(props: ViewPostProps) -> ServerRenderer<ViewBoardPost> {
    ServerRenderer::<ViewBoardPost>::with_props(|| props)
}

#[get("/b/{name:.*}/posts/{id:.*}")]
/// Available at "/b/{name}/posts/{id:.*}"
pub async fn view_board_post_request(
    req: HttpRequest,
    data: web::Data<db::bundlesdb::AppData>,
) -> impl Responder {
    // you're able to do this even if the board is private ON PURPOSE

    // verify auth status
    let token_cookie = req.cookie("__Secure-Token");
    let mut set_cookie: &str = "";

    let token_user = if token_cookie.is_some() {
        Option::Some(
            data.db
                .get_user_by_hashed(token_cookie.as_ref().unwrap().value().to_string()) // if the user is returned, that means the ID is valid
                .await,
        )
    } else {
        Option::None
    };

    if token_user.is_some() {
        // make sure user exists, refresh token if not
        if token_user.as_ref().unwrap().success == false {
            set_cookie = "__Secure-Token=refresh; SameSite=Strict; Secure; Path=/; HostOnly=true; HttpOnly=true; Max-Age=0";
        }
    }

    // get board
    let name: String = req.match_info().get("name").unwrap().to_string();

    let board: bundlesdb::DefaultReturn<Option<Board<String>>> =
        data.db.get_board_by_name(name.clone()).await;

    if board.success == false {
        let renderer = ServerRenderer::<crate::pages::errors::_404Page>::new();
        return HttpResponse::NotFound()
            .append_header(("Content-Type", "text/html"))
            .body(utility::format_html(
                renderer.render().await,
                "<title>404: Not Found</title>",
            ));
    }

    // get post
    let id: String = req.match_info().get("id").unwrap().to_string();
    let post: bundlesdb::DefaultReturn<Option<Log>> = data.db.get_log_by_id(id.clone()).await;

    if post.success == false {
        let renderer = ServerRenderer::<crate::pages::errors::_404Page>::new();
        return HttpResponse::NotFound()
            .append_header(("Content-Type", "text/html"))
            .body(utility::format_html(
                renderer.render().await,
                "<title>404: Not Found</title>",
            ));
    }

    // ...
    let renderer = build_view_post_renderer_with_props(ViewPostProps {
        board: board.payload.unwrap(),
        post: post.payload.unwrap(),
        auth_state: if req.cookie("__Secure-Token").is_some() {
            Option::Some(true)
        } else {
            Option::Some(false)
        },
        user: if token_user.is_some() {
            Option::Some(token_user.unwrap().payload.unwrap())
        } else {
            Option::None
        },
    });

    let render = renderer.render();
    return HttpResponse::Ok()
        .append_header(("Set-Cookie", set_cookie))
        .append_header(("Content-Type", "text/html"))
        .body(format_html(
            render.await,
            &format!("<title>{}</title>", &name),
        ));
}

#[function_component]
fn BoardSettings(props: &SettingsProps) -> Html {
    let metadata = serde_json::from_str::<bundlesdb::BoardMetadata>(&props.board.metadata).unwrap();

    return html! {
        <div>
            <div class="toolbar flex justify-space-between">
                // left
                <div class="flex">
                    <a class="button" href="/" style="border-left: 0">
                        <b>{"::SITE_NAME::"}</b>
                    </a>

                    <a class="button" href={format!("/b/{}", props.board.name)} style="border-left: 0">
                        {props.board.name.clone()}
                    </a>
                </div>

                // right
                <div class="flex">
                    <a class="button" href={format!("/b/{}", props.board.name)}>{"Feed"}</a>
                </div>
            </div>

            <div class="toolbar-layout-wrapper">
                <main class="flex flex-column g-4 small">
                    <h2 class="full text-center">{"Board Settings"}</h2>

                    <div class="card round secondary flex flex-column g-4">
                        <div class="flex full justify-space-between">
                            <div class="flex g-4">
                                <form action="/api/metadata" id="update-form">
                                    <button class="green round secondary">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-save"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                                        {"Save"}
                                    </button>
                                </form>

                                <button class="secondary round" id="add_field">{"Add Field"}</button>
                            </div>

                            <a href={format!("/b/{}", props.board.name)} class="button round secondary">{"Cancel"}</a>
                        </div>

                        <div id="options-field" class="flex flex-wrap mobile:flex-column g-4 full justify-space-between" />
                    </div>

                    <script type="module">
                        {format!("import {{ paste_settings }} from \"/static/js/SettingsEditor.js\";
                        paste_settings({}, \"{}\", document.getElementById(\"options-field\"), \"board\");", serde_json::to_string(&metadata).unwrap(), &props.board.name)}
                    </script>

                    <Footer auth_state={props.auth_state} />
                </main>
            </div>
        </div>
    };
}

fn build_settings_with_props(props: SettingsProps) -> ServerRenderer<BoardSettings> {
    ServerRenderer::<BoardSettings>::with_props(|| props)
}

#[get("/b/{name:.*}/manage")]
/// Available at "/b/{name}/manage"
pub async fn board_settings_request(
    req: HttpRequest,
    data: web::Data<bundlesdb::AppData>,
) -> impl Responder {
    // get board
    let name: String = req.match_info().get("name").unwrap().to_string();
    let name_c = name.clone();

    let board: bundlesdb::DefaultReturn<Option<Board<String>>> =
        data.db.get_board_by_name(name).await;

    if board.success == false {
        return HttpResponse::NotFound().body(board.message);
    }

    // verify auth status
    let token_cookie = req.cookie("__Secure-Token");
    let mut set_cookie: &str = "";

    let token_user = if token_cookie.is_some() {
        Option::Some(
            data.db
                .get_user_by_hashed(token_cookie.as_ref().unwrap().value().to_string()) // if the user is returned, that means the ID is valid
                .await,
        )
    } else {
        Option::None
    };

    if token_user.is_some() {
        // make sure user exists, refresh token if not
        if token_user.as_ref().unwrap().success == false {
            set_cookie = "__Secure-Token=refresh; SameSite=Strict; Secure; Path=/; HostOnly=true; HttpOnly=true; Max-Age=0";
        }
    } else {
        return HttpResponse::NotAcceptable().body("An account is required to do this");
    }

    // ...
    let metadata =
        serde_json::from_str::<bundlesdb::BoardMetadata>(&board.payload.as_ref().unwrap().metadata)
            .unwrap();

    let user = token_user.unwrap().payload.unwrap();
    let can_view: bool = (user.username == metadata.owner) | (user.role == String::from("staff"));

    if can_view == false {
        return HttpResponse::NotFound()
            .body("You do not have permission to manage this board's contents.");
    }

    // ...
    let renderer = build_settings_with_props(SettingsProps {
        board: board.payload.clone().unwrap(),
        auth_state: if req.cookie("__Secure-Token").is_some() {
            Option::Some(req.cookie("__Secure-Token").is_some())
        } else {
            Option::Some(false)
        },
    });

    let render = renderer.render();
    return HttpResponse::Ok()
        .append_header(("Set-Cookie", set_cookie))
        .append_header(("Content-Type", "text/html"))
        .body(format_html(
            render.await,
            &format!(
                "<title>{}</title>
                <meta property=\"og:title\" content=\"{} (board settings) - ::SITE_NAME::\" />",
                &name_c, &name_c
            ),
        ));
}
