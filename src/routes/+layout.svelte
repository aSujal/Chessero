<script lang="ts">
  import "../app.css";
  import { page } from "$app/state";

  let { children } = $props();

  const links = [{ href: "/", label: "Home" }];
</script>

<svelte:head>
  <link rel="icon" href="/svelte.svg" />
</svelte:head>

<header class="topbar glass">
  <span class="logo">Chessero</span>
  <nav>
    {#each links as link}
      <a href={link.href} class:active={page.url.pathname === link.href}>{link.label}</a>
    {/each}
  </nav>
</header>

<!-- older method or something... <slot/> -->
{@render children?.()}

<style>
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin: 16px 20px 0;
    padding: 8px 8px 8px 18px;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
    letter-spacing: -0.01em;
  }

  nav {
    display: flex;
    gap: 4px;
  }

  nav a {
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.875rem;
    padding: 6px 12px;
    border-radius: 10px;
    transition:
      color 0.2s,
      background 0.2s;
  }

  nav a:hover {
    color: var(--text);
  }

  nav a.active {
    color: var(--accent);
    background: var(--accent-soft);
  }
</style>
