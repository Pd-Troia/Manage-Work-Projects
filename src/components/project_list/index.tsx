import * as React from 'react';
import styles from "./projectList.module.css"
import { invoke } from '@tauri-apps/api/core';
import ProjectItem from './ProjectItem';
export interface IProjectListProps {
}

export default function ProjectList ({}: IProjectListProps) {
    const [projectNames,setprojectNames] = React.useState<string[]>([])
    React.useEffect(()=>{
        (async()=>{
            const projects = await invoke("get_projects") as string[]            
            setprojectNames(projects)
        })()        
    },[])    
    return (
    <div className={styles.container}>
        <h1>Lista de Projetos</h1>
        <div className={styles.wrapper}>
            {projectNames.map((item)=>{
                return (
                    <ProjectItem key={item} projectPath={item}/>
                )
            })}
        </div>
    </div>
  );
}
