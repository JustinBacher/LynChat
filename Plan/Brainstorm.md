# Lyn

This program is supposed to be an assistant that allows for daily users and power users alike.
The program should be designed to be used locally with support for local LLMs and web API based LLMs alike.
When used the program allows users to ask it to do tasks on their computer such as

- looking up info from the web
  - Like how people normally use google
  - getting news
  - new programs they can install
    - The program should also support installing the applications
- everyday tasks like calculator, moving files, finding the date.
  - Finding photos, files moving, grouping and rearranging files/photos

It should not have a concept of chats but rather remember chats by summarizing what was conversed.
It should store all user data/previous chat info locally, but it should have the option of storing their data in the cloud.

## Sensitive data anonymity

When storing user data in the cloud or using web LLMs it needs to ensure that the cloud service does not know what user input the data.
    - This is to ensure sensitive data is not leaked. The less the web app knows the less chances a bad actor has to intercept information.
    - This means that when ran on the client and the user wants to converse with an LLM that is accessed through a web API it shouldn't be using an API key the user provides, rather the client should send a request to our servers which send a request to their LLM of choice and at that point, store their information for later retrieval.

To maintain the highest level of user trust the application should be open source with only minor exceptions made for items that are closed source.

## Self managed context

Instead of allowing the LLM to manage it's own context, the LLM should maintain the context and send only relevant information for the current prompt to the LLM and the LLM should have a clean context on each request.

- The message sent to the LLM should inform the LLM of what it's role is, the current status of the conversation, any contextual information relevant to the conversation as well as the prompt from the user

## Tools

Tools should be offered to LLM that allow it to do such things like modify the users computer, query for information like prior discussed topics like relevant historical chats, user preferences, files, etc...

This mixes into the aforementioned tasks that can be performed by the LLM, but it offers the LLM the ability to request information if not provided initially.

## Miscellaneous

- This application to be performant, async, and should support internationalization.
- I want to offer a GUI client, web client as well as a CLI or TUI client.
- To store memories of chats and context, I would like to use Qdrant.
- The program should do it's best to recognize key information about the user and/or key conversations for later retrieval. This is to include preferences such as speaking preferences, response lengths, etc..

### Rust specifics

- Rust version should be 2024 edition
- Do not use Tokio, use async-std
- Use thiserror but use the global Result type defined in prelude, which means Modules can implement their own error types using their own Enums but they will need to be included into the project's prelude error Enum
- Imports should be merged by module
- Modules should be specific in their intentions (ie. smaller/more specific modules are preferred over larger/broader ones. Unless implementing something specific like many implementations of a trait, except this should be done using multiple sub-modules/files).
- Use traits to reduce code duplication and keep relevant code together when applicable
- Avoid heavily nested code, code should only be nested 2-3 depths at absolute maximum where possible
- Use tracing and tracing-subscriber crates over using logging crate
- Keep code concise and use rust conventions like `match`, `let Ok/Some = ...`,

