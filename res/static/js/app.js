$('#start_recording').on('click', function() {
    let recording = $(this).hasClass("btn-danger");

    axios.get('/api/v1/record/' + !recording)
        .then(function(response) {
            console.log(response);
            setRecordingState(response.data.recording);
        })
        .catch(function(error) {
            console.log(error);
        });
});

// Fetch the current status from the server
$(document).ready(function() {
    fetchRecordingState();

    fetchTemplates().then(function(templates) {
        // Put the template items into the list
        let list = $('.list-template');
        list.html("");
        templates.forEach(function(template) {
            $('<li/>').text(template).appendTo(list);
        });
    });
});

/**
 * Set the recording state.
 */
function setRecordingState(recording) {
    let button = $('#start_recording');

    if(recording) {
        button.text("Recording...");
        button.removeClass("btn-outline-success");
        button.addClass("btn-danger");
        startLive();
    } else {
        button.text("Start recording");
        button.removeClass("btn-danger");
        button.addClass("btn-outline-success");
        stopLive();
    }
}

/**
 * Fetch the recording state from the server.
 * Update the recording button state.
 */
function fetchRecordingState() {
    axios.get('/api/v1/record')
        .then(function(response) {
            setRecordingState(response.data.recording);
        })
        .catch(function(error) {
            console.log(error);
        });
}

/**
 * Fetch a list of templates from the server.
 */
function fetchTemplates() {
    return new Promise(function(resolve, reject) {
        axios.get('/api/v1/template').then(function(data) {
            resolve(data.data.templates);
        }).catch(reject);
    });
}






/**
 * Fetch live trace data, to render on the screen.
 */
function fetchLiveData() {
    return axios.get('/api/v1/get_live_trace');
}

var liveInterval = null;
var canvas = null;

/**
 * Start a live data interval timer.
 */
function startLive() {
    // Clear the current live interval first
    stopLive();

    // Build a new live interval
    liveInterval = setInterval(function() {
        fetchLiveData()
            .then(function(response) {
                let models = response.data.models;
                if(models.length > 0)
                    render(models[0].trace.points);
            })
            .catch(function(err) {
                console.error("Live data fetch error");
                console.error(err);
            });
    }, 50);
}

/**
 * Stop any current live data interval timer.
 */
function stopLive() {
    // Clear the live interval if set
    if(liveInterval != null)
        clearInterval(liveInterval);

    // Clear the handle
    liveInterval = null;
}

function render(rotations) {
    // Get the drawing context
    let context = canvas[0].getContext("2d");

    let last_x = 200;
    let last_y = 200;
    let last_rot = 0;

    // Make the data usable in JS
    points = rotations.map((rot) => {
        // hacked
        if(rot < 0.05)
            rot = 0;

        // Determine what coordinates to draw to
        let x = last_x + Math.cos(last_rot + rot) * 0.2;
        let y = last_y + Math.sin(last_rot + rot) * 0.2;
        // let x = last_x + Math.cos(last_rot + rot) * 0.2;
        // let y = last_y + Math.sin(last_rot + rot) * 0.2;

        // Update the last values
        last_x = x;
        last_y = y;
        last_rot += rot;

        return { x: x, y: y };
    });

    // Do not draw if too few points
    if(points.len < 3)
        return;

    console.log(points);

    for (var i = 1; i < points.length - 2; i ++) {
        var xc = (points[i].x + points[i + 1].x) / 2;
        var yc = (points[i].y + points[i + 1].y) / 2;
        // context.quadraticCurveTo(points[i].x, points[i].y, xc, yc);

        context.fillRect(points[i].x ,points[i].y, 1, 1);
    }

    // context.quadraticCurveTo(points[i].x, points[i].y, points[i+1].x, points[i+1].y);
}

$(document).ready(function() {
    canvas = $('#visual');
});
