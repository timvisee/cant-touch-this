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
    setShowClearVisualize(false);
});

// Fetch the current status from the server
$(document).ready(function() {
    // TODO: comment-out function that is not yet available
    // hideSaveButton();

    fetchRecordingState();

    // Updat the list of templates
    updateTemplateList();

    initVisualizer();
});

/**
 * Set the recording state.
 */
function setRecordingState(recording) {
    let button = $('#toggle_record');

    if(recording) {
        button.text("Cancel recording");
        button.removeClass("btn-outline-success");
        button.addClass("btn-danger");

        setLiveVisualize(true);
    } else {
        button.text("Start recording");
        button.removeClass("btn-danger");
        button.addClass("btn-outline-success");
    }

    setShowSaveRecording(recording);
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
        axios.get('/api/v1/template')
            .then(function(data) {
                resolve(data.data.templates);
            })
            .catch(reject);
    });
}

/**
 * Update the template list, by fetching an up to date list of templates, then
 * update the DOM.
 */
function updateTemplateList() {
    // TODO: catch and handle errors
    fetchTemplates()
        .then(function(templates) {
            // Put the template items into the list
            let list = $('.list-template');
            list.html("");
            templates.forEach(function(template) {
                // Get the template id and name
                let id = template.id;
                let name = template.name;
                $('<li class="list-group-item" />')
                    .text(name)
                    .append(
                        $('<button type="button" class="btn btn-sm btn-outline-danger" data-id="' + id + '" />')
                            .text("X")
                            .click(deleteTemplateCallback)
                    )
                    .append($('<span class="id"></span>').text('id: ' + id))
                    .appendTo(list);
            });
        });
}

/**
 * Delete the template with the given ID.
 *
 * This will send a request to the server to delete the template.
 * After the deletion, the show list of templates is automatically updated.
 *
 * @param {int} id The ID of the template to delete.
 * @return {Promise} A promise for the request.
 */
function deleteTemplate(id) {
    return new Promise(function(resolve, reject) {
        axios.get('/api/v1/template/' + id + '/delete')
            .then(updateTemplateList)
            .catch(function(err) {
                console.error(err);
                alert("An error occurred while requesting template deletion: " + err);
            });
    });
}

/**
 * A callback to invoke when a template delete button is pressed.
 * The button that was clicked to delete the template should be passed as
 * `this`, for example by directly passing this as callback to the `.click()`
 * event.
 */
function deleteTemplateCallback() {
    // Get the button that was pressed, and mark it as disabled
    let button = $(this);
    button.attr('disabled', true);

    // Delete the template
    deleteTemplate(button.data('id'));
}

/**
 * Set whether to show the given button.
 *
 * @param {object} button A jQuery button.
 * @param {boolean} show True to show, false to hide.
 */
function setShowButton(button, show) {
    button.css('display', show ? 'inline-block' : 'none');
}

/**
 * Set whehter to show the `Clear Visualize` button.
 *
 * @param {boolean} show True to show, false to hide.
 */
function setShowClearVisualize(show) {
    setShowButton($('#clear_visual'), show);
}

/**
 * Set whehter to show the `Save Recording` button.
 *
 * @param {boolean} show True to show, false to hide.
 */
function setShowSaveRecording(show) {
    setShowButton($('#save_recording'), show);
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
                // Get the models
                let models = response.data.models;

                // Visualize and resolve
                renderVisualizer(models);
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

    // Show the visualize button when live visualisation stops
    setShowClearVisualize(!enabled);

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

    // Draw the origin dot
    context.fillStyle = "black";
    context.beginPath();
    context.arc(visualizer.width / 2, visualizer.height / 2, 1.5, 0, 2 * Math.PI);
    context.fill();

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
    let last_x = 400;
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
