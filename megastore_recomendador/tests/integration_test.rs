use megastore_recomendador::produto::Produto;
use megastore_recomendador::hash_table::CatalogoHash;
use megastore_recomendador::grafo::GrafoRecomendacao;

#[test]
fn test_insercao_e_busca() {
    let mut catalogo = CatalogoHash::new();
    let produto = Produto::new(1, "Notebook", "Eletrônicos", 2500.0);
    catalogo.inserir(produto.clone());
    let resultado = catalogo.buscar("Notebook");
    assert_eq!(resultado, Some(&produto));
}

#[test]
fn test_recomendacao() {
    let mut grafo = GrafoRecomendacao::new();
    grafo.adicionar_recomendacao("Notebook", "Mouse");
    grafo.adicionar_recomendacao("Notebook", "Teclado");
    let recomendados = grafo.recomendar("Notebook");
    assert_eq!(recomendados, vec!["Mouse".to_string(), "Teclado".to_string()]);
}
