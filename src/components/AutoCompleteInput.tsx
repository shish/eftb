import { useEffect, useRef, useState, KeyboardEvent, useCallback } from "react";
import { get_word, replace_word, clamp } from "./AutoCompleteInput.util";
import "./AutoCompleteInput.css";
import { useDebounceCallback } from "usehooks-ts";

export function AutoCompleteInput({
  name,
  value,
  onChange,
  required = false,
  placeholder = "",
  getCompletions,
  separator = null,
  debounce = 50,
}: {
  name: string;
  value: string;
  onChange: (value: string) => void;
  required?: boolean;
  placeholder?: string;
  getCompletions: (input: string) => string[];
  separator?: string | null;
  debounce?: number;
}) {
  const [showCompletions, setShowCompletions] = useState(false);
  const [completions, setCompletions] = useState<string[]>([]);
  const [searchPos, setSearchPos] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const [highlighted, setHighlighted] = useState(0);
  const refreshCompletions = useCallback(
    (term: string) => setCompletions(getCompletions(term)),
    [getCompletions, setCompletions],
  );
  const debouncedRefreshCompletions = useDebounceCallback(
    refreshCompletions,
    debounce,
  );

  // When moving the cursor, change the currently-selected word
  useEffect(() => {
    function handleSelectionChange() {
      if (document.activeElement === inputRef.current) {
        setSearchPos(inputRef?.current?.selectionStart ?? 0);
      }
    }
    document.addEventListener("selectionchange", handleSelectionChange);
    return () => {
      document.removeEventListener("selectionchange", handleSelectionChange);
    };
  }, []);

  // When currently-selected word changes, update the completions
  useEffect(() => {
    const term = separator ? get_word(value, separator, searchPos) : value;
    if (term) {
      debouncedRefreshCompletions(term);
    } else {
      setCompletions([]);
    }
  }, [value, separator, searchPos, debouncedRefreshCompletions]);

  // When completions change, highlight the first entry
  useEffect(() => {
    setHighlighted(0);
  }, [showCompletions, completions.length]);

  function onKeyDown(event: KeyboardEvent) {
    // up / down should select previous / next completion
    if (event.code === "ArrowUp") {
      event.preventDefault();
      setHighlighted((prev) => clamp(prev - 1, 0, completions.length - 1));
    } else if (event.code === "ArrowDown") {
      event.preventDefault();
      setHighlighted((prev) => clamp(prev + 1, 0, completions.length - 1));
    }
    // if enter or right are pressed while a completion is selected, add the selected completion
    else if (
      (event.code === "Enter" || event.code == "ArrowRight") &&
      showCompletions &&
      highlighted < completions.length &&
      highlighted >= 0
    ) {
      event.preventDefault();
      setTerm(completions[highlighted]);
    }
    // if escape is pressed, hide the completion block
    else if (event.code === "Escape") {
      event.preventDefault();
      setShowCompletions(false);
    }
  }

  function setTerm(term: string) {
    const newVal = separator
      ? replace_word(value, separator, searchPos, term)
      : term;
    onChange(newVal);
    inputRef?.current?.focus();
  }

  const inputRect = inputRef.current?.getBoundingClientRect();
  return (
    <div>
      <input
        type="text"
        ref={inputRef}
        required={required}
        name={name}
        value={value}
        autoComplete="off"
        spellCheck="false"
        onKeyDown={(e) => onKeyDown(e)}
        onChange={(e) => onChange(e.target.value)}
        onFocus={() => setShowCompletions(true)}
        onBlur={() => setShowCompletions(false)}
        placeholder={placeholder}
      />
      {showCompletions &&
        completions.length > 0 &&
        completions[0] != value &&
        inputRect && (
          <ul
            className="autocomplete"
            style={{
              position: "absolute",
              top: inputRect.top + inputRect.height + "px",
              left: inputRect.left + "px",
              width: inputRect.width ?? "auto",
            }}
          >
            {completions.slice(0, 20).map((c, n) => (
              <li
                key={c}
                // onClick happens after onBlur, so if we use that, the list will close
                // and the click will not register. onMouseDown and onTouchStart happen
                // before onBlur, so they work.
                onMouseDown={(e) => {
                  setTerm(c);
                  e.preventDefault();
                }}
                onTouchStart={(e) => {
                  setTerm(c);
                  e.preventDefault();
                }}
                onMouseMove={() => setHighlighted(n)}
                className={n === highlighted ? "highlighted" : ""}
              >
                {c}
              </li>
            ))}
          </ul>
        )}
    </div>
  );
}
