use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    db: Arc<RwLock<MissionControlDb>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MissionControlDb {
    pub boards: Value,
    pub tasks: Vec<Value>,
    pub agents: Value,
}

impl MissionControlDb {
    pub async fn load() -> Self {
        let path = "mission_control_db.json";
        if let Ok(content) = tokio::fs::read_to_string(path).await {
            if let Ok(db) = serde_json::from_str(&content) {
                return db;
            }
        }

        // Seed initial data if DB doesn't exist
        let default_db = MissionControlDb {
            boards: json!([
                { "id": "1", "name": "Main Board" },
                { "id": "2", "name": "Development" },
                { "id": "3", "name": "Operations" }
            ]),
            tasks: vec![
                json!({ "id": "t1", "title": "PR hygiene automation: auto checklist/comment on PR", "status": "todo", "assigned_agent": "Backend Engineer", "tags": ["CI", "Medium"] }),
                json!({ "id": "t2", "title": "Security: protect system against prompt injection", "status": "todo", "assigned_agent": "Backend Engineer", "tags": ["Security", "Medium"] }),
                json!({ "id": "t3", "title": "Landing page design review + improvement", "status": "todo", "assigned_agent": "Docs+Frontend QA", "tags": ["Medium"] }),
                json!({ "id": "t4", "title": "Implement standardized pagination + reusable list", "status": "in_progress", "assigned_agent": "Backend Engineer", "tags": ["High"] }),
                json!({ "id": "t5", "title": "Docs overhaul (Phase 2): Implement rancher-docs", "status": "in_progress", "assigned_agent": "Docs+Frontend QA", "tags": ["CI", "High"] }),
                json!({ "id": "t6", "title": "Fix: blocked task transitions should return 409/422", "status": "review", "assigned_agent": "Backend Engineer", "tags": ["High"] }),
                json!({ "id": "t7", "title": "Policy: enforce 1 DB migration per PR + audit open PRs", "status": "review", "assigned_agent": "Backend Engineer", "tags": ["High"] }),
                json!({ "id": "t8", "title": "GitHub PR #129: Fix agent task patch auth and add", "status": "done", "assigned_agent": "Unassigned", "tags": ["High"] }),
                json!({ "id": "t9", "title": "Auth/Permissions: assigned agents can't PATCH tasks", "status": "done", "assigned_agent": "Unassigned", "tags": ["Reliability", "Audit", "High"] }),
            ],
            agents: json!([
                { "id": "a1", "name": "Backend Engineer", "model": "gpt-4", "status": "active" },
                { "id": "a2", "name": "Docs+Frontend QA", "model": "gpt-4-turbo", "status": "active" },
                { "id": "a3", "name": "Lead Agent", "model": "claude-3-opus", "status": "idle" },
            ]),
        };
        // Auto-save initial creation
        let _ = tokio::fs::write(path, serde_json::to_string_pretty(&default_db).unwrap()).await;
        default_db
    }

    pub async fn save(&self) {
        let path = "mission_control_db.json";
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = tokio::fs::write(path, content).await;
        }
    }
}

async fn dashboard() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("index.html"))
}

async fn list_boards(State(state): State<AppState>) -> Json<Value> {
    let db = state.db.read().await;
    Json(json!({ "boards": db.boards }))
}

async fn list_tasks(State(state): State<AppState>) -> Json<Value> {
    let db = state.db.read().await;
    Json(json!({ "tasks": db.tasks }))
}

async fn list_agents(State(state): State<AppState>) -> Json<Value> {
    let db = state.db.read().await;
    Json(json!({ "agents": db.agents }))
}

#[derive(Deserialize)]
struct UpdateTaskPayload {
    status: String,
}

async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTaskPayload>,
) -> Json<Value> {
    let mut db = state.db.write().await;

    // Find task and update
    let mut modified = false;
    for task in db.tasks.iter_mut() {
        if task.get("id").and_then(|v| v.as_str()) == Some(&id) {
            if let Some(obj) = task.as_object_mut() {
                obj.insert("status".to_string(), Value::String(payload.status.clone()));
                modified = true;
                break;
            }
        }
    }

    if modified {
        db.save().await;
    }

    Json(json!({ "success": true }))
}

pub async fn start_web_server(port: u16) {
    let initial_db = MissionControlDb::load().await;
    let app_state = AppState {
        db: Arc::new(RwLock::new(initial_db)),
    };

    let app = Router::new()
        .route("/", get(dashboard))
        .route("/api/boards", get(list_boards))
        .route("/api/tasks", get(list_tasks))
        .route("/api/agents", get(list_agents))
        .route("/api/tasks/:id", put(update_task))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("🚀 Starting Mission Control Web UI on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
