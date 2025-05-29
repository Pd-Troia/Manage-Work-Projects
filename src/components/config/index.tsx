import * as React from 'react';
import configurationIcon from "../../assets/config-icon.svg"
import Modal from '../modal/Modal';
import SelectFolder from './SelectFolder/SelectFolder';
import BooleanState from './BooleanState.tsx';
import { invoke } from '@tauri-apps/api/core';
export interface IConfigProps {
}
interface Settings{
    root_folder: string,
    default_login: boolean
}
export default function Config ({}: IConfigProps) {
    const [isModalOpen,setIsModalOpen] = React.useState(false)
    const [config,setConfig] = React.useState<Settings|null>()
    React.useEffect((()=>{
         invoke<Settings>("get_config").then((data)=>{          
            setConfig(data);
        })
    }),[])  
    const changeDefaultLogin = (e:React.ChangeEvent<HTMLInputElement>)=>{
       const isChecked = e.currentTarget.checked ? true : false
       invoke("save_default_login",{
        state:!isChecked
       }).then(()=>{
            if(!config) return
            const obj:Settings = {...config,default_login:isChecked} 
            setConfig(obj)
       }
       )
    }
    return (
    <div>
        <img onClick={()=>setIsModalOpen(true)} src={configurationIcon} alt="config-icon" />
        <Modal handleClose={()=>setIsModalOpen(false)} isOpen={isModalOpen}>
            <>
                <SelectFolder/>
                <BooleanState text='Default login' checked={config?.default_login===true?true:false} handleChange={changeDefaultLogin}/>
            </>
        </Modal>
    </div>
  );
}
