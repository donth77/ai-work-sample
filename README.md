# Live

API Live URL:

https://ai-work-sample.onrender.com/

Frontend Live URL:

https://ai-work-sample-frontend.onrender.com/

# Frontend

## Run Locally

```bash
# install deps
npm install

#  set API url (optional - localhost:8080 default)
echo "VITE_API_BASE_URL=http://localhost:8080" > .env

# start dev server
npm run dev      # http://localhost:5173 by default

```

## Features

- Textarea – Write, paste, or select text to be replaced by AI commands.

- Model menu – Select the LLM: gpt-3.5-turbo or gpt-4.1-mini (default).

- Language menu – Select the language code for AI commands.

- Buttons
  - AI Paraphrase – Rewrites the selected text.
  - AI Summarize – Produces a concise summary.
  - AI Translate – Translates the selection to the selected language in the dropdown.

---

# Backend - Text Editor AI-powered API

## Run Locally

- Create a `.env` file and set `OPENAI_API_KEY=<your-api-key>`

```bash
# 1. Build
cargo build --release

# 2. Configure
cp .env.example .env          # edit OPENAI_API_KEY, PORT (optional)

# 3. Run
cargo run --release
# → server listening on 0.0.0.0:8080 (unless PORT is set)
```

## Endpoints

---

### `POST /api/ai`

| Field     | Type   | Required  | Description                                                                                             |
| --------- | ------ | --------- | ------------------------------------------------------------------------------------------------------- |
| `command` | string | **Yes**   | **`paraphrase`**, **`summarize`**, or **`translate`**.                                                  |
| `text`    | string | **Yes**   | The input text to process (any length OpenAI accepts).                                                  |
| `model`   | string | No        | `gpt-3.5-turbo` or `gpt-4.1-mini` (default).                                                            |
| `lang`    | string | Sometimes | _Required when_ `command == "translate"`. ISO 639-1 or ISO 639-3 code. Special cases: `zh-cn`, `zh-tw`. |

#### Request example

```bash
curl -X POST http://localhost:8080/api/ai \
  -H "Content-Type: application/json" \
  -d '{
        "command":"summarize",
        "text":"Rust is a multi-paradigm systems programming language...",
        "model":"gpt-3.5-turbo"
      }'
```

#### Response

```jsonc
{
  "result": "Rust is a fast, memory-safe systems language that combines...",
  "model": "gpt-3.5-turbo",
  "lang": "en"
}
```

---

### Error Handling

| HTTP Code | Scenario                                                                                         | Sample message                                                                    |
| --------- | ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- | --- |
| `400`     | Invalid JSON, unknown `command`, missing `lang` for translation, bad `model`, bad language code. | `"Invalid command 'foo'. Valid commands: 'paraphrase', 'summarize', 'translate'"` |
| `502`     | Network/JSON error when contacting OpenAI.                                                       | `"error communicating with upstream: ..."`                                        |     |

---

## Commands

| Command        | System prompt summary                       |
| -------------- | ------------------------------------------- | --- |
| **paraphrase** | Re-phrases the input while keeping meaning. |     |
| **summarize**  | Produces a concise overview..               |     |
| **translate**  | Translates to the target language (`lang`). |

### Language validation

- Accepts **ISO 639-1** (2-letter) and **ISO 639-3** (3-letter).
- Exceptions: `zh-cn` (Chinese Simplified) and `zh-tw` (Chinese Traditional).

---

## Built With

#### Backend:

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum)
- [Reqwest](https://docs.rs/reqwest)
- [isolang](https://docs.rs/isolang)

#### Frontend:

- [React](https://react.dev/)
- [TypeScript](https://www.typescriptlang.org/)
- [Vite](https://vitejs.dev/)
- [React Hot Toast](https://react-hot-toast.com/)
