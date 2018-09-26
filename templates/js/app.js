$('#start_recording').on('click', function() {
    axios.get('/api/v1/start_recording')
        .then(function (response) {
            console.log(response);
        })
        .catch(function (error) {
            console.log(error);
        });
});
