🟢 Iniciante - Projetos simples (linha de comando)

    Todo CLI (lista de tarefas)

        Entrada via terminal, salva em um .json ou .toml

        Aprende: struct, enum, serde, file I/O, match

    Conversor de unidades

        Ex: Celsius ↔ Fahrenheit, km ↔ milhas

        Aprende: match, fn, args, pattern matching

    Gerador de senhas

        Opções: tamanho, incluir símbolos, letras maiúsculas etc.

        Aprende: rand, Vec<char>, manipulação de string

    Cronômetro / Temporizador no terminal

        Com contagem regressiva e alerta

        Aprende: std::thread::sleep, loop, chrono

🟡 Intermediário - Persistência e lógica mais elaborada

    Gerenciador de despesas pessoal

        Cadastro simples de gastos (categoria, valor, data)

        Armazena em JSON ou SQLite

        Aprende: serde, rusqlite, structs complexas

    CLI de flashcards

        Para estudar: mostra uma pergunta, espera a resposta

        Aprende: leitura de arquivos, aleatoriedade, lógica condicional

    Simulador de rolagem de dados (RPG)

        Ex: 2d6 + bônus

        Aprende: entrada do usuário, regex básica (parse da expressão)

🟠 Avançando - Conceitos mais de sistema

    Mini Shell

        Interpreta comandos básicos: ls, cd, echo

        Aprende: std::process, match, gerenciamento de processos

    HTTP client CLI

        Tipo um curl básico: GET, POST com corpo JSON

        Aprende: reqwest, async/await, tokio (assíncrono)

    Servidor web básico

        Cria um servidor HTTP simples (página estática ou API)

        Aprende: hyper, axum, tokio, async

🔵 Extras - Projetos gráficos / web low-code

    Web app com frontend em HTML usando leptos ou yew

        Contador, checklist, ou app de notas simples

        Aprende: Rust para WebAssembly

    Automatizador de tarefas

        Lê arquivos .csv, gera relatórios .pdf ou .xlsx

        Aprende: csv, pdf, calamine (Excel)
