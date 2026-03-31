<script>
	import '../app.css';

	let { children } = $props();
</script>

<div class="app">
	<nav class="sidebar">
		<div class="logo">
			<h1>Abaco</h1>
		</div>
		<ul class="nav-links">
			<li><a href="/">Dashboard</a></li>
			<li><a href="/accounts">Plan de cuentas</a></li>
			<li><a href="/journal">Libro diario</a></li>
			<li><a href="/ledger">Libro mayor</a></li>
			<li><a href="/balance">Balance</a></li>
		</ul>
	</nav>
	<main class="content">
		{@render children()}
	</main>
</div>

<style>
	.app {
		display: grid;
		grid-template-columns: 240px 1fr;
		min-height: 100vh;
	}

	.sidebar {
		background-color: var(--color-primary);
		color: white;
		padding: 1.5rem 0;
	}

	.logo {
		padding: 0 1.5rem 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.logo h1 {
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: 0.05em;
	}

	.nav-links {
		list-style: none;
		padding: 1rem 0;
	}

	.nav-links a {
		display: block;
		padding: 0.625rem 1.5rem;
		color: rgba(255, 255, 255, 0.7);
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 500;
		transition: color 0.15s, background-color 0.15s;
	}

	.nav-links a:hover {
		color: white;
		background-color: rgba(255, 255, 255, 0.05);
	}

	.content {
		padding: 2rem;
		overflow-y: auto;
	}
</style>
