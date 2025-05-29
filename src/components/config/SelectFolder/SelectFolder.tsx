import * as React from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { ProjectContex } from '../../project_list';
export interface ISelectFolderProps {
}

export default function SelectFolder () {
  const [path,setPath] = React.useState<String|null>("")
    const {projectDispatch} = React.useContext(ProjectContex);
    const handleClick = async()=>{
    open({
        multiple: false,
        directory: true
    })
    .then(path=>{      
      const processedPath = path?.replace(/\\/g,"/");    
      invoke("save_root_folder",{rootPath:processedPath})
      .then(async()=>{
        const projects = await invoke("get_projects") as string[]       
        projectDispatch(projects)
      })
      .catch(err=>"qual erro?"+err)
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
