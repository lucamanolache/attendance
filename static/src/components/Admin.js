import NavBar from "../navBar";
import React, {useEffect, useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import {Grid, Table} from "@mui/material";
import { DataGrid } from '@mui/x-data-grid';

function Admin(props) {
    const [memberData, setMemberData] = useState([])

    const columns = [
        { field: 'id', headerName: 'ID', width: 100, },
        { field: 'name', headerName: 'Name', width: 350 },
        { field: 'subteam', headerName: 'Subteam', width: 150, },
        { field: 'total_time', headerName: 'Hours', width: 160 },
    ]

    useEffect(() => {
        getStudents();
    }, [])

    const getStudents = () => {
        console.log("Getting leaderboard");

        const request_options = {
            method: 'GET',
            headers: {'Content-Type': 'application/json'},
        }

        // TODO: add error handling
        fetch("/api/get_all", request_options)
            .then(response => response.json())
            .then(data => {
                console.log("Got students")
                var arraysMatch = function (arr1, arr2) {
                    if (arr1.length !== arr2.length) return false;

                    for (var i = 0; i < arr1.length; i++) {
                        if (arr1[i].id !== arr2[i].id) return false;
                    }

                    return true;
                };

                if (!arraysMatch(data, memberData)) {
                    setMemberData(data)
                    console.log(data)
                }
            })
    }

    return (
        <React.Fragment>
            <Grid
                autoComplete="off"
                alignItems="center"
                alignContent="center"
                justifyContent="center"
                height={1000}
                width='100%'
                container
            >
                    <DataGrid
                        rows={memberData}
                        columns={columns}
                        rowsPerPageOptions={[20]}
                        pageSize={17}
                    />
            </Grid>
        </React.Fragment>
    );
}

export default Admin;
