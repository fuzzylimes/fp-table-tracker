const Client = require('./client');

module.exports = async (req, res) => {
    const client = new Client();

    const latest = await getLatest(client).catch(err => {
        console.error(err);
        res.status = 500;
        res.send({Message: "Error querying latest record"})
    })
    res.send(latest);
}

async function getLatest(client) {
    try {
        await client.connect();

        const database = client.db('foxwoods');
        const poker = database.collection('poker');
        const options = { sort: { ts: -1 } }
        const latest = await poker.findOne({}, options);
        return latest;
    } finally {
        await client.close();
    }
}