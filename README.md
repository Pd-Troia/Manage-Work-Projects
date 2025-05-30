# Manage Work projects

Este projeto foi criado com o objetivo de dinamizar meu fluxo de trabalho ao abrir múltiplas instâncias do VSCode, utilizando uma interface gráfica moderna.

A aplicação foi desenvolvida com **Tauri v2** e **React**, permitindo uma experiência leve, rápida e nativa para gerenciamento de projetos locais.

## 💡 Objetivo

Evitar a repetição manual de abrir projeto por projeto no VSCode. Com esta ferramenta, é possível abrir várias instâncias do editor com apenas alguns cliques.

## 📂 Como usar

1. Crie uma **pasta principal** para agrupar todos os projetos relacionados.  
   Exemplo:  
    /meus-projetos  
   &nbsp;&nbsp;&nbsp;├── projeto-a  
   &nbsp;&nbsp;&nbsp;├── projeto-b  
   &nbsp;&nbsp;&nbsp;└── projeto-c  

2. Execute o aplicativo.

3. Na interface, selecione a pasta principal.

4. A interface exibirá todas as subpastas. Ao clicar em uma delas, o VSCode será aberto naquele diretório utilizando `code .`.

## ✅ Funcionalidades atuais

- Interface gráfica feita com React.
- Execução de `code .` em cada subpasta ao clicar.
- Suporte nativo com Tauri para rodar como app desktop.

## 📋 To Do

- [ ] Permitir criar diferentes comandos de execução para cada pasta.
- [ ] Salvar pastas acessadas anteriormente.
- [ ] Criar um seletor para abrir apenas um projeto ao invés de todos.

## 🧪 Requisitos

- **VSCode** instalado e disponível no terminal (comando `code`).
- **Node.js** instalado.
- **Rust** configurado (necessário para Tauri).
- **Tauri CLI** (pode ser instalado com `cargo install create-tauri-app` ou via `npm`).

## 🛠️ Execução em modo desenvolvedor

Clone o repositório e execute:

```bash
npm install
npm run tauri dev
