import { invoke } from "@tauri-apps/api/core"
import styles from "./projectList.module.css"
export interface IProjectItemProps {
  projectPath:string
}

export default function ProjectItem ({projectPath}: IProjectItemProps) {
  const projectName = projectPath.split(/\\/)[1]
  const handleClick = ()=>{   
    invoke("open_project",{projectPath})
    .then(_=>console.log("rodou com sucesso"))
    .catch(err=>console.log("deu erro:",err))
  }
  return (
    <div className={styles.item} onClick={handleClick}>
      {projectName}
    </div>
  );
}
