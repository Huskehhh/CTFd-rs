import React from 'react';
import {makeStyles} from '@material-ui/core/styles';
import Card from '@material-ui/core/Card';
import CardActions from '@material-ui/core/CardActions';
import CardContent from '@material-ui/core/CardContent';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';
import {Link} from "react-router-dom";

const useStyles = makeStyles({
    root: {
        minWidth: 225,
    },
    title: {
        fontSize: 14,
    },
    pos: {
        marginBottom: 12,
    },
});

export default function SimpleCard(props) {
    const classes = useStyles();

    return (
        <Card className={classes.root}>
            <CardContent>
                <Typography variant="h5" component="h2">
                    {props.title}
                </Typography>
                <Typography className={classes.pos} color="textSecondary">
                    {props.body1}
                </Typography>
                <Typography variant="body2" component="p">
                    {props.body2}
                </Typography>
            </CardContent>
            <CardActions>
                {props.button &&
                <Link to={"/ctf/" + props.ctfid}><Button size="small">View board</Button></Link>}
            </CardActions>
        </Card>
    );
}
