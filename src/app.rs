use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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

