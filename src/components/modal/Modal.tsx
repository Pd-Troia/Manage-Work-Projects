import * as React from 'react';
import style from "./modal.module.css"
import close from "../../assets/excluir-icon.svg"
export interface IModalProps {
    isOpen: boolean
    handleClose: ()=>void,
    children: React.ReactElement
}

export default function Modal ({isOpen,handleClose, children}: IModalProps) {
  React.useEffect(() => {
    if (!isOpen) return
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") handleClose() }
    document.addEventListener("keydown", onKey)
    return () => document.removeEventListener("keydown", onKey)
  }, [isOpen, handleClose])

  return (
    <>
      {
      isOpen
        ?(
          <div className={style.overlay} onClick={handleClose}>
            <div className={style.modal} onClick={(e) => e.stopPropagation()}>
              <div className={style.modalHeader}>
                <div className={style.closeContainer}>
                  <img onClick={handleClose} src={close} alt="icon-close" />
                </div>
              </div>
              <div className={style.modalBody}>
                {children}
              </div>
            </div>
          </div>
        )
        
        :<></>
    }
    </>  
  );
}
