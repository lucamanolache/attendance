import React, {useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import {Typography} from "@mui/material";

function Stats() {
    const [data, setData] = useState([])

    return (
        <React.Fragment>
            <Box
                component="TextField"
                sx={{
                    '& .MuiTextField-root': { m: 1, width: '25ch' },
                }}
                noValidate
                autoComplete="off"
            >
                    <Typography> Hello </Typography>
            </Box>
        </React.Fragment>
    );
}

export default Stats;
