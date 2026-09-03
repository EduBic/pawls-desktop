/**
 * This is the main entry point for the UI. You should not need to make any
 * changes here unless you want to update the theme.
 *
 * @see https://github.com/allenai/varnish
 */

import ReactDOM from 'react-dom';
import { BrowserRouter, Route } from 'react-router-dom';

import App from './App';
import { ThemeProvider } from 'styled-components';

const theme = {
    color: {
        T6: "dcdcdc",
        G6: "dcdcdc"
    },
    spacing: {
        md: "16px"
    }
}

import 'antd/dist/antd.css';

ReactDOM.render(
    <ThemeProvider theme={theme}>
        <BrowserRouter>
            <Route path="/" component={App} />
        </BrowserRouter>
    </ThemeProvider>,
    document.getElementById('root')
);
