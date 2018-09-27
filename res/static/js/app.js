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
    } else {
        button.text("Start recording");
        button.removeClass("btn-danger");
        button.addClass("btn-outline-success");
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
