import {
  BatchSpanProcessor,
} from '@opentelemetry/sdk-trace-base';
import { WebTracerProvider } from '@opentelemetry/sdk-trace-web';
import { ZoneContextManager } from '@opentelemetry/context-zone';
import { registerInstrumentations } from '@opentelemetry/instrumentation';
import { B3Propagator } from '@opentelemetry/propagator-b3';
import { getWebAutoInstrumentations } from '@opentelemetry/auto-instrumentations-web';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';
import { diag, DiagConsoleLogger, DiagLogLevel } from '@opentelemetry/api';
import { ATTR_SERVICE_NAME } from '@opentelemetry/semantic-conventions';
import { Resource } from '@opentelemetry/resources';

diag.setLogger(new DiagConsoleLogger(), DiagLogLevel.DEBUG);

const provider = new WebTracerProvider({
  resource: new Resource({
    [ATTR_SERVICE_NAME]: 'deepdi.sh-frontend-web',
  }),
});

const traceExporter = new OTLPTraceExporter({
  url: 'http://localhost:4318/v1/traces',
  headers: {
    'Content-Type': 'application/json',
  },
});

provider.addSpanProcessor(new BatchSpanProcessor(traceExporter));

provider.register({
  // Changing default contextManager to use ZoneContextManager - supports asynchronous operations - optional
  contextManager: new ZoneContextManager(),
  propagator: new B3Propagator(),
});

// Registering instrumentations
registerInstrumentations({
  instrumentations: [
    getWebAutoInstrumentations(),
  ],
});
