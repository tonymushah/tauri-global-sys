# tauri-global-sys

An equivalent of the [JonasKruckenberg / tauri-sys][tauri-sys] but uses the `window.__TAURI__` instead, which remove the dependance of  [esbuild].

This repository is still work in progress so some tauri features may not be implemented yet.

## TODO list

- [ ] [V1 support](https://v1.tauri.app/v1/api/js/)
  - [X] [app](https://v1.tauri.app/v1/api/js/app)
  - [X] [cli](https://v1.tauri.app/v1/api/js/cli)
  - [X] [clipboard](https://v1.tauri.app/v1/api/js/clipboard)
  - [X] [dialog](https://v1.tauri.app/v1/api/js/dialog)
  - [X] [event](https://v1.tauri.app/v1/api/js/event)
  - [X] [fs](https://v1.tauri.app/v1/api/js/fs)
  - [X] [globalShortcut](https://v1.tauri.app/v1/api/js/globalShortcut)
  - [X] [http](https://v1.tauri.app/v1/api/js/http)
  - [X] [notification](https://v1.tauri.app/v1/api/js/notification)
  - [X] [os](https://v1.tauri.app/v1/api/js/os)
  - [X] [path](https://v1.tauri.app/v1/api/js/path)
  - [ ] [process](https://v1.tauri.app/v1/api/js/process)
  - [ ] [shell](https://v1.tauri.app/v1/api/js/shell)
  - [X] [tauri](https://v1.tauri.app/v1/api/js/tauri)
  - [ ] [updater](https://v1.tauri.app/v1/api/js/updater)
  - [ ] [window](https://v1.tauri.app/v1/api/js/window)
- [ ] [V2 support](https://tauri.app/reference/javascript/api/)
  - [ ] [app](https://tauri.app/reference/javascript/api/namespaceapp/)
  - [ ] [core](https://tauri.app/reference/javascript/api/namespacecore/)
  - [ ] [dpi](https://tauri.app/reference/javascript/api/namespacedpi/)
  - [ ] [event](https://tauri.app/reference/javascript/api/namespaceevent/)
  - [ ] [image](https://tauri.app/reference/javascript/api/namespaceimage/)
  - [ ] [menu](https://tauri.app/reference/javascript/api/namespacemenu/)
  - [ ] [path](https://tauri.app/reference/javascript/api/namespacepath/)
  - [ ] [tray](https://tauri.app/reference/javascript/api/namespacetray/)
  - [ ] [webview](https://tauri.app/reference/javascript/api/namespacewebview/)
  - [ ] [webviewWindow](https://tauri.app/reference/javascript/api/namespacewebviewwindow/)
  - [ ] [window](https://tauri.app/reference/javascript/api/namespacewindow/)
- [ ] [v2 plugins] ??

### Why the `mocks` api isn't supported?

Because we can't access them in the webview.

## Licence

MIT

[tauri-sys]: https://github.com/JonasKruckenberg/tauri-sys
[esbuild]: https://esbuild.github.io/
