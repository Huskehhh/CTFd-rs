import React, {useEffect, useState} from 'react';
import axios from 'axios';
import SimpleCard from "../components/SimpleCard";
import '../App.css';

const API_URL = process.env.API_URL;

export default function Home() {
    const [activeCtfs, setActiveCtfs] = useState([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        if (loading) {
            axios.get(API_URL + '/api/v1/active')
                .then(response => {
                    setLoading(false);
                    setActiveCtfs(response.data.data);
                }).catch(err => console.log(err));
        }
    });

    return (
        <div className="App">
            <header className="App-header">
                {!loading && activeCtfs.map((ctf) => {
                    return <div className={"card"}><SimpleCard key={ctf.id} title={ctf.name}
                                                               body1={"Position: " + ctf.stats.position}
                                                               body2={"Points: " + ctf.stats.points}
                                                               button={true} ctfid={ctf.id}/></div>
                })}
            </header>
        </div>
    )
}
