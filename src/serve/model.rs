use serde::{Deserialize, Deserializer, Serialize};
use typed_builder::TypedBuilder;

/// A role of a message sender, can be:
/// - `System`, for starting system message, that sets the tone of model
/// - `Assistant`, for messages sent by ChatGPT
/// - `User`, for messages sent by user
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// A system message, automatically sent at the start to set the tone of the model
    System,
    /// A message sent by ChatGPT
    Assistant,
    /// A message sent by the user
    User,
    /// A system message
    Critic,
}

// ==================== Request Body ====================

#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    #[serde(deserialize_with = "deserialize_model")]
    model: String,

    #[serde(deserialize_with = "deserialize_message")]
    messages: Vec<Message>,

    #[serde(skip_serializing, default)]
    stream: Option<bool>,
}

impl ChatRequest {
    pub fn stream(&self) -> Option<bool> {
        self.stream
    }

    pub fn model(self) -> String {
        self.model
    }
}

#[derive(Serialize, Deserialize, Default, TypedBuilder)]
pub struct Message {
    #[builder(default, setter(into))]
    role: Option<Role>,

    #[builder(default, setter(into))]
    content: Option<String>,
}

fn deserialize_model<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let model = String::deserialize(deserializer)?;
    let model = match model.as_str() {
        "claude-3-haiku" => "claude-3-haiku-20240307",
        "llama-3.1-70b" => "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo",
        "mixtral-8x7b" => "mistralai/Mixtral-8x7B-Instruct-v0.1",
        _ => "gpt-4o-mini",
    };

    Ok(model.to_owned())
}

fn deserialize_message<'de, D>(deserializer: D) -> Result<Vec<Message>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut message: Vec<Message> = Vec::deserialize(deserializer)?;
    for message in &mut message {
        if let Some(role) = message.role.as_mut() {
            if matches!(role, Role::System | Role::Critic) {
                *role = Role::User;
            }
        }
    }
    Ok(message)
}

// ==================== Duck APi Response Body ====================
#[derive(Deserialize)]
pub struct DuckChatCompletion {
    pub message: Option<String>,
    pub created: u64,
    pub id: String,
    pub model: String,
}

// ==================== Response Body ====================

#[derive(Serialize, TypedBuilder)]
pub struct ChatCompletion {
    #[builder(default, setter(into))]
    id: Option<String>,

    object: &'static str,

    #[builder(default, setter(into))]
    created: Option<u64>,

    model: String,

    choices: Vec<Choice>,

    #[builder(default, setter(into))]
    usage: Option<Usage>,
}

#[derive(Serialize, TypedBuilder)]
pub struct Choice {
    index: usize,

    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Message>,

    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    delta: Option<Message>,

    #[builder(setter(into))]
    logprobs: Option<String>,

    #[builder(setter(into))]
    finish_reason: Option<&'static str>,
}

#[derive(Serialize, TypedBuilder)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize)]
pub struct Pong {
    pub message: &'static str,
}

#[derive(Serialize, TypedBuilder)]
pub struct Models {
    object: &'static str,
    data: &'static [ModelData; 4],
}

#[derive(Serialize, Deserialize, TypedBuilder)]
pub struct ModelData {
    id: &'static str,
    #[builder(default = "model")]
    object: &'static str,
    #[builder(default = 1686935002)]
    created: i64,
    owned_by: &'static str,
}
