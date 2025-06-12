# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 1. Descrição do Projeto

Este projeto é um estudo de caso prático para a disciplina de Estruturas de Dados, com o objetivo de implementar um sistema de busca eficiente para o catálogo de produtos da "MegaStore", uma grande varejista de e-commerce. O sistema utiliza **tabelas hash** para indexação e busca rápida, e **grafos** para recomendação de produtos relacionados, tudo implementado em Rust.

## 2. Cenário

A MegaStore enfrenta dificuldades com buscas lentas e imprecisas em seu catálogo online, prejudicando a experiência do cliente e as vendas. O desafio é criar um sistema de busca otimizado, eficiente e escalável, capaz de lidar com milhões de produtos e fornecer recomendações relevantes.

## 3. Objetivos e Funcionalidades

- **Indexação eficiente** do catálogo de produtos usando tabelas hash.
- **Busca rápida e precisa** por nome de produto.
- **Recomendação automática** de produtos relacionados via conexões em grafo.
- **Escalabilidade** para grandes volumes de dados.
- **Segurança** e confiabilidade no acesso aos dados.

## 4. Tecnologias Utilizadas

- **Rust** (edition 2021)
- Estruturas de dados: `HashMap`, grafos customizados
- Ferramentas de teste: `cargo test`
- Modularização em múltiplos arquivos Rust

## 5. Arquitetura do Sistema

```
megastore_recomendador/
├── src/
│   ├── main.rs         # Interface principal e fluxo do sistema
│   ├── produto.rs      # Definição da struct Produto
│   ├── hash_table.rs   # Implementação da tabela hash para busca
│   ├── grafo.rs        # Implementação do grafo para recomendações
│   └── lib.rs          # Módulo de integração
├── tests/
│   └── integration_test.rs # Testes de integração
├── Cargo.toml
└── README.md
```

- **produto.rs**: Define o modelo de produto (`id`, `nome`, `categoria`, `preço`).
- **hash_table.rs**: Implementa uma tabela hash para busca eficiente por nome.
- **grafo.rs**: Implementa um grafo para recomendações de produtos relacionados.
- **main.rs**: Interface de linha de comando para busca e recomendação.

## 6. Algoritmos e Estruturas de Dados

- **Tabela Hash**: Utilizada para indexar produtos por nome, garantindo buscas em tempo constante na média.
- **Grafo**: Cada produto é um vértice; conexões representam recomendações. Busca por vizinhos retorna produtos relacionados.

## 7. Instruções de Execução

Clone o repositório e execute o sistema:

```bash
git clone <url-do-repositorio>
cd megastore_recomendador
cargo run
```

Siga as instruções no terminal para buscar produtos e ver recomendações.

## 8. Instruções de Testes

Para rodar os testes de integração e garantir a qualidade do código:

```bash
cargo test
```

## 9. Exemplos de Uso

- **Busca por produto**:  
  Digite o nome de um produto, por exemplo, `Notebook`. O sistema retorna os detalhes:
  ```
  Produto encontrado:
  Produto { id: 1, nome: "Notebook", categoria: "Eletrônicos", preco: 2500.0 }
  Produtos recomendados para 'Notebook':
  "Mouse"
  "Teclado"
  "Monitor"
  ```
- **Recomendação**:  
  Ao buscar por `Celular`, o sistema recomenda:  
  ```
  "Fone de Ouvido"
  "Carregador"
  "Smartwatch"
  ```

## 10. Considerações sobre Desempenho e Escalabilidade

- **Desempenho**: O uso de tabelas hash garante buscas rápidas mesmo com grandes volumes de dados.
- **Escalabilidade**: O sistema pode ser facilmente expandido para milhões de produtos, pois as operações principais são eficientes.
- **Limitações**: Atualmente, a busca é apenas por nome exato. Técnicas de pré-processamento de texto e cache podem ser implementadas para melhorar ainda mais.

## 11. Contribuições

Contribuições são bem-vindas! Abra uma issue ou pull request para sugerir melhorias, correções ou novas funcionalidades.

---

**Desenvolvido como estudo de caso para a disciplina de Estruturas de Dados.**
