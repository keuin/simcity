import React from 'react'

const ControlToolbar = ({isRunning, onStart, onStop}) => {
    return (
        <div className="controls card mb-4">
            <div className="card-body">
                <div className="d-flex gap-2">
                    <button
                        className="btn btn-success"
                        onClick={onStart}
                        disabled={isRunning}
                    >
                        Start
                    </button>
                    <button
                        className="btn btn-danger"
                        onClick={onStop}
                        disabled={!isRunning}
                    >
                        Stop
                    </button>
                </div>
                <div className="mt-3">
                    <div className="alert alert-info">
                        <strong>Status:</strong> {isRunning ? 'Running' : 'Stopped'}
                    </div>
                </div>
            </div>
        </div>
    )
}

export default ControlToolbar