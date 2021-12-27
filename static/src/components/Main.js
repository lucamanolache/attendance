import Routes from "../routes";
import React from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';

function Main() {
    return (
        <React.Fragment>
            <Box
                component="form"
                sx={{
                    '& > :not(style)': { m: 1, width: '25ch' },
                }}
                noValidate
                autoComplete="off"
            >
                <TextField id="login-box" label="Student ID" variant="standard" />
            </Box>
        </React.Fragment>
    );
}

export default Main;
