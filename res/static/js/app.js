/**
 * A set of colors, used for randomizing visualizer colors.
 */
let COLORS = [
    '#F44336',
    '#536DFE',
    '#9C27B0',
    '#4CAF50',
    '#FF5722',
    '#FFA000',
    '#795548',
    '#607D8B',
    '#757575',
];

$('#toggle_record').on('click', function() {
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

$('#save_recording').on('click', function() {
    axios.get('/api/v1/template/save')
        .then(function(response) {
            console.log(response);
        })
        .catch(function(error) {
            console.log(error);
        })
});

$('#clear_visual').on('click', function() {
    // Render the visualizer with no model data
    renderVisualizer([]);

    // Hide the button
    $('#clear_visual').css('display', 'none');
});

// Fetch the current status from the server
$(document).ready(function() {
    // TODO: comment-out function that is not yet available
    // hideSaveButton();

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
    let button = $('#toggle_record');
    let save = $('#save_recording');

    if(recording) {
        button.text("Recording...");
        button.removeClass("btn-outline-success");
        button.addClass("btn-danger");

        save.css('display', 'inline-block');

        setLiveVisualize(true);
    } else {
        button.text("Start recording");
        button.removeClass("btn-danger");
        button.addClass("btn-outline-success");

        save.css('display', 'none');
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

$('#toggle_visualize').on('click', function() {
    let visualizing = $(this).hasClass("btn-danger");
    setLiveVisualize(!visualizing);
});

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

    // Enable the live visualizer by default now
    setLiveVisualize(true);
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
                renderVisualizer(models);

                // Resolve the promise, pass along the models data
                resolve(models);
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

    // Toggle the clear button
    let clear = $('#clear_visual');
    if(enabled)
        clear.css('display', 'none');
    else
        clear.css('display', 'inline-block');

    // Build a new visualization timer
    if(enabled)
        visualizerTimer = setInterval(fetchVisualizer, 50);

    // Set the button state
    let button = $('#toggle_visualize');
    if(enabled) {
        button.text("Visualizing...");
        button.removeClass("btn-outline-success");
        button.addClass("btn-danger");
    } else {
        button.text("Visualize");
        button.removeClass("btn-danger");
        button.addClass("btn-outline-success");
    }
}

/**
 * Render the given list of models on the visualizer canvas.
 */
function renderVisualizer(models) {
    // Ensure the visualizer is set
    if(visualizer === null)
        return;

    // Get the drawing context, and clear it first
    let context = visualizer.getContext("2d");
    context.clearRect(0, 0, visualizer.width, visualizer.height);

    // Render each model
    models.forEach(function(model, i) {
        _renderVisualizerTrace(context, model.trace.points, i);
    });
}

/**
 * Render a trace based on the given set of points on the visualizer.
 *
 * @param {object} context The canvas 2D drawing context.
 * @param {object[]} points The list of points in a trace to draw.
 * @param {int} i The index of this trace, used to determine what color to use.
 */
function _renderVisualizerTrace(context, points, i) {
    // Determine the color to use, and set it
    let color = COLORS[(i || 0) % COLORS.length];
    context.strokeStyle = color;
    context.fillStyle = color;

    // Cummulative coordinate and rotation values
    let last_x = 200;
    let last_y = 200;
    let last_rot = 0;

    // Map the rotational points into x/y axis we can render
    points = points.map((point) => {
        // Determine what coordinates to draw to
        let x = last_x - Math.cos(last_rot + point.angle) * point.distance;
        let y = last_y + Math.sin(last_rot + point.angle) * point.distance;

        // Update the last values
        last_x = x;
        last_y = y;
        last_rot += point.angle;

        return { x, y };
    });

    // Begin the path
    context.beginPath();

    // Plot each point with a curved line
    for(var i = 0; i < points.length - 1; i++) {
        var xc = (points[i].x + points[i + 1].x) / 2;
        var yc = (points[i].y + points[i + 1].y) / 2;
        context.quadraticCurveTo(points[i].x, points[i].y, xc, yc);
    }

    // Draw the path
    context.stroke();

    // Draw the sample points
    for(var i = 0; i < points.length; i++) {
        context.beginPath();
        context.arc(points[i].x, points[i].y, 1.5, 0, 2 * Math.PI);
        context.fill();
    }
}
