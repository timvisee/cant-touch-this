/**
 * A set of colors, used for randomizing visualizer colors.
 */
const COLORS = [
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

/**
 * The normal gesture controller state.
 */
const STATE_NORMAL = 0;

/**
 * The gesture controlle recording state.
 */
const STATE_RECORDING = 1;

/**
 * The gesture controlle saving state.
 */
const STATE_SAVING = 2;

/**
 * Notify library settings.
 */
const NOTIFY_SETTINGS = {
    type: 'info',
    allow_dismiss: true,
    placement: {
        from: 'top',
        align: 'center'
    },
    timer: 1000,
    delay: 1000,
};

/**
 * The polling rate/interval of the visualizer in milliseconds.
 */
const VISUALIZER_INTERVAL = 50;

/**
 * The current gesture controller state.
 */
var state = STATE_NORMAL;

/**
 * The trim values.
 */
var trim = [0, 0];

$('#toggle_record').on('click', function() {
    let recording = $(this).hasClass("btn-danger");
    let new_state = recording ? (models.length > 0 ? STATE_SAVING : STATE_NORMAL) : STATE_RECORDING;

    // Send the new state
    sendState(new_state);
});

$('#clear_visual').on('click', function() {
    // Render the visualizer with no model data
    models = [];
    renderVisualizer(models);

    // Hide the button
    setShowClearVisualize(false);
});

$('#save_recording').on('click', function() {
    // Get the name
    let name = $('#name').val();

    // Validate the name and trim data
    if(name.length <= 0) {
        alert("Please provide a template name");
        return;
    }
    if(trim === undefined || trim[0] < 0 || trim[0] > trim[1]) {
        alert("Incorrect trim data");
        console.error(trim);
        return;
    }
    if(trim[1] - trim[0] < 5) {
        alert("The trace must be at least 5 frames long");
        return;
    }

    // Send the create request
    axios.get('/api/v1/template/create/' + encodeURIComponent(name) + '/' + trim[0] + '/' + trim[1])
        .then(function(response) {
            updateTemplateList();
            sendState(STATE_NORMAL);
        })
        .catch(function(error) {
            alert("An error occurred while saving your template");
            console.log(error);
        });
});

$('#discard_recording').on('click', function() {
    // Render the visualizer with no model data
    models = [];
    renderVisualizer(models);

    // Hide the trim panel
    setShowTrimPanel(false);

    // Send the new state to the server
    sendState(STATE_NORMAL);
});

$('#add_builtin_templates').on('click', function() {
    // Update the state on the server
    axios.get('/api/v1/template/add_builtin')
        .then(function() {
            updateTemplateList();
        })
        .catch(function(error) {
            alert('Failed to add built-in templates');
            console.log(error);
        });
});

// Fetch the current status from the server
$(document).ready(function() {
    fetchState();

    // Updat the list of templates
    updateTemplateList();

    initVisualizer();

    // Build the trim slider
    buildTrimSlider();
});

/**
 * Send a new state to the server.
 *
 * @param {int} new_state The new state to send.
 */
function sendState(new_state) {
    // Update the state on the server
    axios.get('/api/v1/state/' + new_state)
        .then(function(response) {
            state = response.data.state;
            setState(state);
        })
        .catch(function(error) {
            console.log(error);
        });
}

/**
 * Set the state.
 *
 * @param {int} state The state.
 */
function setState(state) {
    let button = $('#toggle_record');
    let recording = state == STATE_RECORDING;
    let saving = state == STATE_SAVING;

    if(recording) {
        button.text("Recording...");
        button.removeClass("btn-outline-success");
        button.addClass("btn-danger");

        setLiveVisualize(true);
    } else {
        button.text("Start recording");
        button.removeClass("btn-danger");
        button.addClass("btn-outline-success");
    }

    setShowTrimPanel(saving);
    setShowVisualize(!recording && !saving);
}

/**
 * Build the trim slider.
 */
function buildTrimSlider() {
    // Configure the trim range slider
    $("#trim-slider").slider({
        range: true,
        min: 0,
        max: 100,
        values: [0, 100],
        slide: function(event, ui) {
            // Update the trim values
            trim = ui.values;

            // Update the range label
            $("#trim").val(ui.values[0] + " - " + ui.values[1] + " frames");

            // Rerender the trimmed visuals
            if(models !== undefined)
                renderVisualizer(models);
        }
    });

    // Set the initial trim value
    $("#trim").val(
        $("#trim-slider").slider("values", 0) +
        " - " +
        $("#trim-slider").slider("values", 100) +
        " frames"
    );
}

/**
 * Fetch the recording state from the server.
 * Update the recording button state.
 */
function fetchState() {
    axios.get('/api/v1/state')
        .then(function(response) {
            // Update the state
            state = response.data.state;
            setState(state);
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

            // Show a message if there are no templates
            if(templates.length === 0)
                list.html("<i>No templates configured, create one or add built-in templates using the button below!</i>");
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
function setShowVisualize(show) {
    setShowButton($('#toggle_visualize'), show);
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
 * Set whether to show the trim panel.
 *
 * @param {boolean} show True to show, false to hide.
 */
function setShowTrimPanel(show) {
    $('.trim-panel').css('display', show ? 'inline' : 'none');

    // If the slider is shown, update it seeded by the last data
    if(show) {
        // Count the frames the recording has
        trim = [0, models[0].trace.points.length];

        // Update the slider bounds and default value
        let slider = $("#trim-slider");
        slider.slider("option", "max", trim[1]);
        slider.slider("option", "values", trim);
    }
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
 * The last model data we received.
 */
var models = [];

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
}

/**
 * Fetch data to visualize from the server.
 *
 * The fetched data will be rendered automatically 
 */
function fetchVisualizer() {
    return new Promise(function(resolve, reject) {
        axios.get('/api/v1/visualizer')
            .then(function(response) {
                // Get the models and detected gestures
                models = response.data.models;
                let detected = response.data.detected;

                // Render a notification showing the detected gesture
                if(detected !== undefined)
                    detected.forEach((gesture) =>
                        $.notify({
                            message: 'Detected: ' + gesture.name,
                        }, NOTIFY_SETTINGS)
                    );

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
        visualizerTimer = setInterval(fetchVisualizer, VISUALIZER_INTERVAL);

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
        // Trim the points in the save state
        let points = model.trace.points;
        if(state === STATE_SAVING)
            points = points.slice(trim[0], trim[1]);

        // Render the trace
        _renderVisualizerTrace(context, points, i);
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
