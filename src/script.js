document.getElementById('personForm').addEventListener('submit', function(e) {
    e.preventDefault();

    const formData = {
        id: document.getElementById('id').value,
        firstName: document.getElementById('firstName').value,
        lastName: document.getElementById('lastName').value,
        // Samla in andra fält här
    };    
});

function fetchPersons() {
    fetch('http://localhost:3030/get_persons')
    .then(response => response.json())
    .then(data => {
        console.log('Persons:', data);
        // Uppdatera din frontend med persondata här
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}

function createPerson(person) {
    fetch('http://localhost:3030/add_person', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData),
    })
    .then(response => response.json())
    .then(data => console.log('Success:', data))
    .catch((error) => {
        console.error('Error:', error);
    });
}