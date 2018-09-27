$('#start_recording').on('click', function() {
    let recording = $(this).hasClass("btn-danger");

    axios.get('/api/v1/record/' + !recording)
        .then(function (response) {
            console.log(response);
            setRecordingState(response.data.started);
        })
        .catch(function (error) {
            console.log(error);
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