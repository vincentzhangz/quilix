use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Represents a generated route from the app directory.
#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub file_path: String,
    pub is_page: bool,
    pub is_api: bool,
    pub is_layout: bool,
    pub is_loading: bool,
    pub is_error: bool,
    pub is_not_found: bool,
}

/// Generates routes by walking the app directory.
pub fn generate_routes(app_dir: &Path) -> Result<Vec<Route>> {
    let mut routes = Vec::new();
    let entries = glob::glob(&format!("{}/**/*", app_dir.to_string_lossy()))?;

    for entry in entries.flatten() {
        if entry.is_file()
            && let Some(ext) = entry.extension()
            && (ext == "tsx" || ext == "ts")
        {
            let relative = entry.strip_prefix(app_dir)?.to_string_lossy().to_string();
            let route = parse_route(&relative);
            if let Some(r) = route {
                routes.push(r);
            }
        }
    }

    Ok(routes)
}

fn parse_route(relative: &str) -> Option<Route> {
    let path = relative.replace("\\", "/");
    let stem = Path::new(&path).file_stem()?.to_string_lossy().to_string();

    let (route_path, is_page, is_api, is_layout, is_loading, is_error, is_not_found) =
        if stem == "page" {
            let route_path = path_to_route(&path, "page");
            (route_path, true, false, false, false, false, false)
        } else if stem == "layout" {
            let route_path = path_to_route(&path, "layout");
            (route_path, false, false, true, false, false, false)
        } else if stem == "loading" {
            let route_path = path_to_route(&path, "loading");
            (route_path, false, false, false, true, false, false)
        } else if stem == "error" {
            let route_path = path_to_route(&path, "error");
            (route_path, false, false, false, false, true, false)
        } else if stem == "not-found" {
            let route_path = path_to_route(&path, "not-found");
            (route_path, false, false, false, false, false, true)
        } else if stem.starts_with("api.") {
            let route_path = path_to_api_route(&path);
            (route_path, false, true, false, false, false, false)
        } else if stem.starts_with("page.") || !path.contains('/') {
            let route_path = path_to_route(&path, "page");
            (route_path, true, false, false, false, false, false)
        } else {
            return None;
        };

    Some(Route {
        path: route_path,
        file_path: path,
        is_page,
        is_api,
        is_layout,
        is_loading,
        is_error,
        is_not_found,
    })
}

fn path_to_route(path: &str, _component: &str) -> String {
    let path = path.replace("\\", "/");
    let parts: Vec<&str> = path.split('/').collect();

    let mut route = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            continue;
        }
        if part.starts_with('(') && part.ends_with(')') {
            route.push_str(&part[1..part.len() - 1]);
        } else if part.starts_with('[') && part.ends_with(']') {
            route.push_str("/:");
            route.push_str(&part[1..part.len() - 1]);
        } else {
            route.push('/');
            route.push_str(part);
        }
    }

    if route.is_empty() {
        route.push('/');
    }

    route
}

fn path_to_api_route(path: &str) -> String {
    let path = path.replace("\\", "/");
    let parts: Vec<&str> = path.split('/').collect();

    let mut route = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            continue;
        }
        let stem = Path::new(part)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        if stem.starts_with('[') && stem.ends_with(']') {
            route.push_str("/:");
            route.push_str(&stem[1..stem.len() - 1]);
        } else if stem != "api" {
            route.push('/');
            route.push_str(&stem);
        }
    }

    if route.is_empty() {
        route.push('/');
    }

    route
}

/// Groups routes by type (pages/api).
pub fn generate_entries(routes: &[Route]) -> HashMap<String, Vec<String>> {
    let mut entries: HashMap<String, Vec<String>> = HashMap::new();

    for route in routes {
        if route.is_page {
            let path = format!("./{}", route.file_path);
            entries.entry("pages".to_string()).or_default().push(path);
        } else if route.is_api {
            let path = format!("./{}", route.file_path);
            entries.entry("api".to_string()).or_default().push(path);
        }
    }

    entries
}

/// Returns all API routes from a route list.
pub fn get_api_routes(routes: &[Route]) -> Vec<&Route> {
    routes.iter().filter(|r| r.is_api).collect()
}

/// Returns all page routes from a route list.
pub fn get_page_routes(routes: &[Route]) -> Vec<&Route> {
    routes.iter().filter(|r| r.is_page).collect()
}
