# Regressão Temporal

Projeto em Rust para realizar análise de séries temporais simples usando regressão linear.

## 📈 Funcionalidades

- Leitura de arquivos CSV com dados numéricos
- Cálculo da média
- Ajuste de regressão linear simples (reta de melhor ajuste)
- Previsão de valores futuros
- Cálculo de métricas de avaliação (MSE e R²)

## 🧪 Testes

O projeto inclui testes automatizados para as funções principais. Para rodar os testes:

```bash
cargo test
```

## 📂 Estrutura do Projeto

```
.
├── src/
│   ├── lib.rs        # Lógica principal da regressão
│   └── main.rs       # Exemplo de uso
├── tests/
│   └── testes.rs     # Testes unitários
├── dados.csv         # Exemplo de dados de entrada
└── Cargo.toml
```

## 📥 Entrada Esperada

O CSV deve conter uma única coluna com cabeçalho, por exemplo:

```
valor
10.0
12.3
15.1
...
```


Feito por Rafael 👨‍💻
