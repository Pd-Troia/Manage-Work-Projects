import * as React from 'react';
import style from "./modal.module.css"
import close from "../../assets/excluir-icon.svg"
export interface IModalProps {
    isOpen: boolean
    handleClose: ()=>void,
    children: React.ReactElement
}

export default function Modal ({isOpen,handleClose, children}: IModalProps) {
  return (
    <>
      {
      isOpen 
        ?(
          <div className={style.overlay}>
            <div className={style.modal}>
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
