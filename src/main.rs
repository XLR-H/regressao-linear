use regressao_temporal::*;

fn main() {
    let y = ler_csv("dados.csv");
    let (a,b) = regressao_linear(&y);
    println!("Coeficiente angular (a): {}", a);
    println!("coeficiente linear (b): {}", b);

    let y_pred: Vec<f64> = (0..y.len()).map(|x: usize| prever(&a, b, x as f64)).collect();
    let erro = mse(&y, &y_pred);
    let determinacao = r2(&y, &y_pred);
    
    println!("MSE: {}", erro);
    println!("R²: {}", determinacao);
    
    let futuro = y.len() as f64;
    let previsao = prever(&a, b, futuro);
    println!("Previsão para x = {}: {}", futuro, previsao);
}