import NavBar from "../navBar";
import React, {useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import {List, ListItem, ListItemText} from "@mui/material";

function Current() {
    const [students, setStudents] = useState([])

    const request_options = {
        method: 'GET',
    }

    fetch("http://127.0.0.1:3030/api/get_here", request_options)
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

    return (
        <React.Fragment>
            <Box
                alignItems="center"
                justifyContent="center"
            >
                <List
                    sx={{
                        width: '100%',
                        maxWidth: 360,
                        position: 'relative',
                        overflow: 'auto',
                        maxHeight: 300,
                        '& ul': { padding: 0 },
                    }}
                    subheader={<li />}>
                    {students.map((item) => (
                        <ListItem key={item.id}>
                            <ListItemText primary={item.name}/>
                        </ListItem>
                    ))}
                </List>
            </Box>
        </React.Fragment>
    );
}

export default Current;
