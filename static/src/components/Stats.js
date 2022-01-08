import React, {useEffect, useState} from "react";
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import { ResponsiveLine } from '@nivo/line'


function Stats() {
    const [data, setData] = useState(
        []
    )

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
                console.log(response)
                var arraysMatch = function (arr1, arr2) {
                    if (arr1.length !== arr2.length) return false;

                    for (var i = 0; i < arr1.length; i++) {
                        if (arr1[i].id !== arr2[i].id) return false;
                    }

                    return true;
                };

                setData(response['hours_time'])
            })
    }

    return (
            <Box
            height={500}
            width={1000}>
                <ResponsiveLine
                    data={data}
                    margin={{ top: 50, right: 110, bottom: 50, left: 60 }}
                    xScale={{ type: 'point' }}
                    yScale={{ type: 'linear', min: 'auto', max: 'auto', stacked: false, reverse: false }}
                    yFormat=" >-.2f"
                    axisTop={null}
                    axisRight={null}
                    axisBottom={{
                        orient: 'bottom',
                        tickSize: 5,
                        tickPadding: 5,
                        tickRotation: 0,
                        legend: 'Date',
                        legendOffset: 36,
                        legendPosition: 'middle'
                    }}
                    axisLeft={{
                        orient: 'left',
                        tickSize: 5,
                        tickPadding: 5,
                        tickRotation: 0,
                        legend: 'Hours',
                        legendOffset: -40,
                        legendPosition: 'middle'
                    }}
                    pointSize={10}
                    pointColor={{ theme: 'background' }}
                    pointBorderWidth={2}
                    pointBorderColor={{ from: 'serieColor' }}
                    pointLabelYOffset={-12}
                    useMesh={true}
                    legends={[
                        {
                            anchor: 'bottom-right',
                            direction: 'column',
                            justify: false,
                            translateX: 100,
                            translateY: 0,
                            itemsSpacing: 0,
                            itemDirection: 'left-to-right',
                            itemWidth: 80,
                            itemHeight: 20,
                            itemOpacity: 0.75,
                            symbolSize: 12,
                            symbolShape: 'circle',
                            symbolBorderColor: 'rgba(0, 0, 0, .5)',
                            effects: [
                                {
                                    on: 'hover',
                                    style: {
                                        itemBackground: 'rgba(0, 0, 0, .03)',
                                        itemOpacity: 1
                                    }
                                }
                            ]
                        }
                    ]}
                />
            </Box>
    );
}

export default Stats;
