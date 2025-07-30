import React from 'react'

const CityMap = ({city, agents = [], metrics}) => {
    if (!city) return null

    agents = agents || [];

    const {width, height, cells} = city
    const congestionMap = metrics?.congestion_map || {}

    const getCellClass = (cellType) => {
        switch (cellType) {
            case 'Road':
                return 'road'
            case 'House':
                return 'house'
            case 'Office':
                return 'office'
            case 'Park':
                return 'park'
            case 'EnergyStation':
                return 'energy-station'
            case 'Empty':
                return 'empty'
            default:
                return ''
        }
    }

    const getCellDisplayChar = (cellType) => {
        return {
            'Road': '🛣️',
            'House': '🏠',
            'Office': '💼',
            'Park': '🎡',
            'EnergyStation': '⛽',
            'Empty': '',
        }[cellType]
    }

    const getCongestionClass = (x, y) => {
        const key = `${x},${y}`
        const congestion = congestionMap[key] || 0

        if (congestion === 0) return ''
        if (congestion === 1) return 'congestion-low'
        if (congestion === 2) return 'congestion-medium'
        return 'congestion-high'
    }

    return (
        <div className="city-container">
            <table className="city-grid" style={{borderCollapse: 'collapse'}}>
                <tbody>
                {cells.map((row, y) => (
                    <tr key={y}>
                        {row.map((cell, x) => {
                            const cellAgents = agents.filter(
                                agent => agent.position.x === x && agent.position.y === y
                            );

                            return (
                                <td
                                    key={`${x}-${y}`}
                                    className={`cell ${getCellClass(cell.cell_type)} ${getCongestionClass(x, y)}`}
                                    title={`(${x}, ${y}) - ${cell.cell_type}`}
                                    style={{
                                        width: 30,
                                        height: 30,
                                        border: '1px solid #ccc',
                                        textAlign: 'center',
                                        verticalAlign: 'middle',
                                        position: 'relative',
                                    }}
                                >
                                    <span>{getCellDisplayChar(cell.cell_type)}</span>

                                    {cellAgents.map(agent => (
                                        <div
                                            key={agent.id}
                                            className="agent"
                                            title={`Agent ${agent.id} - ${agent.state}`}
                                            style={{
                                                position: 'absolute',
                                                top: 4,
                                                left: 4,
                                                width: 10,
                                                height: 10,
                                                borderRadius: '50%',
                                                backgroundColor: getAgentColor(agent.state),
                                            }}
                                        />
                                    ))}
                                </td>
                            );
                        })}
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    )
}

function getAgentColor(state) {
    switch (state) {
        case 'AtHome':
            return 'blue'
        case 'GoingToWork':
            return 'red'
        case 'AtWork':
            return 'green'
        case 'GoingHome':
            return 'orange'
        case 'GoingToPark':
            return 'yellow'
        case 'AtPark':
            return 'purple'
        default:
            return 'black'
    }
}

export default CityMap