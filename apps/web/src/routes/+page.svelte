<svelte:head>
	<title>Abaco — Dashboard</title>
</svelte:head>

<div class="dashboard">
	<h2>Dashboard</h2>
	<div class="cards">
		<div class="card">
			<span class="card-label">Activos</span>
			<span class="card-value">—</span>
		</div>
		<div class="card">
			<span class="card-label">Pasivos</span>
			<span class="card-value">—</span>
		</div>
		<div class="card">
			<span class="card-label">Patrimonio</span>
			<span class="card-value">—</span>
		</div>
		<div class="card">
			<span class="card-label">Resultado</span>
			<span class="card-value">—</span>
		</div>
	</div>
</div>

<style>
	.dashboard h2 {
		margin-bottom: 1.5rem;
	}

	.cards {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}

	.card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 1.25rem;
		box-shadow: var(--shadow-sm);
	}

	.card-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		margin-bottom: 0.5rem;
	}

	.card-value {
		font-size: 1.5rem;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}
</style>
