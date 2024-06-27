import { useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import "../App.css";
import "../index.css";
import { Link, Outlet } from "react-router-dom";



export default function Navbar() {

  
  return(
    <div className="drawer">
    <input id="my-drawer-3" type="checkbox" className="drawer-toggle" />
    <div className="drawer-content flex flex-col">
      {/* Navbar */}
      <div className="navbar bg-base-300 w-full">
        <div className="flex-none lg:hidden">
          <label htmlFor="my-drawer-3" aria-label="open sidebar" className="btn btn-square btn-ghost">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              className="inline-block h-6 w-6 stroke-current">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M4 6h16M4 12h16M4 18h16"></path>
            </svg>
          </label>
        </div>
        <div className="mx-2 flex-1 px-2">Hermes</div>
        <div className="hidden flex-none lg:block">
          <ul className="menu menu-horizontal">
            {/* Navbar menu content here */}
            <li><Link to={`share`}>Share Files</Link></li>
            <li><Link to={`recieve`}>Recieve Files</Link></li>
          </ul>
        </div>
      </div>
      {/* {activeComponent == 0 ? <ShareButton/> : <RecieveScreen/>} */}
      <Outlet />
    </div>
    <div className="drawer-side">
      <label htmlFor="my-drawer-3" aria-label="close sidebar" className="drawer-overlay"></label>
      <ul className="menu bg-base-200 min-h-full w-80 p-4">
        {/* Sidebar content here */}
        <li><Link to={`share`}>Share Files</Link></li>
        <li><Link to={`recieve`}>Recieve Files</Link></li>
      </ul>
    </div>
  </div>
  )
}