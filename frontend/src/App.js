import React from 'react';
import './App.css';
import {
    BrowserRouter as Router,
    Switch,
    Route,
} from "react-router-dom";
import Home from "./routes/Home";
import Board from "./routes/Board";

export default function App() {
    return (
        <Router>
            <Switch>
                <Route exact path="/">
                    <Home/>
                </Route>
                <Route path={"/ctf/:CtfId"}>
                    <Board/>
                </Route>
            </Switch>
        </Router>
    )
}
