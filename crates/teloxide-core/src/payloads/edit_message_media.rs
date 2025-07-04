//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{
    BusinessConnectionId, InlineKeyboardMarkup, InputMedia, Message, MessageId, Recipient,
};

impl_payload! {
    /// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is a part of a message album, then it can be edited only to a photo or a video. Otherwise, message type can be changed arbitrarily. When inline message is edited, new file can't be uploaded. Use previously uploaded file via its file_id or specify a URL. On success, the edited Message is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    ///
    /// See also: [`EditMessageMediaInline`](crate::payloads::EditMessageMediaInline)
    #[derive(Debug, Clone, Serialize)]
    pub EditMessageMedia (EditMessageMediaSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`).
            pub chat_id: Recipient [into],
            /// Identifier of the message to edit
            #[serde(flatten)]
            pub message_id: MessageId,
            /// A JSON-serialized object for a new media content of the message
            pub media: InputMedia,
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the message to be edited was sent
            pub business_connection_id: BusinessConnectionId,
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
