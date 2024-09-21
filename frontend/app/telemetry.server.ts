import opentelemetry from '@opentelemetry/sdk-node';
// import { getNodeAutoInstrumentations } from '@opentelemetry/auto-instrumentations-node';
import { Resource } from '@opentelemetry/resources';
import { ATTR_SERVICE_NAME } from '@opentelemetry/semantic-conventions';
import { RemixInstrumentation } from 'opentelemetry-instrumentation-remix';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';

// configure the SDK to export telemetry data to the console
// enable all auto-instrumentations from the meta package
const traceExporter = new OTLPTraceExporter({
  url: 'http://localhost:4318/v1/traces',
});

const sdk = new opentelemetry.NodeSDK({
  resource: new Resource({
    [ATTR_SERVICE_NAME]: 'deepdi.sh-frontend-server',
  }),
  traceExporter,
  instrumentations: [
    /* getNodeAutoInstrumentations({
      '@opentelemetry/instrumentation-fs': {
        enabled: false,
      },
    }), */
    new RemixInstrumentation(),
  ],
});

sdk.start();
