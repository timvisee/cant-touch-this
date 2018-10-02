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

    initVisualizer();
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
        setLiveVisualize(true);
    } else {
        button.text("Start recording");
        button.removeClass("btn-danger");
        button.addClass("btn-outline-success");
        setLiveVisualize(false);
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
 * The visualizer element if available, should be a canvas.
 */
var visualizer = null;

/**
 * The timer that is used for requesting visualizer data.
 */
var visualizerTimer = null;

/**
 * Initialize the visualizer.
 *
 * This automatically requests visualization data from the server once.
 */
function initVisualizer() {
    // Get all jQuery elements, ensure there is at least one
    let elements = $('#visualizer');
    if(elements.length <= 0)
        return;

    // Get the visualizer element
    visualizer = elements.get(0);

    // Fetch initial data for the visualizer
    fetchVisualizer();
}

/**
 * Fetch data to visualize from the server.
 *
 * The fetched data will be rendered automatically 
 */
function fetchVisualizer() {
    return new Promise(function(resolve, reject) {
        axios.get('/api/v1/visualizer/points')
            .then(function(response) {
                // Skip if there are no models to render
                let models = response.data.models;
                if(models.length <= 0)
                    return;

                // Get the points, visualize and resolve
                let points = models[0].trace.points;
                renderVisualizer(points);

                // Resolve the promise, pass along the points data
                resolve(points);
            })
            .catch(function(err) {
                // Stop the live visualizer due to errors
                console.error(err);
                setLiveVisualize(false);

                // Reject the returned promise
                reject(err);
            });
    });
}

/**
 * Set whether live visualisation is enabled or not.
 */
function setLiveVisualize(enabled) {
    // Disable active timer
    if(visualizerTimer != null)
        clearInterval(visualizerTimer);
    visualizerTimer = null;

    // Build a new visualization timer
    if(enabled)
        visualizerTimer = setInterval(fetchVisualizer, 50);
}

/**
 * Render the given set of points using the visualizer.
 */
function renderVisualizer(points) {
    // Ensure the visualizer is set
    if(visualizer === null)
        return;

    // Get the drawing context, and clear it first
    let context = visualizer.getContext("2d");
    context.clearRect(0, 0, visualizer.width, visualizer.height);

    // Cummulative coordinate and rotation values
    let last_x = 200;
    let last_y = 200;
    let last_rot = 0;

    // Map the rotational points into x/y axis we can render
    points = points.map((point) => {
        // // TODO: temporary filter
        // if(rot < 0.05)
        //     rot = 0;

        // Determine what coordinates to draw to
        let x = last_x + Math.cos(last_rot + point.angle) * point.distance;
        let y = last_y + Math.sin(last_rot + point.angle) * point.distance;

        // Update the last values
        last_x = x;
        last_y = y;
        last_rot += point.angle;

        return { x, y };
    });

    for(var i = 0; i < points.length; i ++)
        context.fillRect(points[i].x ,points[i].y, 1, 1);

    // Do not draw if too few points
    if(points.len < 3)
        return;

    for(var i = 1; i < points.length - 2; i ++) {
        var xc = (points[i].x + points[i + 1].x) / 2;
        var yc = (points[i].y + points[i + 1].y) / 2;
        // context.quadraticCurveTo(points[i].x, points[i].y, xc, yc);
    }

    // context.quadraticCurveTo(points[i].x, points[i].y, points[i+1].x, points[i+1].y);
}
