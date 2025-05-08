use std::fs::File;
use std::io::{BufRead, BufReader};

fn ler_csv(caminho: &str) -> Vec<f64> {
    let arquivo = File::open(caminho).expect("Não foi possivel abrir o arquivo");
    let leitor = BufReader::new(arquivo);

    let mut linhas = leitor.lines();
    linhas.next(); //pula o cabeçalho

    let mut dados = Vec::new();
    for linha in linhas {
        if let Ok(conteudo) = linha {
            if let Ok(valor) = conteudo.trim().parse::<f64>() {
                dados.push(valor);
            }
        }
    }

    dados
}

fn media(v: &Vec<f64>) -> f64 {
    let soma: f64 = v.iter().sum();
    soma / v.len() as f64
}

fn regressao_linear(y: &Vec<f64>) -> (f64, f64) {
    let n = y.len();
    let x: Vec<f64> = (0..n).map(|v| v as f64).collect();
    let media_x = media(&x);
    let media_y = media(&y);

    let numerador: f64 = x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - media_x) * (yi - media_y))
        .sum();
    let denominador: f64 = x.iter()
        .map(|xi| (xi - media_x).powi(2))
        .sum();

    let a = numerador / denominador;
    let b = media_y - a * media_x;

    (a,b)
}

fn prever(a: &f64, b:f64, x:f64) -> f64 {
    a * x + b
}

fn mse(y: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
    y.iter()
        .zip(y_pred.iter())
        .map(|(yi, ypi)| (yi - ypi).powi(2))
        .sum::<f64>() / y.len() as f64
}

fn r2(y: &Vec<f64>, y_pred: &Vec<f64>) ->f64 {
    let media_y = media(y);
    let ss_total: f64 = y.iter().map(|yi| (yi - media_y).powi(2)).sum();
    let ss_res: f64 = y.iter()
        .zip(y_pred.iter())
        .map(|(yi, ypi)| (yi - ypi).powi(2))
        .sum();

    1.0 - (ss_res / ss_total)
}

fn main() {
    let y = ler_csv("dados.csv");
    let (a, b) = regressao_linear(&y);
    println!("Coeficiente angular (a): {}", a);
    println!("Coeficiente linear (b): {}", b);

    let y_pred: Vec<f64> = (0..y.len()).map(|x: usize| prever(&a, b, x as f64)).collect();
    let erro = mse(&y, &y_pred);
    let determinacao = r2(&y, &y_pred);

    println!("MSE: {}", erro);
    println!("R²: {}", determinacao);

    let futuro = y.len() as f64;
    let previsao = prever(&a, b, futuro);
    println!("Previsão para x = {}: {}", futuro, previsao);
}

