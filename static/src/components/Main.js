import NavBar from "../navBar";
import React, {useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import {Grid} from "@mui/material";

function Main(props) {
    const [text, setText] = useState('')
    const [helper, setHelper] = useState('')
    const [error, setError] = useState(false)

    const isValid = (t) => {
        let valid = /^\d+$/.test(t);
        valid &= t.length === 8
        valid |= t.length === 0
        valid |= t === "OGMonkeLARA"
        setHelper('')
        if (!valid) {
            setError(true);
            setText(t);
            return false;
        } else {
            setError(false);
            setText(t);
            return true;
        }
    }
    const handleKeyDown = (event) => {
        event.preventDefault();
        if (event.key === 'Enter') {
            if (isValid(text)) {
                if (text == "OGMonkeLARA") {
                    console.log("monke")
                    props.setCoolKid(true);
                    setText('')
                } else {
                    add_student(text)
                }
            }
            setText('')
        }
    }

    const add_student = (text) => {
        console.log("Requesting to add " + text);

        const request_options = {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({ id: parseInt(text) })
        }

        // TODO: add error handling
        fetch("/api/login", request_options)
            .then(response => response.json())
            .then(data => {
                console.log(data);
                if (data.leaving) {
                    setHelper("Goodbye " + data.name + ", you stayed " + Math.round(data.time_spent / 60) + " minutes");
                } else {
                    setHelper("Welcome " + data.name);
                }
            })
    }

    return (
        <React.Fragment>
            <Grid
                component="TextField"
                sx={{
                    '& .MuiTextField-root': { m: 1, width: '25ch' },
                }}
                noValidate
                autoComplete="off"
                onKeyUp={handleKeyDown}
                alignItems="center"
                alignContent="center"
                justifyContent="center"
                container
            >
                <TextField
                    id="login-box"
                    label="Student ID"
                    variant="standard"
                    error={error}
                    value={text}
                    helperText={helper}
                    onChange={s => isValid(s.target.value)}/>
            </Grid>
        </React.Fragment>
    );
}

export default Main;
