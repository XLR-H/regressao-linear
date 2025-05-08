use regressao_temporal::*;

#[test]
fn test_media() {
    let dados = vec![1.0, 2.0, 3.0];
    assert_eq!(media(&dados), 2.0);
}

#[test]
fn test_prever() {
    let resultado = prever(&2.0, 1.0, 3.0); // f(x) = 2x + 1
    assert_eq!(resultado, 7.0);
}
