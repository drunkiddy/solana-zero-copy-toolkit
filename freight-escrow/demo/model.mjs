// Executable reference model, NOT a blockchain client or custody service.
const MAX = (1n << 64n) - 1n;
function amount(value, positive = false) {
  const n = BigInt(value);
  if (n < 0n || n > MAX || (positive && n === 0n)) throw Error('Invalid amount');
  return n;
}
export class EscrowModel {
  #orders = new Map();
  fund({ id, shipper, carrier, arbitrator, principal, deadline }, now) {
    if (![shipper, carrier, arbitrator].every(x => typeof x === 'string' && x.length)) throw Error('Missing role');
    if (new Set([shipper, carrier, arbitrator]).size !== 3) throw Error('Distinct roles required');
    if (!Number.isSafeInteger(deadline) || deadline <= now) throw Error('Invalid deadline');
    const key = JSON.stringify([shipper, String(id)]);
    if (this.#orders.has(key)) throw Error('Order already exists');
    principal = amount(principal, true);
    this.#orders.set(key, { shipper, carrier, arbitrator, principal, vault: principal, deadline, status: 'Funded', carrierPaid: 0n, shipperPaid: 0n });
    return key;
  }
  get(key) { return { ...this.#read(key) }; }
  #read(key) {
    const order = this.#orders.get(key);
    if (!order) throw Error('Unknown order');
    return order;
  }
  #check(order, actor, role, status) {
    if (actor !== order[role]) throw Error('Unauthorized');
    if (order.status !== status) throw Error('Invalid state');
  }
  accept(key, actor, now) {
    const o = this.#read(key);
    this.#check(o, actor, 'carrier', 'Funded');
    if (now >= o.deadline) throw Error('Acceptance expired');
    o.status = 'Accepted';
  }
  dispute(key, actor) {
    const o = this.#read(key);
    if (actor !== o.shipper && actor !== o.carrier) throw Error('Unauthorized');
    if (o.status !== 'Accepted') throw Error('Invalid state');
    o.status = 'Disputed';
  }
  donate(key, units) {
    const o = this.#read(key);
    units = amount(units);
    if (o.vault + units > MAX) throw Error('Overflow');
    o.vault += units;
  }
  #settle(o, carrierPaid, status) {
    if (o.vault < o.principal) throw Error('Underfunded');
    o.carrierPaid = carrierPaid;
    o.shipperPaid = o.vault - carrierPaid;
    o.vault = 0n;
    o.status = status;
  }
  release(key, actor) {
    const o = this.#read(key);
    this.#check(o, actor, 'shipper', 'Accepted');
    this.#settle(o, o.principal, 'Settled');
  }
  refund(key, actor, now) {
    const o = this.#read(key);
    this.#check(o, actor, 'shipper', 'Funded');
    if (now < o.deadline) throw Error('Refund not yet available');
    this.#settle(o, 0n, 'Refunded');
  }
  resolve(key, actor, carrierUnits) {
    const o = this.#read(key);
    this.#check(o, actor, 'arbitrator', 'Disputed');
    carrierUnits = amount(carrierUnits);
    if (carrierUnits > o.principal) throw Error('Award exceeds principal');
    this.#settle(o, carrierUnits, 'Settled');
  }
}
