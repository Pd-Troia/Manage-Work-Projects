import * as React from "react"
import { invoke } from "@tauri-apps/api/core"
import styles from "./projectList.module.css"

export interface IProjectItemProps {
  projectPath: string
}

export default function ProjectItem({ projectPath }: IProjectItemProps) {
  const projectName = projectPath.split("\\").pop() || projectPath
  const [isOpen, setIsOpen] = React.useState(false)
  const [subDirs, setSubDirs] = React.useState<string[]>([])
  const [loaded, setLoaded] = React.useState(false)
  const [buffer, setBuffer] = React.useState<string[]>([])
  const [dropUp, setDropUp] = React.useState(false)
  const containerRef = React.useRef<HTMLDivElement>(null)

  const openBuffered = (dirs: string[]) => {
    dirs.forEach((dir) => {
      invoke("open_single_dir", { dirPath: dir }).catch((err) => console.log("erro:", err))
    })
  }

  const closeDropdown = (currentBuffer: string[]) => {
    setIsOpen(false)
    setBuffer([])
    if (currentBuffer.length > 0) openBuffered(currentBuffer)
  }

  React.useEffect(() => {
    if (!isOpen) return
    const handleClickOutside = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setBuffer((prev) => { closeDropdown(prev); return [] })
      }
    }
    document.addEventListener("mousedown", handleClickOutside)
    return () => document.removeEventListener("mousedown", handleClickOutside)
  }, [isOpen])

  const handleMainClick = () => {
    invoke("open_project", { projectPath })
      .then(() => console.log("projeto aberto"))
      .catch((err) => console.log("erro:", err))
  }

  const handleExpandClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    if (isOpen) {
      setBuffer((prev) => { closeDropdown(prev); return [] })
      return
    }
    // Detect if dropdown would go below viewport
    if (containerRef.current) {
      const rect = containerRef.current.getBoundingClientRect()
      setDropUp(rect.bottom + 200 > window.innerHeight)
    }
    if (!loaded) {
      invoke("get_project_dirs", { projectPath })
        .then((dirs) => {
          setSubDirs(dirs as string[])
          setLoaded(true)
        })
        .catch((err) => console.log("erro ao buscar dirs:", err))
    }
    setIsOpen(true)
  }

  const handleSubDirClick = (dirPath: string) => {
    setBuffer((prev) =>
      prev.includes(dirPath) ? prev.filter((d) => d !== dirPath) : [...prev, dirPath]
    )
  }

  const handleLoginClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    invoke("vendor_login", { projectPath })
      .then(() => console.log("login iniciado"))
      .catch((err) => console.log("erro login:", err))
  }

  return (
    <div className={styles.item} ref={containerRef}>
      <div className={styles.itemMain} onClick={handleMainClick} title={projectName}>
        {projectName}
      </div>
      <div
        className={`${styles.itemExpand} ${isOpen ? styles.itemExpandOpen : ""}`}
        onClick={handleExpandClick}
        title="Ver subprojetos"
      >
        {isOpen ? "▲" : "▼"}
      </div>
      {isOpen && (
        <div className={`${styles.dropdown} ${dropUp ? styles.dropdownUp : ""}`}>
          <div className={styles.dropdownActions}>
            <button className={styles.loginBtn} onClick={handleLoginClick} title="vtex login">
              login
            </button>
            {buffer.length > 0 && (
              <span className={styles.bufferCount} title="itens selecionados">
                {buffer.length} selecionado{buffer.length > 1 ? "s" : ""}
              </span>
            )}
          </div>
          {subDirs.length === 0 ? (
            <div className={styles.dropdownEmpty}>sem subprojetos</div>
          ) : (
            subDirs.map((dir) => {
              const name = dir.split("\\").pop() || dir
              const selected = buffer.includes(dir)
              return (
                <div
                  key={dir}
                  className={`${styles.dropdownItem} ${selected ? styles.dropdownItemSelected : ""}`}
                  onClick={() => handleSubDirClick(dir)}
                  title={name}
                >
                  {name}
                </div>
              )
            })
          )}
        </div>
      )}
    </div>
  )
}
