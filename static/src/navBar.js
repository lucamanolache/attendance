import * as React from 'react';
import PropTypes from 'prop-types';
import Box from '@mui/material/Box';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Typography from '@mui/material/Typography';
import {
    MemoryRouter,
    Route,
    Routes,
    Link,
    matchPath,
    useLocation,
} from 'react-router-dom';
import { StaticRouter } from 'react-router-dom/server';
import Main from "./components/Main";
import AddStudent from "./components/AddStudent";

function Router(props) {
    const { children } = props;
    if (typeof window === 'undefined') {
        return <StaticRouter location="/">{children}</StaticRouter>;
    }

    return (
        <MemoryRouter initialEntries={['/']} initialIndex={0}>
            {children}
        </MemoryRouter>
    );
}

Router.propTypes = {
    children: PropTypes.node,
};

function useRouteMatch(patterns) {
    const { pathname } = useLocation();

    for (let i = 0; i < patterns.length; i += 1) {
        const pattern = patterns[i];
        const possibleMatch = matchPath(pattern, pathname);
        if (possibleMatch !== null) {
            return possibleMatch;
        }
    }

    return null;
}

function MyTabs() {
    // You need to provide the routes in descendant order.
    // This means that if you have nested routes like:
    // users, users/new, users/edit.
    // Then the order should be ['users/add', 'users/edit', 'users'].
    const routeMatch = useRouteMatch(['login', 'current', 'leaderboard', 'stats']);
    const currentTab = routeMatch?.pattern?.path;

    return (
        <Tabs value={currentTab}>
            <Tab label="Login" value="login" to="/" component={Link} />
            <Tab label="At Lab" value="current" to="/current" component={Link} />
            <Tab label="Leader Board" value="leaderboard" to="/leaderboard" component={Link} />
            <Tab label="Statistics" value="stats" to="/stats" component={Link} />
        </Tabs>
    );
}

export default function TabsRouter() {
    return (
        <Router>
            <Box sx={{ width: '100%' }}>
                <MyTabs />
                <Routes>
                    <Route path="/" element={<Main/>} />
                    <Route path="/current" element={<AddStudent />}/>
                </Routes>
            </Box>
        </Router>
    );
}