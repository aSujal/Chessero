<script lang="ts">
  import { get } from "svelte/store";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let boardState: BoardState | null = $state(null);
  let selectedSquareIndex: number | null = $state(null);
  let availableSquares: { row: number; col: number }[] | null = $state(null);

  onMount(async () => {
    const response: BoardState = await invoke("get_initial_board");
    console.log(response);
    boardState = response;
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

  function getMatrixCoords(index: number): string {
    const row = Math.floor(index / 8);
    const col = index % 8;
    return `[${row}][${col}]`;
  }

  function coordsToNotation(row: number, col: number): string {
    return `${files[col]}${ranks[row]}`;
  }

  function formatMove(moveItem: UndoRecord): string {
    const { mv, moved_piece, captured_piece } = moveItem;
    const [_, fromCol] = mv.from;
    const [toRow, toCol] = mv.to;

    const pieceType = moved_piece.piece_type;

    let piecePrefix = "";
    if (pieceType !== "pawn") {
      piecePrefix = pieceType === "knight" ? "N" : pieceType.charAt(0).toUpperCase();
    }

    if (pieceType == "king" && fromCol == 4) {
      if (toCol == 6) {
        return "O-O";
      }
      if (toCol == 2) {
        return "O-O-O";
      }
    }
    const isCapture = captured_piece !== null;
    const toNotation = coordsToNotation(toRow, toCol);

    if (pieceType === "pawn") {
      if (isCapture) {
        return `${files[fromCol]}x${toNotation}`;
      }
      return toNotation;
    }

    return `${piecePrefix}${isCapture ? "x" : ""}${toNotation}`;
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

    //Handle move
    if (selectedSquareIndex !== null && availableSquares?.some((s) => s.row === clickedRow && s.col === clickedCol)) {
      const fromRow = getRow(selectedSquareIndex);
      const fromCol = getColumn(selectedSquareIndex);

      const updatedBoard: BoardState = await invoke("make_move", {
        board: boardState,
        fromRow: fromRow,
        fromCol: fromCol,
        toRow: clickedRow,
        toCol: clickedCol,
      });

      boardState = updatedBoard;
      console.log(updatedBoard);
      selectedSquareIndex = null;
      availableSquares = null;
      return;
    }

    if (clickedPiece) {
      selectedSquareIndex = index;
      getAvailableSquares(index);
    } else {
      selectedSquareIndex = null;
      availableSquares = null;
    }
  }

  async function handleUndo(index?: number) {
    if (!boardState) return;
    console.log("boardState", boardState?.move_history?.length);
    console.log("target_index", index);
    const updatedBoard: BoardState = await invoke("undo_move", {
      board: boardState,
      target_index: index,
    });

    boardState = updatedBoard;
    selectedSquareIndex = null;
    availableSquares = null;
  }
</script>

<div class="game-container">
  <div class="chessboard">
    {#each squares as index}
      {@const row = getRow(index)}
      {@const col = getColumn(index)}
      {@const isSelected = selectedSquareIndex === index}
      {@const lastMovedSquareFrom = boardState?.move_history?.length
        ? boardState.move_history[boardState.move_history.length - 1].mv.from
        : null}
      {@const lastMovedSquareTo = boardState?.move_history?.length
        ? boardState.move_history[boardState.move_history.length - 1].mv.to
        : null}
      {@const isLastMovedSquareFrom = lastMovedSquareFrom && lastMovedSquareFrom[0] === row && lastMovedSquareFrom[1] === col}
      {@const isLastMovedSquareTo = lastMovedSquareTo && lastMovedSquareTo[0] === row && lastMovedSquareTo[1] === col}

      {@const piece = boardState ? boardState.squares[row][col] : null}
      <button
        class="square {isDarkSquare(index) ? 'dark' : 'light'}"
        class:highlighted={isSelected || isLastMovedSquareFrom || isLastMovedSquareTo}
        onclick={() => handleSquareClick(index)}
        aria-label="{files[col]}{ranks[row]}"
      >
        <!-- <span>
        {getChessNotation(index)}
        </span> -->
        <!-- <span class="matrix">{getMatrixCoords(index)}</span> -->
        <!-- <span class="matrix">{index}</span> -->
        {#if availableSquares?.some((s) => s.row == row && s.col == col)}
          <div class:available-square={!piece} class:capture-target={piece && piece.color != boardState?.active_color}></div>
        {/if}
        {#if piece}
          {@const colorKey = piece.color === "white" ? "w" : "b"}
          {@const pieceKey = piece.piece_type === "knight" ? "n" : piece.piece_type.charAt(0)}
          <img src="/pieces/{colorKey}{pieceKey}.png" alt="{colorKey}{pieceKey}" class="piece-img" />
        {/if}
        {#if getColumn(index) == 0}
          <span id="ranks" class:coordinate-dark={isDarkSquare(index)} class:coordinate-light={!isDarkSquare(index)}
            >{ranks[getRow(index)]}</span
          >
        {/if}
        {#if getRow(index) == 7}
          <span id="files" class:coordinate-dark={isDarkSquare(index)} class:coordinate-light={!isDarkSquare(index)}
            >{files[getColumn(index)]}</span
          >
        {/if}
      </button>
    {/each}
  </div>

  <div class="sidebar">
    <div class="controls">
      <button onclick={() => handleUndo()}>Undo</button>
    </div>
    <div class="history-container">
      <h3>White - Black</h3>

      <div class="history-list">
        {#if boardState?.move_history}
          {#each Array(Math.ceil(boardState.move_history.length / 2)) as _, i}
            <div class="history-row">
              <div class="move-number">
                {i + 1}.
              </div>
              <button class="history-move">
                {formatMove(boardState.move_history[i * 2])}
              </button>
              {#if boardState.move_history[i * 2 + 1]}
                <button class={`history-move`}>{formatMove(boardState.move_history[i * 2 + 1])}</button>
              {/if}
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  * {
    box-sizing: border-box;
  }

  :root {
    --board-border-radius: 5px;
  }

  .game-container {
    display: flex;
    gap: 20px;
  }

  .sidebar,
  .chessboard {
    width: 75vmin;
    height: 75vmin;
    max-width: 800px;
    max-height: 800px;
  }

  .chessboard {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    grid-template-rows: repeat(8, 1fr);
  }

  .sidebar {
    background-color: #0000002e;
    border-radius: var(--board-border-radius);

    display: flex;
    flex-direction: column;
  }

  .history-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .history-list {
    padding-right: 5px;
    overflow-y: auto;
  }

  .history-row {
    display: grid;
    grid-template-columns: 20px 1fr 1fr;
    align-items: center;
    padding-inline: 20px;
  }

  .history-row:nth-child(even) {
    background: #0000001e;
  }
  .history-row:nth-child(odd) {
    background: #0000004e;
  }

  .history-move {
    background: transparent;
    color: white;
    border: none;

    padding: 8px;
    text-align: left;
  }

  #files {
    position: absolute;
    bottom: 0;
    right: 5px;
  }

  #ranks {
    position: absolute;
    top: 0;
    left: 5px;
  }

  /* .matrix {
    font-size: 0.7rem;
    color: rgba(0, 0, 0, 0.45);
  }
  */

  .piece-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    z-index: 2;
  }

  .coordinate-light {
    color: #739552;
  }

  .coordinate-dark {
    color: #eeeed2;
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
  /* highlight selected square */
  .square.highlighted::before {
    background-color: #ffff33;
    z-index: 1;
    opacity: 0.5;
    content: "";
    width: 100%;
    height: 100%;
    position: absolute;
  }

  .available-square {
    width: 30%;
    height: 30%;
    background-color: black;
    opacity: 0.2;
    z-index: 6;
    border-radius: 50%;
    position: absolute;
  }

  .capture-target {
    border: 5px solid black;
    opacity: 0.2;
    z-index: 6;
    border-radius: 50%;
    position: absolute;
    top: 0px;
    bottom: 0px;
    right: 0px;
    left: 0px;
  }

  /* corners */
  .square:first-of-type {
    border-top-left-radius: var(--board-border-radius);
  }

  .square:nth-child(8) {
    border-top-right-radius: var(--board-border-radius);
  }

  .square:nth-child(57) {
    border-bottom-left-radius: var(--board-border-radius);
  }

  .square:last-of-type {
    border-bottom-right-radius: var(--board-border-radius);
  }

  /* square colors */

  .light {
    background-color: #eeeed2;
  }

  .dark {
    background-color: #769656;
  }
</style>
