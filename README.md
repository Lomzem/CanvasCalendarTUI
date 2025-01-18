# Canvas Calendar TUI

This is a tool meant for the terminal that allows me to view my upcoming assignments on [canvas](https://www.instructure.com/canvas) faster.

The tool is designed specifically for me personally, but it's also possible to repurpose it for your own canvas account as well.

## Prerequisites
- The app uses environment variables to get your [Canvas Access Key](https://community.canvaslms.com/t5/Admin-Guide/How-do-I-manage-API-access-tokens-as-an-admin/ta-p/89)
- Store the Canvas Access Token in the environment variable **CANVAS_ACCESS_TOKEN** (for example add
`export CANVAS_ACCESS_TOKEN="key-here"`
into your .bashrc file)
- Store the Base Canvas URL in the environment variable **CANVAS_URL** (for example add
`export CANVAS_URL="https://csuchico.canvas.edu"`

## Controls
TODO! List controls
Just use j, k, l, h, o
<!-- I based the controls on vim bindings as a neovim user. Here are the current supported keybinds: -->

## Future Wanted Features
- Make browser open feature more extensible
