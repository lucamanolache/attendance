import NavBar from "../navBar";
import React, {useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';

function Current() {
    const [students, setStudents] = useState([])

    const request_options = {
        method: 'GET',
    }

    fetch("http://127.0.0.1:3030/api/get_here", request_options)
        .then(response => response.json())
        .then(response => {
            console.log(response)
            if (response !== students) {
                setStudents(response)
            }
            })

    return (
        <React.Fragment>
        </React.Fragment>
    );
}

export default Current;
