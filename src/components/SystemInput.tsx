export function SystemInput({
  name,
  value,
  onChange,
}: {
  name: string;
  value: string;
  onChange: (value: string) => void;
}) {
  return (
    <input
      type="text"
      required={true}
      name={name}
      value={value}
      onChange={(e) => onChange(e.target.value)}
      placeholder="Enter system name"
    />
  );
}
