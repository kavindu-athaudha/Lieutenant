<script lang="ts">
	import hljs from 'highlight.js';
	import 'highlight.js/styles/github.css'; // Get different styles here: https://highlightjs.org/examples
	import { onMount } from 'svelte';
	import { FileCopyAltOutline } from 'flowbite-svelte-icons';

	export let markdownText = '';

	function applyFormatting(rawHTMLContent: string) {
		rawHTMLContent = rawHTMLContent.replace(/<\/pre>/g, '</pre></div><br/>');
		rawHTMLContent = rawHTMLContent.replace(/<\/table>/g, '</table> <br/>');
		rawHTMLContent = rawHTMLContent.replace(/<thead>/g, '<thead style="background: #ccc;">');
		rawHTMLContent = rawHTMLContent.replace(/<th>/g, '<th style="padding: 0.5em">');
		rawHTMLContent = rawHTMLContent.replace(/<td>/g, '<td style="padding: 0.5em">');
		rawHTMLContent = rawHTMLContent.replace(
			/<table>/g,
			'<br/> <table style="border: 1px solid var(--primary-color);">'
		);
		rawHTMLContent = rawHTMLContent.replace(
			/<h1>/g,
			'<font-weight: bold; h1 style="font-size: 2em; padding-top: 2em;">'
		);
		rawHTMLContent = rawHTMLContent.replace(
			/<h2>/g,
			'<h2 style="font-weight: bold; font-size: 1.75em; padding-top: 2em;">'
		);
		rawHTMLContent = rawHTMLContent.replace(
			/<h3>/g,
			'<h3 style="font-weight: bold; padding-top: 2em">'
		);
		rawHTMLContent = rawHTMLContent.replace(/\[\^(\d+)\]/g, '<sup>[$1]</sup>');
		rawHTMLContent = rawHTMLContent.replace(
			/<a\s+([^>]*href="[^"]*")>/g,
			'<a $1 style="color: var(--primary-color); text-decoration: underline; text-underline-offset: 3px;">'
		);

		rawHTMLContent = rawHTMLContent.replace(
			/<blockquote>/g,
			'<blockquote style="font-style: italic; background-color: #f9f9f9; padding: 10px 20px; border-left: 5px solid #ccc;">'
		);
		rawHTMLContent = rawHTMLContent.replace(
			/<li>/g,
			'<li style="margin-bottom: 10px; padding-left: 20px; list-style-type: disc; list-style-position: inside;">'
		);
		rawHTMLContent = rawHTMLContent.replace(
			/<img\s+([^>]*)>/g,
			'<img $1 style="max-height: 20vh; width: auto;">'
		);
		rawHTMLContent = rawHTMLContent.replace(
			/(?<!<pre>)<code>/g,
			'<code style="background: white; padding: 0.2em; border: 1px solid #AAAAAA; border-radius: 5px; display: inline-block; font-size: 0.9em">'
		);
		rawHTMLContent = rawHTMLContent.replace(
			/<pre([^>]*)>\s*<code([^>]*)>/g,
			'<div class="CodeBlock"><pre$1 style="font-size: 0.9em;"><code$2>'
		);

		return rawHTMLContent;
	}

	function highlightCodeBlocks(rawHTMLContent: string) {
		const parser = new DOMParser();
		const doc = parser.parseFromString(rawHTMLContent, 'text/html');
		const codeBlocks = doc.querySelectorAll('pre code');

		codeBlocks.forEach((block) => {
			const language = block.className.replace('language-', '');
			try {
				if (language) {
					block.innerHTML = hljs.highlight(language, block.textContent || '').value;
				} else {
					block.innerHTML = hljs.highlightAuto(block.textContent || '').value;
				}
			} catch (error) {
				console.warn(`Could not find the language '${language}', using auto-detection.`);
				block.innerHTML = hljs.highlightAuto(block.textContent || '').value;
			}
		});

		return doc.body.innerHTML;
	}

	function addFloatingButtons() {
		const codeBlocks = document.querySelectorAll('.CodeBlock');
		codeBlocks.forEach((block) => {
			const button = document.createElement('button');
			button.classList.add('floating-button');

			const icon = new FileCopyAltOutline({ target: button });

			button.addEventListener('click', () => {
				const code = block.querySelector('code')?.textContent;
				if (code) {
					navigator.clipboard.writeText(code);
				}
			});

			block.style.position = 'relative';
			block.appendChild(button);
		});
	}

	let rawHTMLContent = marked.parse(markdownText);
	let formattedHTMLContent = applyFormatting(rawHTMLContent);
	let codeHighlightedAndFormattedHTMLContent = highlightCodeBlocks(formattedHTMLContent);

	onMount(() => {
		addFloatingButtons();
	});
</script>

{@html codeHighlightedAndFormattedHTMLContent}

<style>
	:global(.CodeBlock) {
		position: relative;
	}
	:global(.floating-button) {
		position: absolute;
		top: 0.5em;
		right: 0.5em;
		background-color: var(--primary-color);
		color: white;
		border: none;
		border-radius: 5px;
		font-size: 0.25em; /* Reduce font size to 25% */
		cursor: pointer;
		transition: transform 0.1s; /* Change animation time to 0.1s */
		opacity: 25%;
	}
	:global(.floating-button:hover) {
		transform: scale(1.5);
		opacity: 100%;
	}
</style>
