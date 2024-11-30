import * as React from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
export interface ISelectFolderProps {
}

export default function SelectFolder () {
  const [path,setPath] = React.useState<String|null>("")
    const handleClick = async()=>{
    open({
        multiple: false,
        directory: true
    })
    .then(path=>{
      console.log("path",path)
      invoke("save_root_folder",{rootPath:path}).catch(err=>"qual erro?"+err)
      setPath(path);
    })
    .catch(e=>console.log("erro ao selecionar o path",e))
  }
    return (
    <button onClick={handleClick}>
        {!path ? "Selecione uma pasta" : "Troque de pasta"}
    </button >
  );
}
