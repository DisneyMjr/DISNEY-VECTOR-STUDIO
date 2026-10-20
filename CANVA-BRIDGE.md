# Canva Bridge (beta) — VectorSmart

## O que funciona nesta versão

- Menu **File → Scripts → Canva Bridge · Exportar para Canva…**.
- Abre o assistente de exportação PDF já existente no VectorSmart.
- Abre o Canva no navegador para importar o PDF exportado.
- Preserva Smart AI, Smart Social, JSX e Smart QR Pro.
- Preserva workflows de MSI (Windows) e Linux.

## Como usar

1. Abra a arte no VectorSmart.
2. File → Scripts → Canva Bridge → **Exportar a arte atual como PDF**.
3. Salve o PDF.
4. Volte a Canva Bridge → **Abrir Canva**.
5. No Canva, importe o PDF pelo fluxo de upload/importação.

**Limites:** não é sincronização automática, não vincula a conta Canva e não faz upload pela API nesta versão. PDFs são aceitos pela API oficial para importação de designs, mas a editabilidade final dos elementos depende do conversor do Canva.

## Próxima etapa: conexão oficial OAuth

As APIs Canva Connect exigem OAuth 2.0 com PKCE, credenciais de cliente protegidas em um backend web e URI de retorno HTTPS registrada no Canva Developers. A própria documentação do Canva recomenda **não** colocar o segredo diretamente em software desktop. Para implementar acesso autenticado, criar serviço web seguro (por exemplo, PHP no cPanel com HTTPS), cadastrar o app em https://www.canva.com/developers, configurar o fluxo OAuth, guardar tokens no servidor e implementar a API de importação de designs ou upload de recursos.

Referências oficiais:
- https://www.canva.dev/docs/apps/rest-apis/security-recommendations/
- https://www.canva.dev/docs/apps/rest-apis/authentication/
- https://www.canva.dev/docs/apps/rest-apis/reference/design-imports/

**Nenhuma credencial deve ser enviada em chats, comitada no GitHub ou empacotada no MSI.**

## Testes

Código-fonte alterado e presença dos workflows inspecionados. Rust/MSI/Linux não foram compilados neste ambiente; validar pelo GitHub Actions antes de distribuir.
