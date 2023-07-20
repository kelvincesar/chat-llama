use serde::{Serialize, Deserialize};


// Armazena as mensagens do chat
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}

 impl Conversation {
     pub fn new() -> Self {
         Self {
             messages: Vec::new()
         }
     }
 }

// Estrutura de mensagem
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,     // `true` em casos que a mensagem for do usuário
    pub text: String,
}




