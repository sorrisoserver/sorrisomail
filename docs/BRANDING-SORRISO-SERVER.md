# Identidade visual — Sorriso Server Mail (nome oficial)

## Referência oficial
A arte oficial foi fornecida pelo responsável pela Sorriso Server nesta conversa em 21/10/2026. É uma marca com fundo azul-marinho escuro, símbolo de nuvem e circuitos em azul/ciano, logotipo SORRISOSERVER e slogan "TRANSFORMANDO IDEIAS EM SOLUÇÕES WEB".

**Não substituir a logo por uma versão gerada ou redesenhada sem aprovação.** A imagem JPEG original precisa ser adicionada posteriormente ao repositório como ativo de referência; por enquanto, não há ativo binário nesta branch.

## Cores iniciais (aproximadas, extraídas da referência JPEG)
- Fundo principal: `#061625`
- Azul de destaque: `#138ED2`
- Azul médio: `#107EC5`

Ajustar valores finais após aprovação de visualização no aplicativo.

## Diretrizes
1. Preservar o histórico e o código-base do MailCraft, incluindo avisos de licença e atribuição.
2. Trabalhar somente na branch `branding-sorriso-server`; alterações na `main` exigem revisão.
3. A identidade visual não deve alterar protocolos IMAP/SMTP, armazenamento de credenciais ou funcionalidades de e-mail.
4. Aplicativo Windows será o primeiro alvo de testes, sem descartar suporte a outras plataformas.
5. Nome oficial aprovado pelo responsável em 21/10/2026: **Sorriso Server Mail**.

## Primeiras tarefas
- [x] Criar branch de personalização.
- [x] Registrar identidade, nome provisório e paleta inicial.
- [ ] Adicionar o arquivo original da logo ao repositório.
- [x] Localizar os pontos principais do nome da janela e dos metadados Windows.
- [x] Atualizar nome da janela e ProductName/FileDescription/CompanyName no executável Windows.
- [ ] Mapear os demais textos, ícones e identificadores do aplicativo.
- [ ] Implementar ícones e identidade visual sem alterar o funcionamento do cliente.
- [ ] Configurar compilação automatizada para Windows.
- [ ] Validar build e realizar testes práticos antes de distribuir.

## Licenciamento
A base MailCraft declara licença `MIT OR Apache-2.0`. Preservar os avisos exigidos e verificar marcas e recursos de terceiros antes de distribuir comercialmente.

## Implementação inicial (21/10/2026)
- `apps/mailcraft/src/main.rs`: título da janela, nome usado na inicialização nativa e menu macOS atualizados para Sorriso Server Mail.
- `apps/mailcraft/build.rs`: nome do produto, descrição e empresa alterados nos recursos VERSIONINFO Windows.
- O executável, os ícones embutidos, o ID de aplicativo (`io.github.billsecond.mailcraft`) e os componentes de atualização/instalação ainda mantêm identificadores originais. **Não distribuir como versão final antes de revisar esses pontos**, especialmente atualizações automáticas, caminhos de dados e associações de arquivos.
- Compilação e testes ainda não executados; nenhuma alegação de funcionamento validado.

## Repositório renomeado
- Repositório oficial do produto: https://github.com/sorrisoserver/sorrisomail
- Origem preservada como fork: https://github.com/UCS-Care-Group/mailcraft
- Nome de pasta/repositório e nome interno das crates Rust são independentes; a renomeação externa não exige refatoração completa dos módulos.

## Separação da identidade — etapa seguinte
- Nome visível do aplicativo: Sorriso Server Mail.
- ID de janela alterado para `br.com.sorrisoserver.sorrisomail`, sem renomear as crates internas Rust para evitar regressões desnecessárias.
- Metadados do Windows atualizados com `InternalName=SorrisoServerMail`.
- Mensagem de janela de caixa postal já aberta personalizada.
- Os arquivos de logo/ícone originais estão preparados fora do repositório e a integração dos binários ainda está pendente.
- Pendente revisar diretórios de credenciais/dados, associação de arquivos, instalador, assinatura e atualização antes do uso real.
- O workflow de teste Windows está configurado, mas isso não equivale a compilação aprovada.

## Preparação do ícone oficial — implementação incremental
- Adicionado `scripts/sorriso-brand-assets.py`, que recebe o arquivo da logo aprovada e exporta PNG e ICO sem redesenhar a marca.
- `apps/mailcraft/build.rs` passou a reconhecer `assets/sorriso-server/sorriso-server.ico`, mantendo temporariamente o ícone original como fallback para não quebrar a compilação.
- **A marca ainda não está embutida no executável**, porque os arquivos binários oficiais não estão versionados no GitHub; não distribuir builds atuais como versão personalizada concluída.
- Não migrar caminhos de dados/credenciais nem alterar associações de arquivos antes de projetar migração segura e testar com perfis limpos.

## Requisito obrigatório aprovado — idioma pt-BR (21/10/2026)
- O aplicativo **Sorriso Server Mail** deve apresentar toda a interface ao usuário exclusivamente em **português do Brasil (pt-BR)**.
- **Não oferecer seletor de idiomas** nas configurações; não permitir mudar para inglês.
- Menus, botões, pastas padrão, mensagens, diálogos, notificações e telas de configuração devem estar traduzidos.
- Textos técnicos que integram protocolos, formatos e comandos internos podem permanecer com identificadores originais para não quebrar compatibilidade.
- A biblioteca atual só contém catálogos en/fr/de/es/it; logo, **a tradução pt-BR ainda não foi implementada**. Tradução parcial com retorno ao inglês não satisfaz o requisito.
- Antes de considerar a versão pronta, criar o catálogo pt-BR completo, fixar o locale independentemente do idioma do Windows, remover o seletor, testar telas e adicionar um teste que identifique frases em inglês remanescentes.

## Funcionalidade planejada — licenciamento comercial anual (21/10/2026)
- O proprietário deseja distribuir Sorriso Server Mail a clientes com chave/licença válida por 1 ano e renovação anual.
- Proposta: ativação por chave, painel administrativo, validação HTTPS, licenças assinadas digitalmente, prazo offline limitado, renovação sem reinstalação, bloqueio/revogação e transferência de dispositivo.
- Recomendação de preservação de dados: após vencimento, manter acesso de leitura/exportação local, mesmo que recursos sincronizados sejam suspensos.
- **Modelo aprovado:** uma licença por computador, com múltiplas contas de e-mail na mesma instalação e validade de 365 dias por ativação/renovação. Possibilidade futura de vender pacotes de licenças, cada uma vinculada a um dispositivo. Sem implementação nesta etapa.
- A administração de assinaturas e a cobrança deverão ser desenvolvidas separadamente do cliente, com verificação de segurança e requisitos de privacidade/LGPD.

## Implementação inicial do pt-BR (21/10/2026)
- `crates/i18n/src/lib.rs`: incluído `BrazilianPortuguese` com código `pt-BR`.
- `crates/i18n/po/pt-BR.po`: catálogo inicial de termos mais visíveis (ainda incompleto).
- `crates/ui-egui/src/backstage.rs`: seletor de idioma retirado das opções.
- `crates/ui-egui/src/lib.rs`: idioma em vigor fixado em `BrazilianPortuguese`; comando `ui.language` não permite mudar para outro idioma.
- **Atenção**: ainda faltam traduções e os testes automatizados não estavam concluídos no momento da alteração. Não declarar a interface 100% em português até concluir o catálogo e revisar visualmente todas as telas.

## Revisão visual em 21/10/2026 — seis capturas reais do Windows
- Título da janela: nome comercial agora aparece, mas pastas e caixa postal ainda em inglês.
- Barra de opções, guias Enviar/Receber, Pasta, Exibir e página Informações exibem inglês residual.
- Assistente de adição de conta ainda renderizava 'MailCraft' diretamente no código; corrigido em `account_setup.rs`.
- Catálogo acrescentado com traduções `msgctxt "folder"`, exigidas pelo nome dos diretórios padrão.
- **Bloqueio crítico identificado**: a versão Alpha salva dados em `%APPDATA%\\MailCraft` (não usar clientes reais até termos diretório isolado `Sorriso Server Mail`, migração opcional segura e testes). Preservar arquivos existentes; não mudar o caminho precipitadamente sem plano de migração.
- Essas correções dependem de nova compilação e inspeção visual. Não declarar tradução integral concluída.

## Preparação para produção — 22/10/2026
- Corrigido `Item::tip_text` no ribbon para aplicar a mesma substituição pt-BR dos rótulos de comandos visuais; requer revisão de tooltips dinâmicos do motor.
- Isolado diretório padrão de configurações no Windows para `%APPDATA%\\Sorriso Server Mail`, no macOS para `~/Library/Application Support/Sorriso Server Mail` e em Linux para `~/.config/sorriso-server-mail` (ou XDG). **Não faz migração automática dos dados já guardados em `MailCraft`.**
- Antes de produção: verificar todos os caminhos dependentes; testar primeira instalação, persistência, leitura/reabertura de caixa postal, autenticação IMAP/SMTP, anexos, certificados, exportação e instalação/desinstalação; completar pt-BR; assinar binários; resolver CI e desenvolver licenciamento comercial anual.
- Nunca cadastrar clientes reais até validar privacidade e isolamento dos dados; a versão Alpha continua sem licença comercial implementada.
