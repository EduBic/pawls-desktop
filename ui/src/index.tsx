/**
 * This is the main entry point for the UI. You should not need to make any
 * changes here unless you want to update the theme.
 *
 * @see https://github.com/allenai/varnish
 */

import ReactDOM from 'react-dom';
import { BrowserRouter, Route } from 'react-router-dom';

import App from './App';

ReactDOM.render(
    <BrowserRouter>
        <Route path="/" component={App} />
    </BrowserRouter>,
    document.getElementById('root')
);
