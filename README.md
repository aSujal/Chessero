# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## TODO

- [x] Fix castling edge cases (captured rooks, verify rook exists).
- [x] Add checkmate and stalemate detection.
- [ ] Add draw detection (starting with insufficient material and the 50-move rule).
- [ ] Add proper SAN notation, including +, #, promotions, and disambiguation.
- [ ] Add FEN import/export so you can load arbitrary positions.
- [ ] Add PGN export/import.
- [ ] Add engine search (Minimax with alpha-beta pruning).
- [ ] Add position evaluation.
- [ ] Add move ordering, transposition tables, iterative deepening, and other engine optimizations.
