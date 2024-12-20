type ApiReturn = {
  version: number;
  // eslint-disable-next-line
  data: any;
};

export function form_api(
  form: HTMLFormElement,
  expected_version: number,
  // eslint-disable-next-line
  onData: (data: null | any) => void,
  onError: (error: null | Error) => void,
) {
  const form_data = new FormData(form);
  // eslint-disable-next-line
  const params = new URLSearchParams(form_data as any).toString();
  const url = form.action + "?" + params;
  api(url, expected_version, onData, onError);
}

export function api(
  url: string,
  expected_version: number,
  // eslint-disable-next-line
  onData: (data: null | any) => void,
  onError: (error: null | Error) => void,
) {
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
