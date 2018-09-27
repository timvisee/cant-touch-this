$('#start_recording').on('click', function() {
    axios.get('/api/v1/record/true')
        .then(function (response) {
            console.log(response);
        })
        .catch(function (error) {
            console.log(error);
        });
});
