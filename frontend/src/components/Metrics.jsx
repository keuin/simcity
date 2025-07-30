import React from 'react'
import {Bar} from 'react-chartjs-2'
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    BarElement,
    Title,
    Tooltip,
    Legend,
} from 'chart.js'

ChartJS.register(
    CategoryScale,
    LinearScale,
    BarElement,
    Title,
    Tooltip,
    Legend
)

const Metrics = ({metrics}) => {
    if (!metrics) return null

    const {
        timestamp,
        average_commute_time,
        average_distance,
        energy_usage,
        max_congestion,
        most_congested_position,
    } = metrics

    const congestionData = Object.entries(metrics.congestion_map || {})
        .sort((a, b) => b[1] - a[1]) // sort by congestion descending
        .slice(0, 5) // Take top 5

    const chartData = {
        labels: congestionData.map(([pos]) => pos),
        datasets: [
            {
                label: 'Congestion level',
                data: congestionData.map(([_, value]) => value),
                backgroundColor: 'rgba(255, 99, 132, 0.5)',
                borderColor: 'rgba(255, 99, 132, 1)',
                borderWidth: 1,
            },
        ],
    }

    const chartOptions = {
        responsive: true,
        plugins: {
            legend: {
                position: 'top',
            },
            title: {
                display: true,
                text: 'Most congested locations',
            },
        },
        scales: {
            y: {
                beginAtZero: true,
                title: {
                    display: true,
                    text: 'Agent count',
                },
            },
            x: {
                title: {
                    display: true,
                    text: 'Position (x,y)',
                },
            },
        },
    }

    return (
        <div className="metrics mb-4">
            <div className="row">
                <div className="col-md-6">
                    <table className="table table-sm">
                        <tbody>
                        <tr>
                            <th>Simulation Time</th>
                            <td>{timestamp} seconds</td>
                        </tr>
                        <tr>
                            <th>Average Commute Time</th>
                            <td>{average_commute_time.toFixed(2)} seconds</td>
                        </tr>
                        <tr>
                            <th>Average Distance</th>
                            <td>{average_distance.toFixed(2)} cells</td>
                        </tr>
                        <tr>
                            <th>Energy Usage</th>
                            <td>{energy_usage.toFixed(2)} units</td>
                        </tr>
                        <tr>
                            <th>Max Congestion</th>
                            <td>
                                {max_congestion} agents
                                {most_congested_position && ` at (${most_congested_position.x}, ${most_congested_position.y})`}
                            </td>
                        </tr>
                        </tbody>
                    </table>
                </div>
                <div className="col-md-6">
                    {congestionData.length > 0 ? (
                        <Bar data={chartData} options={chartOptions}/>
                    ) : (
                        <p>Empty congestion data</p>
                    )}
                </div>
            </div>
        </div>
    )
}

export default Metrics