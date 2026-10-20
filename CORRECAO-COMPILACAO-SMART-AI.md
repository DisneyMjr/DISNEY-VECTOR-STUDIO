# Correção da compilação Rust do Smart AI

Foram corrigidos literais de ponto flutuante inválidos no arquivo `crates/ui-egui/src/dialogs/smart_ai.rs`, incluindo `hf*.69` para `hf*0.69`, `91.` para `91.0` e equivalentes.

O ZIP preserva os arquivos de workflow já corrigidos na versão anterior.

**Validação:** inspeção de código e integridade ZIP. Não foi possível executar `cargo check` neste ambiente; caso a compilação aponte novos erros, compartilhe o início do log.

Para instalar: copie os arquivos sobre o repositório local, preserve `.git`, execute `git add -A`, `git commit` e `git push origin main`.
