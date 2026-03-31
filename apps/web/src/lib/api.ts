const API_BASE = import.meta.env.VITE_API_URL ?? 'http://localhost:3000';

export async function api<T>(path: string, options?: RequestInit): Promise<T> {
	const res = await fetch(`${API_BASE}${path}`, {
		headers: {
			'Content-Type': 'application/json',
			...options?.headers
		},
		...options
	});

	if (!res.ok) {
		const text = await res.text();
		throw new Error(text || res.statusText);
	}

	return res.json();
}
