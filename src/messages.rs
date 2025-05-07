use crate::{
    bot::Bot,
    // database::Question,
    error,
};
use serenity::all::*;

impl Bot {
    pub async fn send_success_embed(
        ctx: &Context,
        command: &CommandInteraction,
        title: &str,
        description: String,
        ephemeral: bool,
    ) -> error::Result<()> {
        command
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .add_embed(
                            CreateEmbed::new()
                                .title(title)
                                .description(description)
                                .color(0x00ff00),
                        )
                        .ephemeral(ephemeral),
                ),
            )
            .await?;
        Ok(())
    }

    pub async fn send_error_embed(
        ctx: &Context,
        command: &CommandInteraction,
        title: &str,
        description: &str,
        ephemeral: bool,
    ) -> error::Result<()> {
        command
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .add_embed(
                            CreateEmbed::new()
                                .title(title)
                                .description(description)
                                .color(0xff0000),
                        )
                        .ephemeral(ephemeral),
                ),
            )
            .await?;
        Ok(())
    }

    // pub async fn edit_question(
    //     self,
    //     ctx: &Context,
    //     question: Question,
    //     message: Message,
    // ) -> error::Result<()> {
    //     match message.guild_id {
    //         Some(id) => {
    //             let answers = Answer::fetch_by_question_and_guild(
    //                 &self.pool,
    //                 question.question_id,
    //                 id.into(),
    //             );
    //             Ok(())
    //         }
    //         _ => Err(),
    //     }
    // }

    //     pub async fn send_basic_question(
    //         self,
    //         ctx: &Context,
    //         question: Question,
    //         guild: Guild,
    //     ) -> error::Result<()> {
    //         // let answers = Answer::fetch_by_question_and_guild(&self.pool, question.question_id, guild.id.into());

    //         Ok(())
    //     }
}
