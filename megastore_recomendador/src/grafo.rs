use std::collections::HashMap;

/// Grafo simples para recomendações de produtos relacionados.
pub struct GrafoRecomendacao {
    adjacencias: HashMap<String, Vec<String>>,
}

impl GrafoRecomendacao {
    pub fn new() -> Self {
        GrafoRecomendacao {
            adjacencias: HashMap::new(),
        }
    }

    /// Adiciona uma relação de recomendação entre dois produtos.
    pub fn adicionar_recomendacao(&mut self, produto: &str, recomendado: &str) {
        self.adjacencias
            .entry(produto.to_string())
            .or_default()
            .push(recomendado.to_string());
    }

    /// Retorna os produtos recomendados para um produto.
    pub fn recomendar(&self, produto: &str) -> Vec<String> {
        self.adjacencias
            .get(produto)
            .cloned()
            .unwrap_or_else(Vec::new)
    }
}
