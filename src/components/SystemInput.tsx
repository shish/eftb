// import systemnames from "../consts/systemnames.json";
import { useEffect, useState } from "react";
import { AutoCompleteInput } from "./AutoCompleteInput";

export function SystemInput({
  name,
  value,
  onChange,
}: {
  name: string;
  value: string;
  onChange: (value: string) => void;
}) {
  const [systemNames, setSystemNames] = useState<string[]>([]);

  useEffect(() => {
    import("../consts/systemnames.json")
      .then((data) => {
        setSystemNames(data.default);
      })
      .catch((e) => {
        console.error("Failed to load system names:", e);
        setSystemNames([]);
      });
  }, []);

  function getCompletions(input: string): string[] {
    return systemNames.filter((v) =>
      v.toLowerCase().startsWith(input.toLowerCase()),
    );
  }

  return (
    <AutoCompleteInput
      required={true}
      name={name}
      value={value}
      onChange={(v) => onChange(v)}
      getCompletions={getCompletions}
      placeholder="Enter system name"
    />
  );
}
