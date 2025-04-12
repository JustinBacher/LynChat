# Prologue

So we have implemented the basics of how the project will work, and we're ready to start brainstorming ideas for the next steps.

## Tools

Currently, the project uses ollama-rs and that  project seems to have alot of shortcomings, so we're looking at other options.

- currently the [genai](https://docs.rs/genai/0.1.23/genai/index.html) crate looks like the best option.
  - it had alot of options enabling us to use many of the other clients and it seems tools are easier to implement.

The way we implement tools is incorrect, we are using an old implementation where we send the llm a brief summary of the tool and then the llm uses that to decide what to do.
The new way tools are implemented in llms is by using the API itself, and genai seems like it implements that.

We also plan on implementing a TON of new tools that will be useful for the project. Tools like

- Controlling the computer in different ways
  - Running scripts (Ensuring the script is safe)
- Installing/uninstalling software
- working with files
  - getting files that match a query
  - deleting files
  - creating files
  - editing files
  - moving files
  - etc...
- Implementing MCP servers

With having so many tools, we're going to need a way to store them, and we're going to need a way to search through them.
This means we're embedding them, (already done) but the discovery tool is not implemented correctly. Currently the implementation is very basic, and we need to implement a way to search through the tools and it's not even recognizing the discovery tool at all. Which is the reason for using [genai](https://docs.rs/genai/0.1.23/genai/index.html) instead of the old implementation.

## Voice

Currently, there is no voice support, but we're planning on implementing it.
Whisper seems to be the best option for voice recognition but I'd like to have it vendored with the project so that we can use it without having to install it.
As for tts I have no idea what to do.

## GUI

We're planning on implementing a GUI for the project.
We're thinking of using [tauri](https://tauri.app/) for the GUI
