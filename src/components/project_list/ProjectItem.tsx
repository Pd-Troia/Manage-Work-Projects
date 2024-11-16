import styles from "./projectList.module.css"
export interface IProjectItemProps {
  projectPath:string
}

export default function ProjectItem ({projectPath}: IProjectItemProps) {
  const projectName = projectPath.split(/\\/)[1]
  
  return (
    <div className={styles.item}>
      {projectName}
    </div>
  );
}
