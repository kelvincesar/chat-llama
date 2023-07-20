use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::input_area::InputArea;


use crate::api::converse;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    
    // Sinal (read and write) para a área do chat
    let (conversation, set_conversation) = create_signal(cx, Conversation::new());
    
    // Ação de envio da mensagem inserida pelo usuário e renderização da mesma
    // na área do chat
    let send = create_action(cx, move |new_message: &String| {
        // Instancia nova mensagem
        let user_message = Message {
            text: new_message.clone(),
            user: true
        };

        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        // Envia mensagem para o backend processar com o llama model
        // Nesta parte do client, a integração da rota e formatação necessária para a API
        // é resolvida pela declaração `server(Converse "/api")` 
        converse(cx, conversation.get())
    });

    // Agurda um input do usuário
    create_effect(cx, move|_| {
        // input_signal: insere `...` na conversa após inserção de mensagem;
        if let Some(_) = send.input().get() {
            let model_message = Message {    
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            })
        }

    });
    // Aguarda mensagem do backend com a inferencia
    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                // captura a última mensagem do chat e modifica os `...` pela
                // resposta da inferencia
                c.messages.last_mut().unwrap().text = response;
            })
        }
    });
    
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to LLAMA chat"/>
        
        // Recebe signal read da conversa
        <ChatArea conversation/>

        // Recebe send para permitir a inserção de novas mensagens;
        <InputArea send/>
    }
}

