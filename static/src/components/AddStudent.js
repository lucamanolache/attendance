import NavBar from "../navBar";
import React, {useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';

function AddStudent() {
    const [id, setId] = useState('')
    const [name, setName] = useState('')
    const [error, setError] = useState(false)

    const isValidName = (t) => {
        setName(t);
    }
    const isValidId = (t) => {
        let valid = /^\d+$/.test(t);
        valid &= t.length === 8
        valid |= t.length === 0
        if (!valid) {
            setError(true);
            setId(t);
            return false;
        } else {
            setError(false);
            setId(t);
            return true;
        }
    }
    const handleKeyDown = (event) => {
        event.preventDefault();
        if (event.key === 'Enter') {
            if (isValidId(id)) {
                login(id)
            }
            setId('')
        }
    }

    const login = (id) => {
        console.log("Requesting to add " + id);

        const request_options = {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({ id: parseInt(id), name: name })
        }

        fetch("http://127.0.0.1:3030/api/add_students", request_options).then(r => console.log("Added student"))
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
                    id="id-box"
                    label="Student ID"
                    variant="standard"
                    error={error}
                    value={id}
                    onChange={s => isValidId(s.target.value)}/>
                <TextField
                    id="name-box"
                    label="Student Name"
                    variant="standard"
                    value={name}
                    onChange={s => isValidName(s.target.value)}/>
            </Box>
        </React.Fragment>
    );
}

export default AddStudent;
