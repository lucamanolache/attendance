import NavBar from "../navBar";
import React, {useState, useEffect} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import {Grid} from "@mui/material";

function Login(props) {
    const [text, setText] = useState('')

    const handleKeyDown = (event) => {
        event.preventDefault();
        if (event.key === 'Enter') {
            const request_options = {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({ password: text })
            }

            fetch("/api/get_cookie", request_options)
                .then(response => {
                    if (response.statusText === "Not Acceptable") {
                        alert("Invalid password");
                    } else {
                        response.text().then(text => {
                            console.log(text);
                            if (text == "true") {
                                props.admin(true);
                                props.login(true);
                            } else {
                                props.login(true);
                            }
                        })
                    }
                    setText('')
                })
        }
    }

    return (
        <React.Fragment>
            <div>
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
                        type="password"
                        label="Password"
                        variant="standard"
                        value={text}
                        onChange={s => setText(s.target.value)}/>
                </Grid>
            </div>
        </React.Fragment>
    );
}

export default Login;
