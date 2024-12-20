import React from "react";

type DataContextType = {
  stars: string[];
};
// eslint-disable-next-line
export const DataContext = React.createContext<DataContextType>({
  stars: [],
});

export function DataProvider({ children }: { children: React.ReactNode }) {
  const [stars, setStars] = React.useState<string[]>([]);
  React.useEffect(() => {
    fetch("/api/stars")
      .then((res) => res.json())
      // eslint-disable-next-line
      .then((data) => setStars(data.data))
      .catch((err) => console.error(err));
  }, []);

  return (
    <DataContext.Provider value={{ stars }}>{children}</DataContext.Provider>
  );
}
