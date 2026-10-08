import * as Sentry from '@sentry/react';
import { useState } from 'react';
import { calcularTotalPedido, confirmarPedido } from './pedido';

// Renders fine until `quebrar` is true, then throws inside render: the
// ErrorBoundary case of the GAP §3 script.
function Resumo({ quebrar }: { quebrar: boolean }) {
  if (quebrar) {
    throw new RangeError('resumo do pedido fora do intervalo');
  }
  return <p id="resumo">resumo ok</p>;
}

export default function App() {
  const [quebrar, setQuebrar] = useState(false);
  const [cliques, setCliques] = useState(0);

  return (
    <main>
      <h1>BugLenz e2e</h1>
      <button
        id="btn-click"
        type="button"
        onClick={() => {
          setCliques((n) => n + 1);
          // Unhandled exception in an event handler: the SDK's global handler
          // reports it and marks the session `unhandled`.
          calcularTotalPedido([]);
        }}
      >
        erro no clique ({cliques})
      </button>
      <button
        id="btn-promise"
        type="button"
        onClick={() => {
          // Unhandled promise rejection, nothing awaits it on purpose.
          void confirmarPedido('42');
        }}
      >
        promise rejeitada
      </button>
      <button id="btn-render" type="button" onClick={() => setQuebrar(true)}>
        erro de render
      </button>
      <Sentry.ErrorBoundary fallback={<p id="fallback">falhou</p>}>
        <Resumo quebrar={quebrar} />
      </Sentry.ErrorBoundary>
    </main>
  );
}
