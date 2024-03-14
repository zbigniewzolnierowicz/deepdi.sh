import { getWebAutoInstrumentations } from "@opentelemetry/auto-instrumentations-web";
import { ZoneContextManager } from "@opentelemetry/context-zone";
import { registerInstrumentations } from "@opentelemetry/instrumentation";
import { Resource } from "@opentelemetry/resources";
import {
  ConsoleSpanExporter,
  SimpleSpanProcessor,
  TracerConfig,
  WebTracerProvider,
} from "@opentelemetry/sdk-trace-web";
import { SEMRESATTRS_SERVICE_NAME } from "@opentelemetry/semantic-conventions";

const providerConfig: TracerConfig = {
  resource: new Resource({
    [SEMRESATTRS_SERVICE_NAME]: "recipes-frontend",
  }),
};

export const instrument = () => {
  const provider = new WebTracerProvider(providerConfig);

  if (import.meta.env.PROD) {
    provider.addSpanProcessor(new SimpleSpanProcessor(new ConsoleSpanExporter()));
  }

  provider.register({
    contextManager: new ZoneContextManager(),
  });

  registerInstrumentations({
    instrumentations: [getWebAutoInstrumentations()],
  });
};
