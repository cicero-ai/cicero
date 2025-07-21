
use cicero_sdk::{NavMenu, NavMenuType};
use atlas_http::HttpBody;
use crate::utils::api_client;
use log::{error, trace};

pub fn render() -> String {

    // Get menus to display
    let menus = match api_client::send_body::<Vec<NavMenu>>("v1/echo/get_menus", "GET", &HttpBody::empty()) {
        Ok(r) => r,
        Err(e) => {
            error!("Did not receive valid response from Apollo server, unable to determine nav menus.");
            Vec::new()
        }
    };

    // Go through  menus and create html
    let mut html = String::new();
    for menu in menus {

        if menu.menu_type == NavMenuType::Separator {
            let sep = format!(r#"<li class="nav-item-header"><div class="text-uppercase font-size-xs line-height-xs">{}</div> <i class="icon-menu" title="{}"></i></li>"#, menu.name.clone(), menu.name);
            html.push_str(&sep.as_str());
            continue;
        }

        // Internal menu
        if menu.menu_type == NavMenuType::Internal {
            let url = format!("/account/{}/{}", menu.parent.unwrap(), menu.slug);
            let tmp = format!(r#"<li class="nav-item"><a href="{}" class="nav-link">{}</a></li>"#, url, menu.name);
            html.push_str(&tmp.as_str());
            continue;
        }

        // Add parent menu
        let mut parent = format!(r#"<li class="nav-item nav-item-submenu">
            <a href="{}" class="nav-link"><i class="{}"></i> <span>{}</span></a>
                <ul class="nav nav-group-sub" data-submenu-title="{}">
        "#,"#", menu.icon.unwrap_or("".to_string()), menu.name.clone(), menu.name); 

        // Go through all sub-menus
        for (slug, name) in menu.submenus {
            let url = format!("/account/{}/{}", menu.slug, slug);
            let submenu = format!(r#"<li class="nav-item"><a href="{}" class="nav-link">{}</a></li>"#, url, name);
            parent.push_str(&submenu.as_str());
        }
        parent.push_str("</ul></li>");
        html.push_str(&parent.as_str());
    }

    html
}




