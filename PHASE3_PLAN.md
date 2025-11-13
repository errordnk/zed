# Phase 3: Remove Editor-Focused AI Crates

## Goal
Remove editor-focused AI/code completion, keep only terminal ACP assistant.

## Terminal AI Use Case
```
User: "посмотри что там в докере, почему падает нжинкс"
AI: → docker ps → docker logs → анализирует → отвечает в чате
```

## Crates to REMOVE (23 crates)

### Editor AI/Code Completion
1. `agent_ui` - inline assistant, buffer codegen для редактора
2. `acp_thread` - AI threads для редактора
3. `acp_tools` - AI tools для редактора
4. `assistant_text_thread` - AI чат для редактора
5. `assistant_slash_command` - slash commands для редактора
6. `assistant_slash_commands` - slash commands для редактора
7. `zeta` - AI code completion (как Copilot)
8. `zeta2` - AI code completion v2
9. `zeta_cli` - CLI для zeta
10. `zeta2_tools` - tools для zeta2
11. `cloud_zeta2_prompt` - cloud prompts для zeta2
12. `supermaven` - AI code completion
13. `supermaven_api` - Supermaven API
14. `codestral` - Mistral AI code completion
15. `edit_prediction` - AI edit predictions
16. `ai_onboarding` - AI onboarding UI
17. `copilot` - GitHub Copilot integration
18. `prompt_store` - AI prompts storage

### Editor-Specific UI
19. `markdown_preview` - markdown preview в редакторе
20. `language_tools` - LSP tools view
21. `language_onboarding` - language setup
22. `line_ending_selector` - line ending picker
23. `rules_library` - linting rules
24. `action_log` - editor action logging
25. `buffer_diff` - buffer diff view
26. `streaming_diff` - streaming diff for AI

### Snippets (if not needed for terminal)
27. `snippet` - snippet system
28. `snippet_provider` - snippet provider
29. `snippets_ui` - snippets UI

## Crates to KEEP

### Terminal ACP
- ✅ `agent` - базовая ACP инфраструктура для терминала
- ✅ `agent_servers` - кастомные ACP серверы (OpenCode и т.д.)
- ✅ `agent_settings` - настройки ACP
- ✅ `context_server` - контекст команд терминала

### LLM Providers (для ACP)
- ✅ `anthropic` (Claude)
- ✅ `google_ai` (Gemini)
- ✅ `open_ai` (Codex)
- ✅ `deepseek`, `lmstudio`, `mistral`, `ollama`, `open_router`, `bedrock`, `vercel`, `x_ai`
- ✅ `cloud_llm_client`, `cloud_api_client`, `cloud_api_types`
- ✅ `language_model`, `language_models` - базовая LLM инфраструктура
- ✅ `aws_http_client` - для bedrock

### Core Infrastructure
- ✅ `terminal`, `terminal_view` - терминал
- ✅ `workspace`, `worktree`, `project` - проекты
- ✅ `gpui`, `ui`, `ui_input`, `ui_macros` - UI framework
- ✅ `settings`, `settings_ui`, `theme` и т.д.

## Estimated Reduction
- Remove: ~29 crates
- Keep: ~115 crates
- Reduction: ~20% of codebase
