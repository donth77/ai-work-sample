import React, { useRef, useState } from "react";
import { RingLoader } from "react-spinners";
import toast from "react-hot-toast";

const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL || "http://localhost:8080";

/**
 * TextEditor
 *
 */
export default function TextEditor() {
  const [content, setContent] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [loading, setLoading] = useState(false);
  const [selectedModel, setSelectedModel] = useState("gpt-4.1-mini");
  const [selectedLanguage, setSelectedLanguage] = useState("es");

  const handleAI = async (command: string, language?: string) => {
    const textarea = textareaRef.current;
    if (!textarea) return;

    const { selectionStart, selectionEnd, value } = textarea;
    if (selectionStart === selectionEnd) return; // nothing selected

    const selected = value.slice(selectionStart, selectionEnd);
    setLoading(true);
    try {
      const requestBody: any = {
        command,
        text: selected,
        model: selectedModel,
      };

      // Add language parameter for translate command
      if (command === "translate" && language) {
        requestBody.lang = language;
      }

      const resp = await fetch(`${API_BASE_URL}/api/ai`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(requestBody),
      });
      const { result } = await resp.json();

      // replace the selection with the new text
      const newValue =
        value.slice(0, selectionStart) + result + value.slice(selectionEnd);
      setContent(newValue);

      // restore cursor position after the new text
      requestAnimationFrame(() => {
        if (textareaRef.current) {
          textareaRef.current.selectionStart =
            textareaRef.current.selectionEnd = selectionStart + result.length;
        }
      });
    } catch (err) {
      console.error("AI command failed", err);
      toast.error("AI command failed. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  const shouldDisableButtons = loading || !content;

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 8,
        width: "100%",
      }}
    >
      <div style={{ position: "relative" }}>
        <textarea
          ref={textareaRef}
          value={content}
          onChange={(e) => setContent(e.target.value)}
          rows={16}
          style={{ fontFamily: "monospace", width: "100%" }}
        />
        {loading && (
          <div
            style={{
              position: "absolute",
              bottom: "30px",
              right: "20px",
              pointerEvents: "none",
            }}
          >
            <RingLoader size={50} color="#007acc" />
          </div>
        )}
      </div>

      {/* Model Selection */}
      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
        <label htmlFor="model-select" style={{ fontWeight: "bold" }}>
          Model:
        </label>
        <select
          id="model-select"
          value={selectedModel}
          onChange={(e) => setSelectedModel(e.target.value)}
          style={{ padding: "4px 8px", borderRadius: "4px" }}
          disabled={shouldDisableButtons}
        >
          <option value="gpt-3.5-turbo">gpt-3.5-turbo</option>
          <option value="gpt-4.1-mini">gpt-4.1-mini</option>
        </select>
      </div>

      <div
        style={{
          display: "flex",
          justifyContent: "center",
          gap: 12,
          flexWrap: "wrap",
        }}
      >
        <button
          onClick={() => handleAI("paraphrase")}
          disabled={shouldDisableButtons}
        >
          AI Paraphrase
        </button>
        <button
          onClick={() => handleAI("summarize")}
          disabled={shouldDisableButtons}
        >
          AI Summarize
        </button>

        {/* Translate Button with Language Dropdown */}
        <div style={{ display: "flex", alignItems: "center", gap: 4 }}>
          <button
            onClick={() => handleAI("translate", selectedLanguage)}
            disabled={shouldDisableButtons}
          >
            AI Translate
          </button>
          <select
            value={selectedLanguage}
            onChange={(e) => setSelectedLanguage(e.target.value)}
            style={{ padding: "4px 8px", borderRadius: "4px" }}
            disabled={shouldDisableButtons}
          >
            <option value="es">es</option>
            <option value="fr">fr</option>
            <option value="de">de</option>
            <option value="it">it</option>
            <option value="pt">pt</option>
            <option value="ru">ru</option>
            <option value="ja">ja</option>
            <option value="ko">ko</option>
            <option value="zh">zh</option>
            <option value="ar">ar</option>
            <option value="hi">hi</option>
            <option value="nl">nl</option>
            <option value="sv">sv</option>
            <option value="pl">pl</option>
            <option value="tr">tr</option>
            <option value="en">en</option>
          </select>
        </div>
      </div>
    </div>
  );
}
