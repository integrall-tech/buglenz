"""Fills a LOCAL BugLenz instance with demo data (projects, 14 days of events, users, releases,
sessions, transactions and logs) so the screens can be reviewed populated. Throwaway databases only.

    BUGLENZ_URL=http://127.0.0.1:8099 BUGLENZ_EMAIL=admin@example.com BUGLENZ_PASSWORD=... \
        python3 scripts/seed-demo.py

The charts use the ingestion time, so the events all land "now". To spread them over the 14 days
(SQLite database of a local run only) rewrite the dates afterwards:

    sqlite3 db.sqlite "update events set ingested_at = timestamp; update logs set ingested_at = timestamp;
        update transactions set ingested_at = timestamp;
        update issues set first_seen=(select min(timestamp) from events e where e.issue_id=issues.id),
                          last_seen=(select max(timestamp) from events e where e.issue_id=issues.id)
        where exists (select 1 from events e where e.issue_id=issues.id);"
"""
import os
import json, random, time, urllib.request, uuid, http.cookiejar, sys

B = os.environ.get("BUGLENZ_URL", "http://127.0.0.1:8099")
if not B.startswith(("http://127.0.0.1", "http://localhost")):
    sys.exit("seed-demo: refusing to seed a non-local instance")
random.seed(7)
jar = http.cookiejar.CookieJar()
op = urllib.request.build_opener(urllib.request.HTTPCookieProcessor(jar))

def api(method, path, body=None):
    req = urllib.request.Request(B + path, method=method,
        data=json.dumps(body).encode() if body is not None else None,
        headers={"Content-Type": "application/json"})
    return json.load(op.open(req))

api("POST", "/auth/login", {"email": os.environ["BUGLENZ_EMAIL"], "password": os.environ["BUGLENZ_PASSWORD"]})
lst = api("GET", "/api/projects")
lst = lst["items"] if isinstance(lst, dict) else lst
have = {p["name"]: p for p in lst}
PROJECTS = [("vendax-web", "javascript-react"), ("vendax-api", "java-spring-boot"),
            ("vendax-mobile", "flutter"), ("backoffice", "javascript-react")]
proj = {}
for name, platform in PROJECTS:
    p = have.get(name) or api("POST", "/api/projects", {"name": name, "platform": platform})
    proj[name] = {"id": p["id"], "key": p["sentry_key"].replace("-", ""), "platform": platform}

def send(name, items, eid=None):
    p = proj[name]
    eid = eid or uuid.uuid4().hex
    body = json.dumps({"event_id": eid}) + "\n"
    for header, payload in items:
        raw = json.dumps(payload)
        header = dict(header, length=len(raw.encode()))
        body += json.dumps(header) + "\n" + raw + "\n"
    req = urllib.request.Request(f"{B}/api/{p['id']}/envelope/", data=body.encode(),
        headers={"Content-Type": "application/x-sentry-envelope",
                 "X-Sentry-Auth": f"Sentry sentry_version=7, sentry_key={p['key']}"})
    try:
        urllib.request.urlopen(req).read()
        return True
    except Exception as e:
        print("fail", name, e); return False

NOW = time.time()
DAY = 86400
USERS = [f"u-{n}" for n in range(1, 61)]

CATALOG = {
 "vendax-web": [
  ("TypeError", "Cannot read properties of undefined (reading 'itens')", "error", "pedido.ts", "calcularTotal", 3.0),
  ("TypeError", "Cannot read properties of null (reading 'cep')", "error", "endereco.ts", "validarCep", 2.0),
  ("ChunkLoadError", "Loading chunk 482 failed", "warning", "router.ts", "carregarRota", 1.5),
  ("NetworkError", "Falha ao buscar /api/pedidos", "warning", "http.ts", "buscar", 2.5),
  ("RangeError", "Maximum call stack size exceeded", "fatal", "arvore.ts", "percorrer", 0.4),
  ("ValidationError", "CPF informado é inválido", "info", "cliente.ts", "validar", 1.0),
  ("ReferenceError", "carrinho is not defined", "error", "checkout.ts", "finalizar", 1.2)],
 "vendax-api": [
  ("NullPointerException", "Cannot invoke \"Pedido.getItens()\" because \"pedido\" is null", "error", "PedidoService.java", "totalizar", 3.0),
  ("DataIntegrityViolationException", "duplicate key value violates unique constraint \"uk_pedido_numero\"", "error", "PedidoRepository.java", "salvar", 1.5),
  ("SocketTimeoutException", "Read timed out: gateway de pagamento", "warning", "PagamentoClient.java", "cobrar", 2.5),
  ("OutOfMemoryError", "Java heap space", "fatal", "RelatorioJob.java", "gerar", 0.3),
  ("IllegalStateException", "Estoque insuficiente para o SKU 8812", "info", "EstoqueService.java", "reservar", 1.5)],
 "vendax-mobile": [
  ("StateError", "Bad state: sessão expirada durante o checkout", "error", "checkout_page.dart", "_confirmar", 2.5),
  ("FormatException", "Unexpected character (at character 1)", "error", "api_client.dart", "_decode", 2.0),
  ("PlatformException", "camera_access_denied", "warning", "scanner.dart", "abrir", 1.5),
  ("RangeError", "Invalid value: Not in inclusive range 0..3: 4", "fatal", "carrossel.dart", "build", 0.5)],
 "backoffice": [
  ("TypeError", "Cannot read properties of undefined (reading 'map')", "error", "relatorio.tsx", "Tabela", 2.5),
  ("NetworkError", "Falha ao exportar planilha", "warning", "exportar.ts", "baixar", 1.5),
  ("PermissionError", "Perfil sem acesso ao módulo financeiro", "info", "guard.ts", "verificar", 1.0)],
}
RELEASES = {"vendax-web": ["vendax-web@1.4.0", "vendax-web@1.4.1", "vendax-web@1.4.2"],
            "vendax-api": ["vendax-api@2.0.0", "vendax-api@2.0.1", "vendax-api@2.1.0"],
            "vendax-mobile": ["vendax-mobile@3.2.0+41", "vendax-mobile@3.3.0+47"],
            "backoffice": ["backoffice@0.9.3", "backoffice@0.9.4"]}
VOLUME = {"vendax-web": 520, "vendax-api": 380, "vendax-mobile": 240, "backoffice": 110}

def release_at(name, age_days):
    rels = RELEASES[name]
    # newer releases take over as time goes on; the last one only appears in the last 4 days
    idx = len(rels) - 1 - min(len(rels) - 1, int(age_days // 5))
    return rels[max(0, idx if random.random() < 0.85 else idx - 1)]

def make_event(name, ts, age_days):
    t, v, lvl, fn, func, _ = random.choices(CATALOG[name], weights=[c[5] for c in CATALOG[name]])[0]
    eid = uuid.uuid4().hex
    java = name == "vendax-api"
    frames = [{"filename": fn, "function": func, "lineno": random.choice([12, 27, 44, 88]), "in_app": True}]
    ev = {"event_id": eid, "timestamp": ts, "platform": "java" if java else ("other" if name == "vendax-mobile" else "javascript"),
          "level": lvl, "release": release_at(name, age_days),
          "environment": random.choices(["production", "staging"], weights=[8, 2])[0],
          "exception": {"values": [{"type": t, "value": v, "mechanism": {"type": "generic", "handled": lvl != "fatal"},
                                    "stacktrace": {"frames": frames}}]},
          "tags": {"tenant": random.choice(["acme", "globex", "initech"]), "cliente": random.choice(["varejo", "atacado"])}}
    if random.random() < 0.75:
        ev["user"] = {"id": random.choice(USERS)}
    return ev

sent = 0
for name, vol in VOLUME.items():
    batch = []
    for i in range(vol):
        # more recent days heavier; one spike 3 days ago
        age = random.random() ** 1.6 * 14
        if random.random() < 0.12: age = 3 + random.random() * 0.4
        ts = NOW - age * DAY
        ev = make_event(name, ts, age)
        sent += send(name, [({"type": "event"}, ev)], eid=ev["event_id"])
    print(name, "events sent so far", sent)

# sessions: web and mobile (release health)
def session(name, age):
    sid = str(uuid.uuid4()); started = NOW - age * DAY
    status = random.choices(["exited", "crashed", "errored", "ok"], weights=[88, 2, 9, 1])[0]
    rels = RELEASES[name]
    return ({"type": "session"}, {"sid": sid, "did": random.choice(USERS), "init": True,
        "started": time.strftime("%Y-%m-%dT%H:%M:%S.000Z", time.gmtime(started)),
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S.000Z", time.gmtime(started + random.randint(20, 900))),
        "status": status, "errors": 0 if status in ("ok", "exited") else random.randint(1, 3),
        "duration": random.randint(20, 900), "attrs": {"release": release_at(name, age), "environment": "production"}})
for name, n in [("vendax-web", 900), ("vendax-mobile", 600), ("backoffice", 200)]:
    batch = []
    for _ in range(n):
        batch.append(session(name, random.random() ** 1.4 * 14))
        if len(batch) == 25: send(name, batch); batch = []
    if batch: send(name, batch)
    print(name, "sessions", n)

# transactions (performance)
ROUTES = {"vendax-web": ["/pedidos", "/checkout", "/catalogo", "/conta"],
          "vendax-api": ["GET /api/pedidos", "POST /api/pedidos", "GET /api/produtos/{id}", "POST /api/pagamentos"],
          "vendax-mobile": ["HomePage", "CheckoutPage", "ScannerPage"], "backoffice": ["/relatorios", "/financeiro"]}
def txn(name, age):
    end = NOW - age * DAY; dur = max(0.03, random.lognormvariate(-1.4, 0.7))
    if random.random() < 0.04: dur *= 6
    trace = uuid.uuid4().hex; root = uuid.uuid4().hex[:16]; cur = end - dur
    spans = []
    for op_, desc, frac in [("db.query", "SELECT * FROM pedidos WHERE cliente_id = ?", 0.45), ("http.client", "POST /pagamentos/cobrar", 0.3)]:
        sid = uuid.uuid4().hex[:16]
        spans.append({"span_id": sid, "parent_span_id": root, "trace_id": trace, "op": op_, "description": desc,
                      "start_timestamp": cur, "timestamp": cur + dur * frac, "status": "ok"}); cur += dur * frac
    return ({"type": "transaction"}, {"type": "transaction", "event_id": uuid.uuid4().hex,
        "transaction": random.choice(ROUTES[name]), "start_timestamp": end - dur, "timestamp": end,
        "release": release_at(name, age), "environment": "production", "platform": "javascript",
        "contexts": {"trace": {"trace_id": trace, "span_id": root, "op": "http.server", "status": random.choices(["ok", "internal_error"], weights=[96, 4])[0]}},
        "spans": spans})
for name, n in [("vendax-web", 300), ("vendax-api", 400), ("vendax-mobile", 150)]:
    batch = []
    for _ in range(n):
        batch.append(txn(name, random.random() ** 1.3 * 14))
        if len(batch) == 10: send(name, batch); batch = []
    if batch: send(name, batch)
    print(name, "transactions", n)

# logs
MSG = [("info", "Pedido {n} criado para o cliente {u}"), ("info", "Pagamento aprovado em {ms} ms"),
       ("warn", "Tentativa {k} de reenvio do webhook de estoque"), ("error", "Falha ao notificar o cliente {u}"),
       ("debug", "Cache miss para a chave catalogo:{n}")]
for name in ["vendax-api", "vendax-web"]:
    for _ in range(14):
        items = []
        for _ in range(random.randint(8, 20)):
            lvl, tpl = random.choice(MSG); age = random.random() * 14
            items.append({"timestamp": NOW - age * DAY, "trace_id": uuid.uuid4().hex, "level": lvl,
                "body": tpl.format(n=random.randint(1000, 9999), u=random.choice(USERS), ms=random.randint(80, 900), k=random.randint(1, 4)),
                "attributes": {"servico": {"value": name, "type": "string"}}})
        send(name, [({"type": "log", "item_count": len(items), "content_type": "application/vnd.sentry.items.log+json"}, {"items": items})])
    print(name, "logs done")
print("DONE", sent, "events")
