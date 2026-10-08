fn main() {
    println!("Dia 3: structs, enums, pattern matching, Option/Result");

    // TODO 1: defina a struct Contact { name: String, phone: Option<String> }

    // TODO 2: defina enum ContactError { NotFound, InvalidName }

    // TODO 3: implemente
    // fn find_contact<'a>(contacts: &'a [Contact], name: &str) -> Result<&'a Contact, ContactError>

    // let contacts = vec![
    //     Contact { name: "Ana".to_string(), phone: Some("111".to_string()) },
    //     Contact { name: "Beto".to_string(), phone: None },
    // ];
    //
    // match find_contact(&contacts, "Ana") {
    //     Ok(contact) => {
    //         if let Some(phone) = &contact.phone {
    //             println!("{}: {}", contact.name, phone);
    //         } else {
    //             println!("{} nao tem telefone cadastrado", contact.name);
    //         }
    //     }
    //     Err(ContactError::NotFound) => println!("contato nao encontrado"),
    //     Err(ContactError::InvalidName) => println!("nome invalido"),
    // }

    // Desafio: adicione ContactError::DuplicateName(String) e implemente
    // fn add_contact(contacts: &mut Vec<Contact>, new: Contact) -> Result<(), ContactError>
}
