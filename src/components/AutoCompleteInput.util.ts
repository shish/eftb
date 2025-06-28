export function find_word(
  value: string,
  separator: string,
  position: number,
): [number, number] {
  if (
    position < 1 ||
    position > value.length ||
    value[position - 1] === separator
  ) {
    return [0, 0];
  }

  let start = position;
  let end = position;
  while (start > 0 && value[start - 1] !== separator) {
    start--;
  }
  while (end < value.length && value[end] !== separator) {
    end++;
  }

  return [start, end];
}

export function get_word(
  value: string,
  separator: string,
  position: number,
): string {
  const [start, end] = find_word(value, separator, position);
  return value.substring(start, end);
}

export function replace_word(
  value: string,
  separator: string,
  position: number,
  replacement: string,
): string {
  const [start, end] = find_word(value, separator, position);
  return (
    value.substring(0, start) +
    replacement +
    (value[end] === separator ? "" : separator) +
    value.substring(end)
  );
}

export function clamp(val: number, min: number, max: number): number {
  return Math.min(Math.max(val, min), max);
}
