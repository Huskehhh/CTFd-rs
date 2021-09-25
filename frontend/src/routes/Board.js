import React, {useEffect, useState} from 'react';
import '../styles/Board.sass';
import axios from "axios";
import {Link, useParams} from "react-router-dom";
import HomeIcon from "@material-ui/icons/Home";

const PRIORITY_HIGHEST = 'HIGHEST';
const PRIORITY_HIGH = 'HIGH';
const PRIORITY_MEDIUM = 'MEDIUM';
const PRIORITY_LOW = 'LOW';

const TYPE_TODO = 'TODO';
const TYPE_INPROGRESS = 'INPROGRESS';
const TYPE_DONE = 'DONE';

const TITLE_TODO = 'Todo';
const TITLE_INPROGRESS = 'In Progress';
const TITLE_DONE = 'Done';

const CLASS_TASK = 'taskCard';
const CLASS_COLUMN = 'boardColumn';

const API_URL = process.env.REACT_APP_API_URL;

function TaskCard(props) {
    const task = props.task;

    let getPriorityClass = (priority) => {
        if (priority === PRIORITY_LOW) {
            return CLASS_TASK + '--' + PRIORITY_LOW;
        } else if (priority === PRIORITY_MEDIUM) {
            return CLASS_TASK + '--' + PRIORITY_MEDIUM;
        } else if (priority === PRIORITY_HIGH) {
            return CLASS_TASK + '--' + PRIORITY_HIGH;
        } else if (priority === PRIORITY_HIGHEST) {
            return CLASS_TASK + '--' + PRIORITY_HIGHEST;
        }
    };

    const priorityClass = getPriorityClass(task.priority);

    return (
        <div className={`taskCard ${priorityClass}`}>
            <header className="taskCard__header">
                <h4 className="taskCard__title">{task.title}</h4>
                <div className="taskCard__epicLink-wrapper">
                    <span className="taskCard__epicLink-title">{task.category}</span>
                </div>
                <h5>Points: {task.points}</h5>
                {!task.solved && task.working && <h5>Working: {task.working}</h5>}
                {task.solved && task.solver && <h5>Solved by {task.solver} @ {task.solved_time}</h5>}
            </header>

            <footer className="taskCard__footer">
                <span className="taskCard__type">Priority: {task.priority}</span>
            </footer>
        </div>
    );
}

function BoardColumn(props) {
    const columnClass = getColumnClass(props.title);

    function getColumnClass(title) {
        if (title === TITLE_TODO) {
            return CLASS_COLUMN + '--' + TYPE_TODO;
        } else if (title === TITLE_INPROGRESS) {
            return CLASS_COLUMN + '--' + TYPE_INPROGRESS;
        } else if (title === TITLE_DONE) {
            return CLASS_COLUMN + '--' + TYPE_DONE;
        }
    }

    return (
        <div className={`boardColumn ${columnClass}`}>
            <header className="boardColumn__header">
                <h1 className="boardColumn__title">{props.title}</h1>
            </header>
            <div className="boardColumn__taskList">
                {props.tasks.map((task) => <TaskCard task={task}/>)}
            </div>
        </div>
    );
}

export default function Board() {
    let {CtfId} = useParams();
    const [loading, setLoading] = useState(true);
    const [tasks, setTasks] = useState([]);
    const [position, setPosition] = useState(0);
    const [points, setPoints] = useState(0);

    useEffect(() => {
        if (loading) {
            // Load the data initially
            getData();

            // Set a timer to update the data every 10 seconds
            setTimeout(() => getData(), 10000);
        }
    });

    let getData = () => {
        axios.get(API_URL + '/api/v1/' + CtfId + "/challenges")
            .then(response => {
                setLoading(false);
                setTasks(response.data.data);
            }).catch(err => console.log(err));

        axios.get(API_URL + '/api/v1/' + CtfId + "/stats")
            .then(response => {
                setPosition(response.data.position);
                setPoints(response.data.points);
            }).catch(err => console.log(err));
    };

    return (
        <div>
            <header>
                <Link to={"/"}><HomeIcon style={{color: 'gray'}}/></Link>
                <h5 className={"stats"}>Scoreboard position: {position}, Total points: {points}</h5>
            </header>
            <div className="board">
                <BoardColumn title={TITLE_TODO}
                             tasks={tasks.filter(task => {
                                 return task.status === TYPE_TODO
                             })}/>

                <BoardColumn title={TITLE_INPROGRESS}
                             tasks={tasks.filter(task => {
                                 return task.status === TYPE_INPROGRESS
                             })}/>

                <BoardColumn title={TITLE_DONE}
                             tasks={tasks.filter(task => {
                                 return task.status === TYPE_DONE
                             })}/>
            </div>
        </div>
    );
}