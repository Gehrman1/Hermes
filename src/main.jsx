import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Navbar from "./components/Navbar";
import {
  createBrowserRouter,
  Navigate,
  RouterProvider,
} from "react-router-dom";
import ShareButton from "./components/share_button";
import RecieveScreen from "./components/recieve_screen";

const router = createBrowserRouter([
  {
    
    path: "/",
    element: <Navbar/>,
    children: [
      {
        path: "",
        element: <Navigate to="share" />,
      },
      {
        path: "share",
        element: <ShareButton />,
      },
      {
        path: "recieve",
        element: <RecieveScreen />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);