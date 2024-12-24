use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use axum::Json;
use std::net::SocketAddr;

// Estrutura da Tarefa
#[derive(Deserialize, Serialize, Debug)]
struct Task {
    title: String,
    completed: bool,
}

// Função para criar uma nova tarefa (recebe dados via POST)
async fn create_task(Json(payload): Json<Task>) -> Json<Task> {
    // Aqui você pode salvar a tarefa no banco de dados, se necessário
    println!("Tarefa recebida: {:?}", payload); // Apenas imprime a tarefa no terminal
    Json(payload) // Retorna a tarefa criada como resposta
}

// Função para obter a página inicial (GET)
async fn home() -> &'static str {
    "API de Tarefas"
}

// Função principal que configura o servidor
#[tokio::main]
async fn main() {
    // Cria o roteador e mapeia as rotas
    let app = Router::new()
        .route("/", get(home))              // Rota GET na raiz
        .route("/tasks", post(create_task)); // Rota POST para /tasks
    
    // Configura o endereço do servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://127.0.0.1:3000");

    // Inicia o servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
