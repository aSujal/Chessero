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

  function getChessNotation(index: number): string {
    const row = getRow(index);
    const col = getColumn(index);

    return `${files[col]}${ranks[row]}`;
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
    console.log("currentRow", row);
    console.log("currentCol", col);
    console.log("availableSquares", availableSquares);
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
</script>

<div class="chessboard">
  {#each squares as index}
    {@const row = getRow(index)}
    {@const col = getColumn(index)}
    {@const isSelected = selectedSquareIndex === index}
    {@const piece = boardState ? boardState.squares[row][col] : null}
    <button
      class="square {isDarkSquare(index) ? 'dark' : 'light'}"
      class:selected={isSelected}
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

<style>
  :root {
    --board-border-radius: 5px;
  }
  .chessboard {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    grid-template-rows: repeat(8, 1fr);
    width: 75vmin;
    height: 75vmin;
    max-width: 650px;
    max-height: 650px;
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

  .matrix {
    font-size: 0.7rem;
    color: rgba(0, 0, 0, 0.45);
  }

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
  .square.selected::before {
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
