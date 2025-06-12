use std::collections::HashMap;
use crate::produto::Produto;

/// Estrutura para indexação eficiente de produtos usando HashMap.
pub struct CatalogoHash {
    produtos: HashMap<String, Produto>,
}

impl CatalogoHash {
    /// Cria um novo catálogo vazio
    pub fn new() -> Self {
        CatalogoHash {
            produtos: HashMap::new(),
        }
    }

    /// Insere um produto no catálogo.
    pub fn inserir(&mut self, produto: Produto) {
        self.produtos.insert(produto.nome.clone(), produto);
    }

    /// Busca um produto pelo nome.
    pub fn buscar(&self, nome: &str) -> Option<&Produto> {
        self.produtos.get(nome)
    }
}
