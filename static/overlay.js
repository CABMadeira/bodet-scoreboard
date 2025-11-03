// State management
let currentGameState = null;

// Connect to SSE endpoint
const evtSource = new EventSource('/api/stream');

evtSource.onopen = () => {
    updateStatus('Connected', 'connected');
};

evtSource.onerror = () => {
    updateStatus('Connection error', 'error');
};

evtSource.onmessage = (event) => {
    try {
        const data = JSON.parse(event.data);
        updateScoreboard(data);
        updateStatus('Live', 'connected');
    } catch (e) {
        console.error('Error parsing data:', e);
        updateStatus('Parse error', 'error');
    }
};

function updateScoreboard(data) {
    // Update scores with animation only if changed
    updateElementIfChanged('home-score', data.home_score);
    updateElementIfChanged('away-score', data.away_score);
    
    // Update time with subtle animation
    updateTime(data.time_minutes, data.time_seconds);
    
    // Update period
    document.getElementById('period').textContent = data.period_name;
    
    // Update fouls
    updateElement('home-fouls', data.home_fouls);
    updateElement('away-fouls', data.away_fouls);
    
    // Update timeouts
    updateElement('home-timeouts', data.home_timeouts);
    updateElement('away-timeouts', data.away_timeouts);
    
    // Update possession
    updatePossession(data.possession);
    
    // Update game state
    updateGameState(data.game_state);
    
    // Store current state
    currentGameState = data;
}

function updateElementIfChanged(id, value) {
    const element = document.getElementById(id);
    const currentValue = element.textContent;
    const newValue = value.toString();
    
    if (currentValue !== newValue) {
        element.textContent = newValue;
        element.classList.add('updated');
        setTimeout(() => element.classList.remove('updated'), 500);
    }
}

function updateElement(id, value) {
    document.getElementById(id).textContent = value;
}

function updateTime(minutes, seconds) {
    const timeStr = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    const timeElement = document.getElementById('time');
    
    if (timeElement.textContent !== timeStr) {
        timeElement.textContent = timeStr;
        timeElement.classList.add('updated');
        setTimeout(() => timeElement.classList.remove('updated'), 300);
    }
}

function updatePossession(possession) {
    const homePossession = document.getElementById('home-possession');
    const awayPossession = document.getElementById('away-possession');
    
    homePossession.classList.remove('active');
    awayPossession.classList.remove('active');
    
    if (possession === 'Home') {
        homePossession.classList.add('active');
    } else if (possession === 'Away') {
        awayPossession.classList.add('active');
    }
}

function updateGameState(gameState) {
    const element = document.getElementById('game-state');
    element.textContent = gameState.toUpperCase().replace(/([A-Z])/g, ' $1').trim();
    
    // Remove all state classes
    element.classList.remove('pre-game', 'running', 'paused', 'halftime', 'overtime', 'final');
    
    // Add appropriate class
    const stateClass = gameState.toLowerCase().replace(/([A-Z])/g, '-$1').toLowerCase();
    element.classList.add(stateClass);
}

function updateStatus(message, className) {
    const status = document.getElementById('status');
    status.textContent = message;
    status.classList.remove('connected', 'error');
    if (className) {
        status.classList.add(className);
    }
}

// Initial fetch to get current state
fetch('/api/game')
    .then(response => response.json())
    .then(data => {
        if (data) {
            updateScoreboard(data);
        }
    })
    .catch(error => {
        console.error('Error fetching initial state:', error);
    });
