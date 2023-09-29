import mqtt from 'mqtt';
import 'reflect-metadata';
import { DataSource } from 'typeorm';
import { NumberEntity } from './NumberEntity';

const AppDataSource = new DataSource({
    type: 'postgres',
    host: process.env.DB_HOST ?? 'localhost',
    port: Number(process.env.DB_PORT) || 5432,
    username: process.env.DB_USER ?? 'postgres',
    password: process.env.DB_PASSWORD ?? 'password',
    database: process.env.DB_NAME ?? 'mydatabase',
    entities: [NumberEntity],
    synchronize: true,
});

const start = async () => {
    try {
        const connection = await AppDataSource.initialize()
        console.log('Connected to the database.');

        const mqttClient = mqtt.connect('mqtt://mosquitto:1883', {
            clientId: 'ts-subscriber'
        });

        mqttClient.on('connect', () => {
            mqttClient.subscribe('data/random-numbers', (err) => {
                if (err) throw err;
            });
        });

        mqttClient.on('message', (topic, message) => {
            const payload = parseInt(message.toString());
            console.log(`Received message on ${topic}: ${payload}`);

            (async () => {
                try {
                    await connection.query('INSERT INTO numbers (value) VALUES ($1)', [payload]);
                } catch (e) {
                    console.error('Error inserting into database', e);
                }
            })();
        });

        mqttClient.on('error', (error) => {
            console.error('MQTT Error:', error);
        });
    } catch (error) {
        console.error('Error connecting to the database', error);
        process.exit(1);
    }
};

start();
