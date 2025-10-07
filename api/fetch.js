const { MongoClient } = require('mongodb');

export default async function handler(req, res) {
  // Check method
  if (req.method !== 'PUT') {
    return res.status(400).json({ error: 'Bad Request' });
  }

  // Check auth
  const secretKey = process.env.SECRET_KEY;
  if (!req.headers.auth || req.headers.auth !== secretKey) {
    return res.status(400).json({ error: 'Bad Request' });
  }

  try {
    // Fetch data from Foxwoods API
    const apiData = await queryFoxwoods();

    // Parse and transform the data
    const record = parseData(apiData);

    console.log('Record:', record);

    // Write to MongoDB
    await writeToMongo(record);

    return res.status(200).send('');
  } catch (error) {
    console.error('Error:', error);
    return res.status(500).json({ error: 'Internal Server Error' });
  }
}

async function queryFoxwoods() {
  const payload = {
    instance_id: "poker-tables-295-68e4eee4cd552",
    paragraph_id: "295"
  };

  const response = await fetch('https://foxwoods.com/api/poker-tables/load', {
    method: 'POST',
    headers: {
      'Host': 'foxwoods.com',
      'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:96.0) Gecko/20100101 Firefox/96.0',
      'Accept': '*/*',
      'Accept-Language': 'en-US,en;q=0.5',
      'Accept-Encoding': 'gzip, deflate, br, zstd',
      'X-Requested-With': 'XMLHttpRequest',
      'Referer': 'https://www.foxwoods.com/',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(payload)
  });

  const data = await response.json();
  return data;
}

function parseData(apiResponse) {
  // Get timestamp rounded to nearest 30 minutes
  const now = Math.floor(Date.now() / 1000);
  const ts = Math.floor(now / (60 * 30)) * (60 * 30);

  const games = {};
  let tableCount = 0;

  // Process each table
  for (const table of apiResponse.data) {
    const game = {
      tableCount: table.numberOfGames,
      blinds: table.limit
    };

    tableCount += game.tableCount;

    // Clean up game name (remove extra spaces)
    const key = table.name.replace(/   /g, ' ');

    if (games[key]) {
      games[key].push(game);
    } else {
      games[key] = [game];
    }
  }

  return {
    ts,
    tableCount,
    games
  };
}

async function writeToMongo(record) {
  const mongoUser = process.env.SCRAPE_USER;
  const mongoPass = process.env.SCRAPE_PASS;
  const mongoUrl = process.env.MONGO_URL;
  const mongoDb = process.env.MONGO_DB;
  const mongoCollection = process.env.MONGO_COLLECTION;

  const uri = `mongodb+srv://${mongoUser}:${mongoPass}@${mongoUrl}/${mongoDb}`;

  const client = new MongoClient(uri);

  try {
    await client.connect();
    const database = client.db('foxwoods');
    const collection = database.collection(mongoCollection);

    await collection.insertOne(record);
  } finally {
    await client.close();
  }
}
