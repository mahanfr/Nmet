<script>
    import Logo from '$lib/assets/logo.svg'
</script>
<svelte:head>
    <title>Nmet Programmin Language</title>
	<meta name="robots" content="noindex nofollow" />
</svelte:head>
<header>
    <nav class="flex flex-row items-center gap-5 py-5 px-20">
        <img src={Logo} alt="Nemt programming language" width="90px" />
        <a class="text-gray-200" href="https://www.github.com">Getting Started</a>
        <a class="text-gray-200" href="https://www.github.com">Documents</a>
        <a class="text-gray-200" href="https://www.github.com">Download</a>
        <a class="text-gray-200" href="https://github.com/mahanfarzaneh2000/Nmet">Github</a>
        <a class="text-gray-200" href="https://www.github.com">About</a>
    </nav>
</header>
<section class="flex gap-10 flex-row bg-primary justify-between px-20 py-5">
    <div class="w-2/3 text-gray-200 text-xl">
        <p class="font-light">
            <span class="font-bold">Nemet</span>
            <span>is a general purpose</span>
            <span class="font-bold">staticly typed</span>
            <span>and</span>
            <span class="font-bold"> compiled </span>
            <span>programming Language that generates x86-64 assembly as Intermediate representation (IR) which can be compiled to binary using nasm. We eventialy will move away from nasm and implement our own loader but the current goal is to become self hosted by writing the compiler in itself!</span>
        </p>
        <div class="flex flex-row gap-4 mt-5">
            <a class="bg-accent text-gray-800 font-bold text-base border-2 p-2 rounded-lg border-bg" href="">Getting Started</a>
            <a class="bg-secondary text-gray-800 font-bold text-base border-2 p-2 rounded-lg border-bg" href="">Documentation</a>
        </div>
    </div>
    <div class="w-1/3">
        <div class="bg-bg py-1 px-3">
            <p class="text-gray-400">hello_world.nmt</p>
        </div>
        <div class="bg-bg-darker text-gray-200 p-5">
            <pre>{'func main() {\n\tprint "Hello World!\\n";\n}'}</pre>
        </div>
    </div>
</section>
<section class="px-20 py-5 text-gray-200">
    <h2 class="font-semibold text-2xl">Getting Started<h2>
</section>
