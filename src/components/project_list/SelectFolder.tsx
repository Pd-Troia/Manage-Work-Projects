import * as React from 'react';
import { open } from '@tauri-apps/plugin-dialog';
export interface ISelectFolderProps {
}

export default function SelectFolder () {
  const [path,setPath] = React.useState<String|null>("")
    const handleClick = async()=>{
    open({
        multiple: false,
        directory: true
    })
    .then(path=>{setPath(path);console.log(path)})
    .catch(e=>console.log("erro ao selecionar o path",e))
  }
    return (
    <button onClick={handleClick}>
        {path ? "Selecione uma pasta" : "Troque de pasta"}
    </button >
  );
}
