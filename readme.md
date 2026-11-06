## 1. Nível Inicial (Fixando os fundamentos)

Nesta fase, o objetivo é praticar manipulação de strings, structs, enums, gerenciamento de erros com `Result`/`Option` e leitura de arquivos (`std::fs`).

### **Gerenciador de Tarefas via Terminal (Todo CLI)**

* **O que faz:** Um aplicativo de linha de comando para adicionar, listar, marcar como concluída e remover tarefas.
* **O que você pratica:**
* Structs e enums para representar o estado da tarefa.
* Serialização/deserialização salvando as tarefas em um arquivo JSON usando a crate `serde` e `serde_json`.
* Leitura de argumentos do terminal (`std::io::stdin()`).