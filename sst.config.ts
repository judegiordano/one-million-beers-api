/// <reference path="./.sst/platform/config.d.ts" />

const domain = 'judethings.com'

export default $config({
  app(input) {
    return {
      name: "one-million-beers-api",
      removal: 'remove',
      home: 'aws',
      providers: {
        aws: { region: 'us-east-1' }
      },
      stage: input?.stage
    };
  },
  async run() {
    const { stage } = $app;
    const environment = {
      STAGE: stage,
      LOG_LEVEL: process.env.LOG_LEVEL!,
      MONGO_URI: process.env.MONGO_URI!,
      DISCORD_CLIENT_ID: process.env.DISCORD_CLIENT_ID!,
      DISCORD_CLIENT_SECRET: process.env.DISCORD_CLIENT_SECRET!,
      DISCORD_PUBLIC_KEY: process.env.DISCORD_PUBLIC_KEY!,
    }

    const lambda = new sst.aws.Function('OneMillionBeersApi', {
      runtime: 'rust',
      handler: 'bootstrap',
      bundle: 'target/lambda/api',
      memory: '1024 MB',
      timeout: '2 minutes',
      architecture: 'arm64',
      url: { cors: { allowCredentials: true } },
      logging: { retention: '1 week', format: 'json' },
      environment,
    });

    const router = new sst.aws.Router('OmbRouter', {
      routes: { '/*': lambda.url },
      domain: {
        name: `api.one-million-beers.${domain}`,
        dns: sst.aws.dns(),
        redirects: [`www.api.one-million-beers.${domain}`]
      }
    })

    return {
      function: lambda.url,
      url: router.url,
    }
  },
});
