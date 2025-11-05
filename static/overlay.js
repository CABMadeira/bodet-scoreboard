// State management
let currentGameState = null;

// Get URL parameters
const urlParams = new URLSearchParams(window.location.search);
const homeTeamName = urlParams.get('home') || 'HOME';
const awayTeamName = urlParams.get('away') || 'AWAY';
const homeLogo = urlParams.get('homeLogo') || '';
const awayLogo = urlParams.get('awayLogo') || '';

// Update team names
document.getElementById('home-team-name').textContent = homeTeamName;
document.getElementById('away-team-name').textContent = awayTeamName;

// Update logos
function setLogo(team, logoUrl) {
    const img = document.getElementById(`${team}-logo`);
    const placeholder = document.getElementById(`${team}-placeholder`);
    if (logoUrl) {
        img.src = logoUrl;
        img.style.display = 'block';
        placeholder.style.display = 'none';
        img.onerror = () => {
            img.style.display = 'none';
            placeholder.style.display = 'block';
        };
    } else {
        img.style.display = 'none';
        placeholder.style.display = 'block';
    }
}
setLogo('home', homeLogo);
setLogo('away', awayLogo);

// Connect to SSE endpoint
const evtSource = new EventSource('/api/stream');

evtSource.onopen = () => {
    // Connection opened
};

evtSource.onerror = () => {
    // Connection error
};

evtSource.onmessage = (event) => {
    try {
        const data = JSON.parse(event.data);
        updateScoreboard(data);
    } catch (e) {
        console.error('Error parsing data:', e);
    }
};

function updateScoreboard(data) {
    // Update scores with animation only if changed
    updateElementIfChanged('home-score', data.home_score);
    updateElementIfChanged('away-score', data.away_score);
    
    // Update time with subtle animation
    updateTime(data.time);
    
    // Update period
    document.getElementById('period').textContent = data.period_name;
    
    // Update fouls
    updateFouls('home', data.home_fouls);
    updateFouls('away', data.away_fouls);
    
    // Update timeouts
    updateTimeouts('home', data.home_timeouts);
    updateTimeouts('away', data.away_timeouts);
    
    // Update shot clock
    updateShotClock(data.shot_clock);
    
    // Update game state (pause dot)
    updateGameState(data.game_state);
    
    // Store current state
    currentGameState = data;
}

function updateElementIfChanged(id, value) {
    const element = document.getElementById(id);
    if (!element) return;
    
    const currentValue = element.textContent;
    const newValue = value.toString();
    
    if (currentValue !== newValue) {
        element.textContent = newValue;
        element.classList.add('updated');
        setTimeout(() => element.classList.remove('updated'), 500);
    }
}

function updateElement(id, value) {
    const element = document.getElementById(id);
    if (element) {
        element.textContent = value;
    }
}

function updateFouls(team, foulsCount) {
    const container = document.getElementById(`${team}-fouls`);
    if (!container) return;
    
    const lines = container.querySelectorAll('.foul-line');
    const count = parseInt(foulsCount) || 0;
    
    lines.forEach((line, index) => {
        if (index < count) {
            line.classList.add('active');
        } else {
            line.classList.remove('active');
        }
    });
}

function updateTimeouts(team, timeoutsCount) {
    const container = document.getElementById(`${team}-timeouts`);
    if (!container) return;
    
    const lines = container.querySelectorAll('.timeout-line');
    const count = parseInt(timeoutsCount) || 0;
    
    lines.forEach((line, index) => {
        if (index < count) {
            line.classList.add('active');
        } else {
            line.classList.remove('active');
        }
    });
}

function updateTime(timeStr) {
    const timeElement = document.getElementById('time');
    if (!timeElement) return;
    
    if (timeElement.textContent !== timeStr) {
        timeElement.textContent = timeStr;
        timeElement.classList.add('updated');
        setTimeout(() => timeElement.classList.remove('updated'), 300);
    }
}

function updateShotClock(shotClock) {
    const element = document.getElementById('shot-clock');
    if (!element) return;
    
    const newValue = shotClock || '--';
    
    if (element.textContent !== newValue) {
        element.textContent = newValue;
        element.classList.add('updated');
        setTimeout(() => element.classList.remove('updated'), 400);
    }
}

function updateGameState(gameState) {
    const pauseDot = document.getElementById('pause-dot');
    if (!pauseDot) return;
    
    if (gameState === 'paused') {
        pauseDot.classList.add('visible');
    } else {
        pauseDot.classList.remove('visible');
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
