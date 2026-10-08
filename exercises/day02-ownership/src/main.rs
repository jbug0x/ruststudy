fn main() {
    println!("Dia 2: ownership, borrowing, referencias, slices");

    let text = String::from("rust e uma linguagem de programacao de sistemas");

    // TODO 1: implemente word_count(text: &str) -> usize
    // println!("palavras: {}", word_count(&text));

    // TODO 2: implemente longest_word(text: &str) -> &str
    // println!("maior palavra: {}", longest_word(&text));

    // TODO 3 (frame a frame): escreva uma versao que MOVE a String para
    // dentro de uma funcao e tente usar `text` de novo depois. Leia o
    // erro do compilador com atencao antes de "corrigir".

    // Desafio: reverse_words(text: &str) -> String
    // (inverte a ORDEM das palavras, nao os caracteres de cada palavra)

    let _ = &text; // remova esta linha quando usar `text` de verdade
}

// fn word_count(text: &str) -> usize {
//     todo!()
// }

// fn longest_word(text: &str) -> &str {
//     todo!()
// }

// fn reverse_words(text: &str) -> String {
//     todo!()
// }
