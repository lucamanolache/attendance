import React, {useEffect, useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import {Typography} from "@mui/material";
import { AxisOptions, Chart } from "react-charts";

function Stats() {
    const [data, setData] = useState([])

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
        fetch("/api/get_stats", request_options)
            .then(response => response.json())
            .then(response => {
                console.log("Got students")
                var arraysMatch = function (arr1, arr2) {
                    if (arr1.length !== arr2.length) return false;

                    for (var i = 0; i < arr1.length; i++) {
                        if (arr1[i].id !== arr2[i].id) return false;
                    }

                    return true;
                };

                if (!arraysMatch(response, data)) {
                    setData(response)
                    console.log(response)
                }
            })
    }

    const primaryAxis = React.useMemo<
        AxisOptions<typeof data[number]["data"][number]>
        >(
            () => ({
                getValue: (datum) => datum.primary as unknown as Date,
            }),
                []
        );

    const secondaryAxes = React.useMemo<
        AxisOptions<typeof data[number]["data"][number]>[]
        >(
            () => [
                {
                    getValue: (datum) => datum.secondary,
                },
            ],
                []
        );

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
                    <Chart options={{
                        data.hours_time,
                        primaryAxis,
                        secondaryAxes,
                    }}/>
            </Box>
        </React.Fragment>
    );
}

export default Stats;
