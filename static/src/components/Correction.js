import NavBar from "../navBar";
import React, {useEffect, useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import {Grid, List, ListItem, ListItemText} from "@mui/material";

function Correction() {
    const [students, setStudents] = useState([])
    const [id, setId] = useState("")
    const [logout, setLogout] = useState("")
    const [login, setLogin] = useState("")

    const fetchStudents = () => {
        const request_options = {
            method: 'GET',
        }

        fetch("/api/needs_corrections", request_options)
            .then(response => response.json())
            .then(response => {
                console.log("Got students")
                response.sort()

                var arraysMatch = function (arr1, arr2) {
                    if (arr1.length !== arr2.length) return false;

                    for (var i = 0; i < arr1.length; i++) {
                        if (arr1[i].id !== arr2[i].id) return false;
                    }

                    return true;
                };

                if (!arraysMatch(response, students)) {
                    setStudents(response)
                }
            })
    }

    useEffect(() => {
        fetchStudents()
    }, [])

    const handleKeyDown = (event) => {
        event.preventDefault();
        if (event.key === 'Enter') {
            correct()
            setId("")
            setLogout("")
            setLogin("")
            fetchStudents()
        }
    }

    const correct = () => {
        console.log("Requesting to correct " + id);

        let body = JSON.stringify({
            id: parseInt(id),
            login: login,
            logout: logout
        })

        console.log(body)

        const request_options = {
            method: 'POST',
            headers: {'Content-Type': 'application/json',},
            body: JSON.stringify({
                id: parseInt(id),
                login_time: login,
                logout_time: logout
            })
        }

        fetch("/api/correction", request_options).then(r => console.log(r))
    }

    return (
        <React.Fragment>
            <Grid
                alignItems="center"
                justifyContent="center"
                container
                onKeyUp={handleKeyDown}
            >
                <div>
                    <Grid
                        component="TextField"
                        sx={{
                            '& .MuiTextField-root': { m: 1, width: '25ch' },
                        }}
                        noValidate
                        autoComplete="off"
                        alignItems="center"
                        alignContent="center"
                        justifyContent="center"
                        container
                    >
                        <TextField
                            id="id-box"
                            label="Student ID"
                            variant="standard"
                            value={id}
                            onChange={s => setId(s.target.value)}/>
                        <TextField
                            id="login-box"
                            label="Login Time"
                            variant="standard"
                            value={login}
                            onChange={s => setLogin(s.target.value)}/>
                        <TextField
                            id="logout-box"
                            label="Logout Time"
                            variant="standard"
                            value={logout}
                            onChange={s => setLogout(s.target.value)}/>
                    </Grid>
                </div>
                <List
                    sx={{
                        width: '100%',
                        maxWidth: 800,
                        position: 'relative',
                        overflow: 'auto',
                        maxHeight: 1000,
                    }}
                >
                    <li>
                        {students.map((item) => (
                            <ul>
                                <ListItem key={item.id}>
                                    <ListItemText primary={item.name}/>
                                    <ListItemText primary={item.id}/>
                                    <ListItemText primary={item.login_time}/>
                                </ListItem>
                            </ul>
                        ))}
                    </li>
                </List>
            </Grid>
        </React.Fragment>
    );
}

export default Correction;
