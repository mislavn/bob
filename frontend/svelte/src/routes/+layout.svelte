<script>
  import '../app.css';
</script>

<header class="flex h-20 items-center border-b border-solid border-slate-200 px-4">
  <a href="/">BOB</a>
</header>
<div class="flex h-[calc(100vh-theme(space.20))] items-center justify-center">
  <slot />
</div>
