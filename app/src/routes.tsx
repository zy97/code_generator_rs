import {
    createBrowserRouter,
    RouterProvider,
} from "react-router-dom";
import App from "./App";

export const ROUTES = createBrowserRouter([
    {
        path: "/",
        element: <App />,
        children: [
            { index: true, element: <div>hello</div> },
            {
                path: "/sysmanage",
                children: [
                    { index: true, element: <div>hello</div> },
                    { path: "user", element: <div>hello</div> },
                    { path: "role", element: <div>hello</div> },
                    { path: "auditlog", element: <div>hello</div> },
                    { path: "emailsetting", element: <div>hello</div> },
                    { path: "tenants", element: <div>hello</div> },
                    { path: "systemSetting", element: <div>hello</div> },
                ],
            },
            {
                path: "*",
                element: <div >未找到</div>,
            },
        ],
    },
]);


