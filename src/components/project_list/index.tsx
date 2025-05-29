import * as React from 'react';
import styles from "./projectList.module.css"
import { invoke } from '@tauri-apps/api/core';
import ProjectItem from './ProjectItem';
import Config from '../config';
export interface IProjectContext {
    projectDispatch: React.Dispatch<React.SetStateAction<string[]>>
}

const DEFAULT_STATE = {
    projectDispatch: ()=>{}
}
export const ProjectContex = React.createContext<IProjectContext>(DEFAULT_STATE);
export default function ProjectList ({}) {
    const [projectNames,setProjectNames] = React.useState<string[]>([])
    React.useEffect(()=>{
        (async()=>{
            const projects = await invoke("get_projects") as string[]            
            setProjectNames(projects)
        })()        
    },[])    
    return (
    <ProjectContex.Provider value={{projectDispatch:setProjectNames}}>       
        <div className={styles.container}>
            <div className={styles.heeaderContainer}>
                <h1>Lista de Projetos</h1>
                <Config/>
            </div>
            <div className={styles.wrapper}>
                {projectNames.map((item)=>{
                    return (
                        <ProjectItem key={item} projectPath={item}/>
                    )
                })}
            </div>
        </div>
    </ProjectContex.Provider>
  );
}
