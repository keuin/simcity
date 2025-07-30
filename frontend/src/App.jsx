import React, {useState, useEffect} from 'react'
import CityMap from './components/CityMap.jsx'
import StatisticsDisplay from './components/Metrics'
import ControlToolbar from './components/ControlToolbar.jsx'

function App() {
    const [city, setCity] = useState(null)
    const [agents, setAgents] = useState([])
    const [metrics, setMetrics] = useState(null)
    const [isRunning, setIsRunning] = useState(false)
    const [socket, setSocket] = useState(null)

    useEffect(() => {
        fetch('/api/city')
            .then(response => response.json())
            .then(data => setCity(data))
            .catch(error => console.error('Error fetching city data:', error))
    }, [])

    let runSimulation = () => {
        fetch('/api/start')
            .then(response => {
                console.log('Start simulation response status:', response.status)
                return response.json()
            })
            .then(data => {
                console.log('Start simulation response data:', data)
                if (data.status === 'started' || data.status === 'already_running') {
                    setIsRunning(true)
                    console.log('Simulation started successfully')
                } else {
                    console.warn('Unexpected response status:', data.status)
                }
            })
            .catch(error => console.error('Error starting simulation:', error))
    }

    let stopRunSimulation = () => {
        fetch('/api/stop')
            .then(response => {
                console.log('Stop simulation response status:', response.status)
                return response.json()
            })
            .then(data => {
                console.log('Stop simulation response data:', data)
                if (data.status === 'stopped') {
                    setIsRunning(false)
                    console.log('Simulation stopped successfully')
                } else {
                    console.warn('Unexpected response status:', data.status)
                }
                let s = socket;
                if (s) {
                    s.close()
                }
            })
            .catch(error => console.error('Error stopping simulation:', error))
    }

    useEffect(() => {
        if (!isRunning) return;
        console.log('Attempting to connect to WebSocket...')
        const wsUrl = `ws://${window.location.host}/ws`
        console.log('WebSocket URL:', wsUrl)

        const ws = (() => {
            try {
                return new WebSocket(wsUrl)
            } catch (error) {
                alert('failed to connect to WebSocket: ' + JSON.stringify(error))
            }
        })()

        ws.onopen = () => {
            console.log('WebSocket connection established successfully')
            runSimulation()
        }

        ws.onmessage = (event) => {
            console.log('Received WebSocket message:', event.data)
            try {
                const data = JSON.parse(event.data)
                setAgents(data.agents)
                setMetrics(data.metrics)
            } catch (error) {
                console.error('Error parsing WebSocket message:', error)
            }
        }

        ws.onerror = (error) => {
            console.error('WebSocket error:', error)
            alert('Websocket unexpectedly closed: ' + JSON.stringify(error))
        }

        ws.onclose = (event) => {
            console.log('WebSocket connection closed', event.code, event.reason)
            setSocket(null)
            setIsRunning(false)
            stopRunSimulation()
        }

        setSocket(ws)

        return () => {
            ws.close()
        }
    }, [isRunning])

    const startSimulation = () => {
        console.log('Starting simulation...')
        setIsRunning(true)
    }

    const stopSimulation = () => {
        console.log('Stopping simulation...')
        setIsRunning(false)
        if (!socket) return;
        socket.close()
    }

    return (<div className="container">
        <h1 className="my-4">City Traffic Simulator</h1>

        <ControlToolbar
            isRunning={isRunning}
            onStart={startSimulation}
            onStop={stopSimulation}
        />

        {city && (<>
            <h3>City Map</h3>
            <CityMap
                city={city}
                agents={agents}
                metrics={metrics}
            />
        </>)}

        {metrics && <>
            <h3>Statistics</h3>
            <StatisticsDisplay metrics={metrics}/>
        </>}
    </div>)
}

export default App