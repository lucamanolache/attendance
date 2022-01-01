import NavBar from "../navBar";
import React, {useEffect, useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import {Grid, List, ListItem, ListItemText} from "@mui/material";

function Current() {
    const [students, setStudents] = useState([])

    useEffect(() => {
        const request_options = {
            method: 'GET',
        }

        fetch("/api/get_here", request_options)
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
    }, [])

    return (
        <React.Fragment>
            <Grid
                alignItems="center"
                justifyContent="center"
                container
            >
                <List
                    sx={{
                        width: '100%',
                        maxWidth: 360,
                        position: 'relative',
                        overflow: 'auto',
                        maxHeight: 300,
                    }}
                >
                    <li>
                        {students.map((item) => (
                            <ul>
                                <ListItem key={item.id}>
                                    <ListItemText primary={item.name}/>
                                </ListItem>
                            </ul>
                        ))}
                    </li>
                </List>
            </Grid>
        </React.Fragment>
    );
}

export default Current;
