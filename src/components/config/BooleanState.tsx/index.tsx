import * as React from 'react';
import styles from "./boolean.module.css"
export interface IBooleanStateProps {
    text: string,
    handleChange: React.ChangeEventHandler<HTMLInputElement>,
    checked: boolean

}

export default function BooleanState ({text,handleChange,checked}: IBooleanStateProps) {
  return (
    <div className={styles.booleanContainer}>
        <span className={styles.text}>{text}</span>
        <form onSubmit={(e)=>e.preventDefault()}>
            <input className={styles.checkbox} checked={checked} type="checkbox" onChange={handleChange}/>
        </form>
    </div>
  );
}
