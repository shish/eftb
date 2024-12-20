import React from "react";
import { DataContext } from "../providers/DataProvider";

export function StarDataList(props: { id: string }) {
  const { stars } = React.useContext(DataContext);
  return (
    <datalist id={props.id}>
      {stars.map((star) => (
        <option key={star}>{star}</option>
      ))}
    </datalist>
  );
}
