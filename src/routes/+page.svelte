<script>
	import Highlight from 'svelte-highlight';
	import rust from 'svelte-highlight/languages/rust';

	//language=rust
	import code from '$lib/example.rs?raw';

	// Custom styling instead of importing a theme
</script>

<div class="title">
	<img src="/logo.png" alt="hyperion logo" width="200" />
	<h1>HYPERION <span class="alpha-tag">ALPHA</span></h1>
	<div class="tagline">
		super fast minecraft UGC with <a href="https://github.com/SanderMertens/flecs">flecs</a>
	</div>
</div>

<video autoplay width="1000" muted loop>
	<source src="/hyperion.webm" type="video/webm" />
</video>

<h4>Minecraft protocol; your own custom logic.</h4>
<div class="tagline">here is how to implement regen</div>

<div class="code-display">
	<Highlight language={rust} {code} />
</div>

<a href="https://github.com/hyperion-mc/hyperion/tree/main/events/tag">a complete custom event</a>
<a href="https://discord.gg/WKBuTXeBye">join the discord</a>

<style>
	video {
		margin-bottom: 40px;
	}

	.title {
		margin-bottom: 40px;
		position: relative;
		text-align: center;
	}

	h1 {
		/*font-size: 64px;*/
		letter-spacing: 2px;
		margin: 0;
		color: #fff;
		position: relative;
		/* text-shadow:
							0 0 5px rgba(0, 255, 0, 0.3),
							0 0 10px rgba(0, 255, 0, 0.2); */
		/*position: relative;*/
	}

	.alpha-tag {
		font-size: 0.4em;
		background-color: var(--glow-green, #00ff00);
		color: #000;
		padding: 2px 6px;
		border-radius: 3px;
		vertical-align: middle;
		display: inline-block;
		margin-left: -15px;
		margin-top: -10px;
		letter-spacing: 1px;
		font-weight: bold;
		opacity: 0.8;
	}

	.tagline {
		color: var(--text-secondary);
		font-size: 16px;
		/*margin-top: 5px;*/
		letter-spacing: 1px;
	}

	.code-display {
		margin-top: 20px;
		margin-bottom: 20px;
		width: 100%;
		max-width: 800px;
		background-color: var(--code-bg);
		border-left: 2px solid var(--glow-green);
		padding: 15px 20px;
		box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
	}

	/* JetBrains UI Dark theme for highlight.js */
	:global(.hljs) {
		background-color: var(--code-bg) !important;
		color: #a9b7c6 !important;
		padding: 0 !important;
	}

	:global(.hljs-keyword) {
		color: #cc7832 !important;
		font-weight: bold;
	}

	:global(.hljs-function) {
		color: #ffc66d !important;
	}

	:global(.hljs-title) {
		color: #ffc66d !important;
	}

	:global(.hljs-params) {
		color: #a9b7c6 !important;
	}

	:global(.hljs-string) {
		color: #6a8759 !important;
	}

	:global(.hljs-comment) {
		color: #808080 !important;
		font-style: italic;
	}

	:global(.hljs-number) {
		color: #6897bb !important;
	}

	:global(.hljs-built_in) {
		color: #cc7832 !important;
	}

	:global(.hljs-literal) {
		color: #cc7832 !important;
	}

	:global(.hljs-type) {
		color: #aabbcc !important;
	}

	:global(.hljs-attribute) {
		color: #9876aa !important;
	}

	:global(.hljs-symbol) {
		color: #6a8759 !important;
	}

	:global(.hljs-meta) {
		color: #bbb529 !important;
	}

	:global(.hljs-name) {
		color: #e8bf6a !important;
	}
</style>
