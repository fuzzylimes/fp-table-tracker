const Client = require('./client');

module.exports = async (req, res) => {
    // handle our query params
    const {start, stop} = req.query;
    if (!start || !stop) {
        res.status(400);
        res.send({Message: "missing query parameters"});
    }

    // build out our client query db
    const client = new Client();

    const range = await getRange(client, start, stop).catch(err => {
        console.error(err);
        res.status = 500;
        res.send({ Message: "Error querying latest record" })
    })
    res.send(range);
}

async function getRange(client, start, stop) {
    try {
        await client.connect();

        const database = client.db(process.env.MONGO_DB);
        const poker = database.collection('poker');

        // Query for time stamps between start and stop time, ascending order
        const query = { ts: { $gte: parseInt(start), $lt: parseInt(stop) } };
        const options = { sort: { ts: 1 } };
        const cursor = poker.find(query, options);
        let records = await cursor.toArray();
        console.log(`found ${records.length} records`);
        return records;
    } finally {
        await client.close();
    }
}