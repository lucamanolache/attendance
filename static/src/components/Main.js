import Routes from "../routes";
import React, {useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';

function Main() {
    const [text, setText] = useState('')
    const [error, setError] = useState(false)
    const isValid = (t) => {
        let valid = /^\d+$/.test(t);
        valid &= t.length === 8
        valid |= t.length === 0
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
                login(text)
            }
            setText('')
        }
    }

    const login = (text) => {
        console.log("Requesting to add " + text);

        const request_options = {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({ id: parseInt(text) })
        }

        // TODO: add error handling
        fetch("http://127.0.0.1:3030/api/login", request_options)
            .then(response => response.json())
            .then(data => {
                console.log(data)
            })
    }

    return (
        <React.Fragment>
            <Box
                component="TextField"
                sx={{
                    '& .MuiTextField-root': { m: 1, width: '25ch' },
                }}
                noValidate
                autoComplete="off"
                onKeyUp={handleKeyDown}
            >
                <TextField
                    id="login-box"
                    label="Student ID"
                    variant="standard"
                    error={error}
                    value={text}
                    onChange={s => isValid(s.target.value)}/>
            </Box>
        </React.Fragment>
    );
}

export default Main;
