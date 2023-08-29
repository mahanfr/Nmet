<script>
    import Logo from '$lib/assets/logo.svg'
</script>
<svelte:head>
    <title>Nmet Programming Language</title>
	<meta name="robots" content="noindex nofollow" />
</svelte:head>
<header>
    <nav class="flex flex-row items-center gap-5 py-5 px-10 md:px-20">
        <img src={Logo} alt="Nemt programming language" width="90px" />
        <a class="hidden md:block text-gray-200" href="#Getting-Started">Getting Started</a>
        <a class="hidden md:block text-gray-200" href="#Guids">Guids</a>
        <a class="hidden md:block text-gray-200" href="https://github.com/mahanfarzaneh2000/Nmet">Github</a>
        <a class="hidden md:block text-gray-200" href="#About">About</a>
    </nav>
</header>
<section id="About" class="flex gap-10 flex-col md:flex-row bg-primary justify-between px-10 md:px-20 py-10">
    <div class="w-100 md:w-2/3 text-gray-200 text-lg">
        <p class="font-light">
            <span class="font-bold">Nmet</span>
            <span>is a general purpose</span>
            <span class="font-bold">statically typed</span>
            <span>and</span>
            <span class="font-bold"> compiled </span>
            <span>
                programming Language that generates x86-64 assembly as Intermediate representation (IR) 
                which can be compiled to binary using nasm. 
                We eventually will move away from nasm and implement our own loader 
                but the current goal is to become self hosted by writing the compiler in itself!
            </span>
        </p>
        <div class="flex flex-row gap-4 mt-5">
            <a class="bg-accent text-gray-800 font-bold text-base border-2 p-2 rounded-lg border-bg" href="#Getting-Started">Getting Started</a>
            <a class="bg-secondary text-gray-800 font-bold text-base border-2 p-2 rounded-lg border-bg" href="#Guids">Documentation</a>
        </div>
    </div>
    <div class="w-100 md:w-1/3">
        <div class="bg-bg py-1 px-3">
            <p class="text-gray-400">hello_world.nmt</p>
        </div>
        <div class="bg-bg-darker text-gray-200 p-5">
            <pre>{'func main() {\n\tprint "Hello World!\\n";\n}'}</pre>
        </div>
    </div>
</section>
<section id="Getting-Started" class="px-10 md:px-20 py-10 text-gray-200">
    <h2 class="font-semibold text-2xl">Getting Started<h2>
    <p class="font-thin text-lg mt-4">
        The Project is in development state and dose not come with a binary package yet 
        and It is Only available for Linux but you can use WSL in windows!
    </p>
    <p class="font-thin text-lg">
       For getting started clone the 
       <a href="https://github.com/mahanfarzaneh2000/Nmet" class="text-blue-400">repository</a>
       and build the project using Rust toolchain
       Install 
       <a href="https://www.nasm.org/" class="text-blue-400">Nasm</a>
       using your package manager or by downloading it from it's official website.
    </p>
    <p class="font-thin text-lg">
        For installing in Gnu/Linux system run the following commands in the 
        root directory of the project:
    </p>
    <pre class="bg-bg-darker p-4 mt-2 font-thin text-lg">{'$ make'}</pre>
    <p class="font-thin text-xl">or</p>
    <pre class="bg-bg-darker p-4 mt-2 font-thin text-lg">{'$ make install'}</pre>
    <p class="font-thin text-lg">
        Alternatively you can use 
        <span class="font-semibold">Rust Tool Chain</span>
        to build and run the project.
    </p>
</section>
<section id="Guids" class="bg-primary px-10 md:px-20 py-10 text-lg text-gray-200">
    <h2 class="font-semibold text-2xl">Guids<h2>
    <p class="text-lg font-thin">In this section we are showcasing different aspects of Nmet syntax</p>
    <div class="mt-4 font-thin">
        <div class="">
            <h3 class="font-semibold text-xl"># Starting Point<h3>
            <p class="text-lg font-thin pt-2">
                Every program starts with a function named
                <b class="font-semibold">main</b>.
                usally this function has no arguments but you can define one with the type list-of-string to gain access to command line arguments.
            </p>
            <p class="font-thin text-base">
                p.s: Comments are starting with "~" because 
                we hate this character so we made it into a comment
            </p>
            <pre class="bg-bg-darker p-4 my-4 font-thin text-base">{'~~ No Arguments\nfunc main() {\n\t...\n}\n~~ With Arguments\nfunc main(args @[str,?]) {\n\t...\n}'}</pre>
        </div>
        <div class="row-auto">
            <h3 class="font-semibold text-xl"># Expressions<h3>
            <p class="text-lg font-thin">
                Expressions are grammars that can be used in every kind of operation; from mathematical to lists and function calls
                These expressions are just like in other languages.
            </p>
        </div>
        <div class="row-auto">
            <h3 class="font-semibold text-xl"># Print<h3>
            <p class="text-lg font-thin">
                Unlike other system level programming languages Nmet comes with an internal print function, This function dose not require Parentheses and can have as many arguments as you supply it to.
            </p>
            <pre class="bg-bg-darker p-4 my-4 font-thin text-base">{'print "Hello world!";\nprint 12 + 6;\n'}</pre>
        </div>
        <div class="row-auto">
            <h3 class="font-semibold text-xl"># Variable Declare<h3>
            <p class="text-lg font-thin">
                Every programming languages has a way to abstract away direct memory access with variables.
                In Nmet we define variables with "var" keyword followed by a name.
                All types in Nmet are indicated by the at-sign symbol. Adding type to variables is optional
                as long as you provide an initial value.
                Nmet also supports const variables which can be declared by adding a colon behind the question mark.
            </p>
            <pre class="bg-bg-darker p-4 my-4 font-thin text-base">{'var list @[char,12];\nvar x @int = 10\n~~ auto casting and const\nvar is_true := true;'}</pre>
        </div>
        <div>
            <h3 class="font-semibold text-xl"># Variable Assignment<h3>
            <p class="text-lg font-thin">
                Assigning values to a variable is a process which you can transfer result of your processes to a predefined memory
                In Nmet we have different types of assignments including add and assign or divide and assign which
                is represented in the code below:
            </p>
            <pre class="bg-bg-darker p-4 my-4 font-thin text-base">{'a = 10; ~~ a <- 10\na += 10; ~~ a <- a + 10\na -= 10; ~~ a <- a - 10\na *= 10; ~~ a <- a * 10\na /= 10; ~~ a <- a / 10\na %= 10; ~~ a <- a % 10'}</pre>
        </div>
        <div>
            <h3 class="font-semibold text-xl"># Loops<h3>
            <p class="text-lg font-thin">
                loops are one of the most important parts of any programming language. Nmet loops are
                defined using the while keyword followed by a condition which indicates when the looping ends
                pretenses surrounding the condition is optional.
            </p>
            <pre class="bg-bg-darker p-4 my-4 font-thin text-base">{'while a < 10 {\n\t...\n}'}</pre>
        </div>
        <div>
            <h3 class="font-semibold text-xl"># Conditions<h3>
            <p class="text-lg font-thin">
                if statements allow the user of the programming language to run some parts of the code
                conditionally. You can use if statements by writing the if keyword followed by the condition
                and with optional else and else if.
            </p>
            <pre class="bg-bg-darker p-4 my-4 font-thin text-base">{'if a < 10 {\n\t...\n} else if a > 10 {\n\t...\n} else {\n\t...\n}'}</pre>
        </div>
        <div>
            <h3 class="font-semibold text-xl"># Inline Asm<h3>
            <p class="text-lg font-thin">
                because Nmet is a low level language having a way to write in line assembly is essential
                for the usability of it. Inline assembly codes can be defined in a block marked by asm keyword.
                each assembly line is surrounded by double quotes. You can use Nmet declared variables inside
                the assembly lines by using the percentage sign. Make sure to use correct registers for different variable types
                .
            </p>
            <pre class="bg-bg-darker p-4 my-4 font-thin text-base">{'asm {\n\t"mov rax, %my_u64_variable"\n\t"inc rax"\n}'}</pre>
        </div>
    </div>
</section>
<footer class="text-center text-gray-400 p-5">
    Copywrite Mahan Farzaneh 2023-2024
</footer>
