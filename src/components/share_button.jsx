import { invoke } from "@tauri-apps/api"
import { useState } from "react";

async function share_file() {
    console.log("Here")
    invoke('my_custom_command').then((message) => console.log(message))
    
  }

export default function ShareButton() {
  const [serverOnline, setServerOnline] = useState(false);
  const handleClick = async ()  => {
    await share_file()
    setServerOnline(true);
  };
  
    return(
        <div className="drawer-content flex flex-col items-center justify-center pt-20 mt-40">
     <div className="drawer-content flex flex-col items-center justify-center">
     <button className="btn" onClick={() => {
      share_file()
      setServerOnline(true)
     }}>
  Share
  <svg
    xmlns="http://www.w3.org/2000/svg"
    className="h-6 w-6"
    fill="none"
    viewBox="0 0 512 512"
    stroke="currentColor">
    <path
    fill = "#AB7C94"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      d="M307 34.8c-11.5 5.1-19 16.6-19 29.2v64H176C78.8 128 0 206.8 0 304C0 417.3 81.5 467.9 100.2 478.1c2.5 1.4 5.3 1.9 8.1 1.9c10.9 0 19.7-8.9 19.7-19.7c0-7.5-4.3-14.4-9.8-19.5C108.8 431.9 96 414.4 96 384c0-53 43-96 96-96h96v64c0 12.6 7.4 24.1 19 29.2s25 3 34.4-5.4l160-144c6.7-6.1 10.6-14.7 10.6-23.8s-3.8-17.7-10.6-23.8l-160-144c-9.4-8.5-22.9-10.6-34.4-5.4z" />
  </svg>
</button>

{serverOnline == true ? (<div>Server Online</div>) : (<div>Server Offline</div>)}
      </div>
      </div>
    )
} 


