# Publicação unificada do VectorSmart

No GitHub, abra Actions > Publicar versão VectorSmart (Windows, Linux e macOS) > Run workflow.
Digite uma versão nova, por exemplo 1.3.1, observando que o projeto pode exigir correspondência com a versão de Cargo.toml.

O workflow executa cinco jobs: validar, windows, linux, macos e publicar. O último job só cria a Release depois que os três sistemas compilarem com sucesso.

O workflow de iOS não está incluído: o aplicativo iOS ainda precisa de um projeto Xcode, integração do motor Rust e validação em aparelhos.

A compilação final, assinatura do macOS e execução no GitHub Actions não foram validadas neste ambiente.
