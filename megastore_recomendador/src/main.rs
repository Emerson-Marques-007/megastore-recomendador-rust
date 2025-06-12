use std::io;

mod produto;
mod hash_table;
mod grafo;

use produto::Produto;
use hash_table::CatalogoHash;
use grafo::GrafoRecomendacao;

fn main() {
    let mut hash_table = CatalogoHash::new();
    let mut grafo = GrafoRecomendacao::new();

    let produtos = vec![
        Produto::new(1, "Notebook", "Eletrônicos", 2500.0),
        Produto::new(2, "Mouse", "Periféricos", 80.0),
        Produto::new(3, "Teclado", "Periféricos", 150.0),
        Produto::new(4, "Monitor", "Eletrônicos", 900.0),
        Produto::new(5, "Celular", "Eletrônicos", 1800.0),
        Produto::new(6, "Fone de Ouvido", "Acessórios", 120.0),
        Produto::new(7, "Carregador", "Acessórios", 100.0),
        Produto::new(8, "Smartwatch", "Acessórios", 600.0),
        Produto::new(9, "TV", "Eletrônicos", 2800.0),
        Produto::new(10, "Controle Remoto", "Acessórios", 90.0),
    ];

    for produto in &produtos {
        hash_table.inserir(produto.clone());
        // grafo.adicionar_produto(produto.clone()); // Remova ou comente esta linha
    }

    // Recomendações (ajuste conforme sua implementação do grafo)
    grafo.adicionar_recomendacao("Notebook", "Mouse");
    grafo.adicionar_recomendacao("Notebook", "Teclado");
    grafo.adicionar_recomendacao("Notebook", "Monitor");
    grafo.adicionar_recomendacao("Celular", "Fone de Ouvido");
    grafo.adicionar_recomendacao("Celular", "Carregador");
    grafo.adicionar_recomendacao("Celular", "Smartwatch");
    grafo.adicionar_recomendacao("TV", "Controle Remoto");
    grafo.adicionar_recomendacao("TV", "Fone de Ouvido");

    println!("🔎 Digite o nome do produto que deseja buscar:");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler a entrada");
    let nome_busca = entrada.trim();

    match hash_table.buscar(nome_busca) {
        Some(produto) => {
            println!("\n🔍 Produto encontrado:");
            println!("{:?}", produto);

            println!("\n🤖 Produtos recomendados para '{}':", nome_busca);
            let recomendacoes = grafo.recomendar(nome_busca);
            if recomendacoes.is_empty() {
                println!("Nenhuma recomendação encontrada.");
            } else {
                for rec in recomendacoes {
                    println!("{:?}", rec);
                }
            }
        }
        None => println!("Produto '{}' não encontrado na base.", nome_busca),
    }
}
