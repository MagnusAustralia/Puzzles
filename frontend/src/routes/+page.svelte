<script lang="ts">
	import { onMount } from 'svelte';

	let x: number = 5;
	let y: number = 5;
	let nonogramSolved: boolean = false;

	let rowHints: number[][] = [];
	let colHints: number[][] = [];

	let gridState: (boolean | 'X' | null)[][] = [];
	let isMouseDown = false;
	let currentButton: number | null = null;

	const resizeGrid = () => {
		const newGridState = Array.from({ length: y + 1 }, (_, rowIndex) => {
			return Array.from({ length: x }, (_, colIndex) => {
				if (gridState[rowIndex] && gridState[rowIndex][colIndex] !== undefined) {
					return gridState[rowIndex][colIndex];
				}
				return null;
			});
		});
		gridState = newGridState;
	};

	$: x, y, resizeGrid();

	const toggleBox = (row: number, col: number, button: number): void => {
		if (button === 0) {
			gridState[row][col] = gridState[row][col] === true ? null : true;
		} else if (button === 2) {
			gridState[row][col] = gridState[row][col] === 'X' ? null : 'X';
		}
	};

	const handleMouseDown = (event: MouseEvent, row: number, col: number) => {
		isMouseDown = true;
		currentButton = event.button; // 0 for left click, 2 for right click
		toggleBox(row, col, currentButton);

		// Add a global mouseup listener to capture the event even when outside the grid
		window.addEventListener('mouseup', handleMouseUp);
	};

	const handleMouseEnter = (row: number, col: number) => {
		if (isMouseDown && currentButton !== null) {
			toggleBox(row, col, currentButton);
		}
	};

	const handleMouseUp = () => {
		isMouseDown = false;
		currentButton = null;

		// Remove the global mouseup listener
		window.removeEventListener('mouseup', handleMouseUp);
	};

	const handleContextMenu = (event: MouseEvent) => {
		event.preventDefault();
	};

	const generateNewNonogram = async () => {
		let response: Response;
		try {
			response = await fetch(`http://127.0.0.1:3000/generate_nonogram?x=${x}&y=${y}`, {
				method: 'GET'
			});
			let hints = await response.json();
			console.log(hints);
			rowHints = hints.x_hints;
			colHints = hints.y_hints;
			gridState = [];
			resizeGrid();
		} catch (error) {
			console.error('Error generating new nonogram:', error);
		}
	};

	const checkNonogram = () => {
		let isValid = true;
		// Check Rows
		for (let index = 1; index < gridState.length; index++) {
			const column = gridState[index];
			let lines: number[] = [];
			let current = false;
			for (let j = 0; j < column.length; j++) {
				const element = column[j];
				if (element === true) {
					if (current) {
						lines[lines.length - 1] = lines[lines.length - 1] + 1;
					} else {
						lines.push(1);
						current = true;
					}
				} else {
					current = false;
				}
			}
			if (lines.toString() !== rowHints[index - 1].toString()) {
				isValid = false;
			}
		}

		// Check Columns
		for (let colIndex = 0; colIndex < gridState[0].length; colIndex++) {
			let lines: number[] = [];
			let current = false;

			for (let rowIndex = 0; rowIndex < gridState.length; rowIndex++) {
				const element = gridState[rowIndex][colIndex];

				if (element === true) {
					if (current) {
						lines[lines.length - 1] = lines[lines.length - 1] + 1;
					} else {
						lines.push(1);
						current = true;
					}
				} else {
					current = false;
				}
			}

			if (lines.toString() !== colHints[colIndex].toString()) {
				isValid = false;
			}
		}

		if (isValid) {
			nonogramSolved = true;
		}
	};

	onMount(() => {
		generateNewNonogram();
	});
</script>

<section class="flex flex-col items-center justify-center space-y-5">
	<h1 class="mt-4 text-xl">Nonogram</h1>
	<div class="flex flex-row space-x-5">
		<label>
			Columns:
			<input type="range" min="5" max="15" bind:value={x} />
			<br />
			{x}
		</label>
		<label>
			Rows:
			<input type="range" min="5" max="15" bind:value={y} />
			<br />
			{y}
		</label>
	</div>

	{#if !nonogramSolved}
		<div class="grid-container flex flex-col items-center space-y-5">
			<div>
				<div class="grid">
					{#each gridState as row, rowIndex}
						<div class="row flex items-center">
							<div class="row-hint mr-2 flex flex-row justify-end space-x-5">
								{#each rowHints[rowIndex - 1] as hintPart}
									<span class="text-right">{hintPart}</span>
								{/each}
							</div>
							{#if rowIndex === 0}
								{#each rowHints.slice(0, x) as hint, _}
									<div class="col-hint flex flex-col items-center justify-end">
										{#each hint as hintPart}
											<span class="hint-part">{hintPart}</span>
										{/each}
									</div>
								{/each}
							{:else}
								{#each row as cell, colIndex}
									<button
										type="button"
										aria-label={`Row ${rowIndex + 1}, Col ${colIndex + 1}`}
										class="box"
										on:mousedown={(event) => handleMouseDown(event, rowIndex, colIndex)}
										on:mouseenter={() => handleMouseEnter(rowIndex, colIndex)}
										on:contextmenu={handleContextMenu}
										style="
											background-color: {cell === true ? 'black' : 'white'};
											border-left-width: {colIndex % 5 === 0 ? (colIndex === 0 ? '3px' : '1.5px') : '1px'};
											border-right-width: {(colIndex + 1) % 5 === 0 ? '3px' : '1px'};
											border-top-width: {rowIndex === 1 ? '3px' : '1px'};
											border-bottom-width: {rowIndex % 5 === 0 ? '3px' : '1px'};
										"
									>
										{#if cell === 'X'}
											<span class="x-symbol">X</span>
										{/if}
									</button>
								{/each}
							{/if}
						</div>
					{/each}
				</div>
			</div>
			<div class="">
				<button
					class="rounded-lg bg-blue-500 p-2 font-bold text-white"
					on:click={() => checkNonogram()}>Check Nonogram</button
				>
				<br />
				<button
					class="rounded-lg bg-red-500 p-2 font-bold text-white"
					on:click={() => generateNewNonogram()}>Regenerate Nonogram</button
				>
			</div>
		</div>
	{:else}
		<h1>Solved</h1>
	{/if}
</section>

<style>
	.grid-container {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.col-hint {
		display: flex;
		flex-direction: column;
		justify-content: end; /* Align items to the bottom */
		align-items: center; /* Center items horizontally */
		width: 50px; /* Ensure consistent width */
		min-height: 50px; /* Set a minimum height if needed */
	}

	.hint-part {
		margin: 2px 0; /* Add a bit of spacing between the hint parts */
		text-align: center;
		font-weight: bold;
	}

	.row-hint {
		min-width: 50px;
		text-align: right;
		margin-right: 10px;
		font-weight: bold;
	}

	.box {
		width: 50px;
		height: 50px;
		border: 1px solid black;
		box-sizing: border-box;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 24px;
	}

	.x-symbol {
		color: red;
		font-weight: bold;
	}
</style>
