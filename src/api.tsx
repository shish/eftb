type ApiReturn = {
  version: number;
  // eslint-disable-next-line
  data: any;
};

export function api(
  expected_version: number,
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
    .then((data: ApiReturn) => {
      if (data.version == expected_version) {
        onError(null);
        onData(data.data);
      } else {
        onError(new Error("Version mismatch - refresh the page?"));
        onData(null);
      }
    })
    .catch((error) => {
      onError(error as Error);
      onData(null);
    });
}
