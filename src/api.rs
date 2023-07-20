use leptos::*;
use crate::model::conversation::Conversation;


#[server(Converse "/api")]
pub async fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    use llm:models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev:ConnectionInfo;

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner()
    })
    .await.unwrap();

    use llm::KnownModel;
    let character_name = "### Assistente";
    let user_name = "### Humano";
    let persona = "A chat between a human and an assistant";

    let mut history = format!(
        "{character_name}:Olá - Como posso te ajudar?\n\
        {user_name}:Qual a capital do Brasil?\n\
        {character_name}:Brasília é a capital do Brasil.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{character_name}:{msg}\n")
        } else {
            format!("{user_name}:{msg}\n")
        };
        history.push_str(&curr_line);
    }

    let mut inference_result = String::new();
    let mut rng = rand::thread_rnd();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());
    
    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{persona}\n{history}\n{character_name}:")
                .as_str()
                .into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        inference_callback(String::from(user_name), &mut buf, &mut inference_result),
    )
    .unwrap_or_else(|e| panic!("{e}"));

    Ok(inference_result)
}

// Callback function indicando quando a inferencia do modelo deve parar
// A função para de gerar tokens quando encontra o parâmetro stop_sequence
// que neste caso é o user_name.
//
cfg_if! {
    if #[cfg(feature = "ssr")] {
    use std::convert::Infallible;
        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;

            move |resp| match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok::<llm::InferenceFeedback, Infallible>(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    if buf.is_empty() {
                        out_str.push_str(&t);
                    } else {
                        out_str.push_str(&reverse_buf);
                    }

                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }
        }
    }
}
