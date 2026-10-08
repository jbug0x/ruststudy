// Dia 6: organize aqui a logica (reaproveite dia 3 ou dia 4), separada
// do main.rs, para poder testar com `cargo test -p day06-modules-testing`.

pub mod contacts {
    // TODO: mova (ou recrie) Contact, ContactError e find_contact do dia 3
    // para dentro deste modulo, e troque o tratamento manual de erro por
    // propagacao com `?` onde fizer sentido.
}

#[cfg(test)]
mod tests {
    // use super::contacts::*;

    #[test]
    fn todo_caso_de_sucesso() {
        // TODO: teste o caminho feliz
        assert!(true);
    }

    #[test]
    fn todo_caso_de_erro() {
        // TODO: teste um caso de erro, verificando a variante exata
        // com `assert!(matches!(resultado, Err(MeuErro::Variante)))`
        assert!(true);
    }

    #[test]
    fn todo_caso_de_borda() {
        // TODO: teste lista vazia, string vazia, ou outro caso de borda
        assert!(true);
    }
}
