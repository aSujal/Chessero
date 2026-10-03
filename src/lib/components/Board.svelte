<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let boardState: BoardState | null = $state(null);
  let selectedSquareIndex: number | null = $state(null);
  let availableSquares: { row: number; col: number }[] | null = $state(null);
  let uciInput: string = $state("");
  let uciError: string = $state("");

  //array so castling can be animated too
  let slides: Move[] = $state([]);
  let slideId: number = $state(0);

  let boardEl: HTMLDivElement | undefined = $state();
  let dragging: { from: number; x: number; y: number } | null = $state(null);

  const lastRecord = $derived.by(() => {
    const history = boardState?.move_history ?? [];
    return history[history.length - 1];
  });

  const status = $derived.by(() => {
    if (!boardState) return "";
    const player = boardState.active_color === "white" ? "White" : "Black";
    const state = lastRecord?.game_state ?? "ongoing";

    if (state === "checkmate") return `Checkmate`;
    if (state === "stalemate") return "Stalemate";
    if (state.startsWith("draw")) return "Draw";
    if (state === "check") return `${player} is in check`;
    return `${player} to move`;
  });

  onMount(async () => {
    boardState = await invoke("get_initial_board");
  });

  const files = ["a", "b", "c", "d", "e", "f", "g", "h"];
  const ranks = ["8", "7", "6", "5", "4", "3", "2", "1"];
  const squares = Array.from({ length: 64 }, (_, i) => i);
  const getRow = (index: number): number => Math.floor(index / 8);
  const getColumn = (index: number): number => index % 8;

  function isDarkSquare(index: number): boolean {
    const row = getRow(index);
    const col = getColumn(index);
    return (row + col) % 2 == 1;
  }

  function coordsToNotation(row: number, col: number): string {
    return `${files[col]}${ranks[row]}`;
  }

  function formatMove(moveItem: UndoRecord): string {
    const { mv, moved_piece, captured_piece } = moveItem;
    const [_, fromCol] = mv.from;
    const [toRow, toCol] = mv.to;

    const pieceType = moved_piece.piece_type;
    let notation = "";

    let piecePrefix = "";
    if (pieceType !== "pawn") {
      piecePrefix = pieceType === "knight" ? "N" : pieceType.charAt(0).toUpperCase();
    }

    //castling
    if (pieceType == "king" && fromCol == 4) {
      if (toCol == 6) {
        notation = "O-O";
      } else if (toCol == 2) {
        notation = "O-O-O";
      }
    }
    if (notation === "") {
      const isCapture = captured_piece !== null;
      const toNotation = coordsToNotation(toRow, toCol);

      if (pieceType === "pawn") {
        if (isCapture) {
          notation = `${files[fromCol]}x${toNotation}`;
        } else {
          notation = toNotation;
        }
      } else {
        notation = `${piecePrefix}${isCapture ? "x" : ""}${toNotation}`;
      }
    }

    if (moveItem.game_state == "check") {
      notation += "+";
    } else if (moveItem.game_state == "checkmate") {
      notation += "#";
    }

    return notation;
  }

  function slidePieces(from: [number, number], to: [number, number]) {
    slides = [{ from, to }];

    // castling moves the rook too
    const piece = boardState?.squares[to[0]][to[1]];
    if (piece?.piece_type === "king" && Math.abs(to[1] - from[1]) === 2) {
      const row = to[0];
      const kingside = Math.max(from[1], to[1]) === 6;
      const corner = kingside ? 7 : 0;
      const inner = kingside ? 5 : 3;
      const undoing = to[1] === 4;

      slides.push(undoing ? { from: [row, inner], to: [row, corner] } : { from: [row, corner], to: [row, inner] });
    }

    slideId++;
  }

  function handlePointerDown(e: PointerEvent, index: number) {
    if (e.button !== 0) return;
    const piece = boardState?.squares[getRow(index)][getColumn(index)];
    if (!piece || piece.color !== boardState?.active_color) return;

    selectedSquareIndex = index;
    getAvailableSquares(index);
    dragging = { from: index, x: e.clientX, y: e.clientY };
  }

  function handlePointerMove(e: PointerEvent) {
    if (!dragging) return;
    dragging.x = e.clientX;
    dragging.y = e.clientY;
  }

  function handlePointerUp(e: PointerEvent) {
    if (!dragging || !boardEl) return;

    const rect = boardEl.getBoundingClientRect();
    const squareSize = rect.width / 8;
    const col = Math.floor((e.clientX - rect.left) / squareSize);
    const row = Math.floor((e.clientY - rect.top) / squareSize);

    if (row >= 0 && row < 8 && col >= 0 && col < 8) {
      const targetIndex = row * 8 + col;
      tryMove(dragging.from, targetIndex);
    }

    dragging = null;
  }

  async function getAvailableSquares(index: number) {
    if (!boardState) return;
    const row = getRow(index);
    const col = getColumn(index);
    const moves: [number, number][] = await invoke("get_moves", {
      board: boardState,
      row: row,
      col: col,
    });

    availableSquares = moves.map(([r, c]) => ({ row: r, col: c }));
  }

  async function handleSquareClick(index: number) {
    const clickedRow = getRow(index);
    const clickedCol = getColumn(index);
    const clickedPiece = boardState?.squares[clickedRow][clickedCol];

    if (selectedSquareIndex !== null) {
      const fromRow = getRow(selectedSquareIndex);
      const fromCol = getColumn(selectedSquareIndex);
      if (await tryMove(selectedSquareIndex, index)) {
        slidePieces([fromRow, fromCol], [clickedRow, clickedCol]);
        return;
      }
    }

    if (clickedPiece) {
      selectedSquareIndex = index;
      getAvailableSquares(index);
    } else {
      selectedSquareIndex = null;
      availableSquares = null;
    }
  }

  async function tryMove(fromIndex: number, toIndex: number): Promise<boolean> {
    const fromRow = getRow(fromIndex);
    const fromCol = getColumn(fromIndex);
    const toRow = getRow(toIndex);
    const toCol = getColumn(toIndex);

    if (availableSquares?.some((s) => s.row === toRow && s.col === toCol)) {
      const updatedBoard: BoardState = await invoke("make_move", {
        board: boardState,
        fromRow: fromRow,
        fromCol: fromCol,
        toRow: toRow,
        toCol: toCol,
      });

      slides = [];
      boardState = updatedBoard;
      selectedSquareIndex = null;
      availableSquares = null;

      return true;
    }
    return false;
  }

  async function handleUndo(index?: number) {
    if (!boardState) return;
    const undone = index === undefined ? lastRecord : undefined;
    const updatedBoard: BoardState = await invoke("undo_move", {
      board: boardState,
      target_index: index,
    });

    boardState = updatedBoard;
    selectedSquareIndex = null;
    availableSquares = null;

    if (undone) {
      slidePieces(undone.mv.to, undone.mv.from);
    } else {
      slides = [];
    }
  }

  async function handlePlayUCI() {
    if (!boardState || !uciInput.trim()) return;
    try {
      const updatedBoard: BoardState = await invoke("play_uci_sequence", {
        board: boardState,
        moves: uciInput.trim(),
      });
      boardState = updatedBoard;
      uciInput = "";
      uciError = "";
      slides = [];
      selectedSquareIndex = null;
      availableSquares = null;
    } catch (error) {
      console.error("Invalid UCI sequence:", error);
      uciError = "Invalid UCI sequence. Example: e2e4 e7e5";
    }
  }
</script>

<svelte:window onpointermove={handlePointerMove} onpointerup={handlePointerUp} onpointercancel={() => (dragging = null)} />
<div class="game-container" class:dragging>
  <div class="chessboard" bind:this={boardEl}>
    {#each squares as index}
      {@const row = getRow(index)}
      {@const col = getColumn(index)}
      {@const isSelected = selectedSquareIndex === index}
      {@const lastMovedSquareFrom = lastRecord ? lastRecord.mv.from : null}
      {@const lastMovedSquareTo = lastRecord ? lastRecord.mv.to : null}
      {@const isLastMovedSquareFrom = lastMovedSquareFrom && lastMovedSquareFrom[0] === row && lastMovedSquareFrom[1] === col}
      {@const isLastMovedSquareTo = lastMovedSquareTo && lastMovedSquareTo[0] === row && lastMovedSquareTo[1] === col}

      {@const piece = boardState ? boardState.squares[row][col] : null}
      <button
        class="square {isDarkSquare(index) ? 'dark' : 'light'}"
        class:highlighted={isSelected || isLastMovedSquareFrom || isLastMovedSquareTo}
        onclick={() => handleSquareClick(index)}
        onpointerdown={(e) => handlePointerDown(e, index)}
        aria-label="{files[col]}{ranks[row]}"
      >
        {#if availableSquares?.some((s) => s.row == row && s.col == col)}
          <div class:available-square={!piece} class:capture-target={piece && piece.color != boardState?.active_color}></div>
        {/if}
        {#key slideId}
          {#if piece}
            {@const colorKey = piece.color === "white" ? "w" : "b"}
            {@const pieceKey = piece.piece_type === "knight" ? "n" : piece.piece_type.charAt(0)}
            {@const slide = slides.find((s) => s.to[0] === row && s.to[1] === col)}
            {@const drag = dragging?.from === index ? dragging : null}
            <div
              class="piece {piece.color}"
              class:sliding={!!slide}
              class:dragged={!!drag}
              style:--img="url(/pieces/{colorKey}{pieceKey}.png)"
              style:--dx="{slide ? (slide.from[1] - col) * 100 : 0}%"
              style:--dy="{slide ? (slide.from[0] - row) * 100 : 0}%"
              style:left={drag ? `${drag.x}px` : null}
              style:top={drag ? `${drag.y}px` : null}
              role="img"
              aria-label="{piece.color} {piece.piece_type}"
            ></div>
          {/if}
        {/key}
        {#if col == 0}
          <span class="rank-label" class:coordinate-dark={isDarkSquare(index)} class:coordinate-light={!isDarkSquare(index)}
            >{ranks[row]}</span
          >
        {/if}
        {#if row == 7}
          <span class="file-label" class:coordinate-dark={isDarkSquare(index)} class:coordinate-light={!isDarkSquare(index)}
            >{files[col]}</span
          >
        {/if}
      </button>
    {/each}
  </div>

  <div class="sidebar glass">
    <div class="sidebar-header">
      <h3>Moves</h3>
      <span class="status" class:alert={lastRecord?.game_state === "check" || lastRecord?.game_state === "checkmate"}>
        {status}
      </span>
    </div>

    <div class="history-list">
      {#if boardState?.move_history}
        {#each Array(Math.ceil(boardState.move_history.length / 2)) as _, i}
          {@const latest = boardState.move_history.length - 1}
          <div class="history-row">
            <div class="move-number">
              {i + 1}.
            </div>
            <button class="history-move" class:latest={latest === i * 2}>
              {formatMove(boardState.move_history[i * 2])}
            </button>
            {#if boardState.move_history[i * 2 + 1]}
              <button class="history-move" class:latest={latest === i * 2 + 1}>
                {formatMove(boardState.move_history[i * 2 + 1])}
              </button>
            {/if}
          </div>
        {/each}
      {/if}
    </div>

    <div class="controls">
      <div class="uci-input-group">
        <input
          type="text"
          placeholder="UCI moves (e2e4 e7e5...)"
          bind:value={uciInput}
          oninput={() => (uciError = "")}
          onkeydown={(e) => e.key === "Enter" && handlePlayUCI()}
        />
        <button class="glass glass-button accent" onclick={() => handlePlayUCI()}>Play</button>
      </div>
      {#if uciError}
        <p class="error">{uciError}</p>
      {/if}
      <button class="glass glass-button" onclick={() => handleUndo()}>Undo</button>
    </div>
  </div>
</div>

<style>
  .game-container {
    --board-size: min(72vmin, 720px);

    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 20px;
  }

  .chessboard {
    width: var(--board-size);
    height: var(--board-size);
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    grid-template-rows: repeat(8, 1fr);
    border-radius: 12px;
    touch-action: none;
    overflow: hidden;
  }

  .square {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    user-select: none;
    border: none;
    padding: 0;
    margin: 0;
  }

  .square:has(.piece) {
    cursor: grab;
  }

  .dragging,
  .dragging .square {
    cursor: grabbing;
  }

  .light {
    background-color: var(--square-light);
  }

  .dark {
    background-color: var(--square-dark);
  }

  /* selected square and last move */
  .square.highlighted::before {
    content: "";
    position: absolute;
    inset: 0;
    background-color: var(--highlight-move);
    opacity: 0.55;
  }

  .available-square {
    position: absolute;
    width: 28%;
    height: 28%;
    border-radius: 25%;
    background-color: var(--capture-target);
    opacity: 0.5;
    z-index: 6;
  }

  .capture-target {
    border: 5px solid var(--capture-target);
    opacity: 0.25;
    z-index: 6;
    border-radius: 25%;
    position: absolute;
    inset: 2px;
  }

  .piece {
    width: 100%;
    height: 100%;
    background: var(--img) center / contain no-repeat;
    mask: var(--img) center / contain no-repeat;
  }

  .piece.dragged {
    position: fixed;
    width: calc(var(--board-size) / 8);
    height: calc(var(--board-size) / 8);
    transform: translate(-50%, -50%);
    pointer-events: none;
    z-index: 100;
  }

  .piece.sliding {
    z-index: 1;
    animation: slide 0.18s ease-out;
  }

  @keyframes slide {
    from {
      transform: translate(var(--dx), var(--dy));
    }
  }

  .file-label,
  .rank-label {
    position: absolute;
    font-size: 0.7rem;
    font-weight: 600;
  }

  .file-label {
    bottom: 2px;
    right: 5px;
  }

  .rank-label {
    top: 3px;
    left: 5px;
  }

  .coordinate-light {
    color: var(--square-dark);
  }

  .coordinate-dark {
    color: var(--square-light);
  }

  .sidebar {
    width: 300px;
    height: calc(var(--board-size));
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 16px 18px 12px;
  }

  h3 {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 600;
  }

  .status {
    font-size: 0.75rem;
    color: var(--text-muted);
    padding: 4px 10px;
    border-radius: 25px;
    background: rgba(255, 255, 255, 0.06);
    white-space: nowrap;
  }

  .status.alert {
    color: var(--accent);
    background: var(--accent-soft);
  }

  .history-list {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 0 10px;
    scrollbar-width: thin;
  }

  .history-row {
    display: grid;
    grid-template-columns: 28px 1fr 1fr;
    align-items: center;
    padding: 2px 8px;
    border-radius: 10px;
    font-variant-numeric: tabular-nums;
  }

  .history-row:nth-child(odd) {
    background: rgba(255, 255, 255, 0.03);
  }

  .move-number {
    color: var(--text-muted);
    font-size: 0.85rem;
  }

  .history-move {
    font: inherit;
    font-size: 0.9rem;
    color: var(--text);
    background: transparent;
    border: none;
    border-radius: 8px;
    text-align: left;
  }

  .history-move.latest {
    color: var(--accent);
    background: var(--accent-soft);
    font-weight: 600;
  }

  .controls {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 14px;
    border-top: 1px solid var(--hairline);
  }

  .uci-input-group {
    display: flex;
    gap: 8px;
  }

  input {
    flex: 1;
    min-width: 0;
    font: inherit;
    font-size: 0.875rem;
    color: var(--text);
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--hairline);
    border-radius: 12px;
    padding: 9px 12px;
    outline: none;
    transition:
      border-color 0.2s,
      box-shadow 0.2s;
  }

  input::placeholder {
    color: var(--text-muted);
  }

  input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-soft);
  }

  .error {
    margin: 0;
    font-size: 0.8rem;
    color: #ff6b5e;
  }
</style>
