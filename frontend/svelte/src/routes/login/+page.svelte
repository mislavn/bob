<script>
  import Input from '../../components/input.svelte';
</script>

<div class="flex flex-1 flex-col justify-center">
  <div class="">
    <h2 class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
      Sign in to your account
    </h2>
  </div>

  <div class="mt-10 px-5 sm:mx-auto sm:w-full sm:max-w-sm sm:px-0">
    <form method="POST" action="#" class="space-y-3">
      <Input label="Email address" id="email" />
      <Input label="Password" id="password" />

      <button
        type="submit"
        class="mt-2 flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
      >
        Sign in
      </button>
    </form>
  </div>
</div>
