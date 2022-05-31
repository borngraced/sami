import {ReactNode} from "react";

interface LayoutIt {
    isFor: 'page' | 'article';
    children: ReactNode;
}

const Layout = ({ isFor, children }:LayoutIt) => {
return (
    <div className="block max-w-screen-lg mx-auto p-2 md:p-4">
        {children}
    </div>
)
}

export default Layout

Layout.defaultProps = {
    isFor: 'page'
}