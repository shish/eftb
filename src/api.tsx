export function api(
  form: HTMLFormElement,
  // eslint-disable-next-line
  onData: (data: null | any) => void,
  onError: (error: null | Error) => void,
) {
  const form_data = new FormData(form);
  // eslint-disable-next-line
  const params = new URLSearchParams(form_data as any).toString();
  const url = form.action + "?" + params;

  fetch(url)
    .then((response) =>
      response.ok
        ? response.json()
        : response.text().then((text) => Promise.reject(Error(text))),
    )
    .then((data) => {
      onError(null);
      onData(data);
    })
    .catch((error) => {
      onError(error as Error);
      onData(null);
    });
}
