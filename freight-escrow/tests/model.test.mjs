import test from 'node:test';
import assert from 'node:assert/strict';
import { EscrowModel } from '../demo/model.mjs';
function fixture() {
  const model = new EscrowModel();
  const args = { id: 'load-1', shipper: 'shipper', carrier: 'carrier', arbitrator: 'arbiter', principal: 2500000000n, deadline: 200 };
  const key = model.fund(args, 100);
  return { model, key, args };
}
test('delivery approval pays principal and cannot be replayed', () => {
  const { model: m, key: k } = fixture();
  m.accept(k, 'carrier', 150); m.release(k, 'shipper');
  assert.equal(m.get(k).carrierPaid, 2500000000n);
  assert.equal(m.get(k).vault, 0n);
  assert.throws(() => m.release(k, 'shipper'));
});
test('wrong signers cannot accept, release, dispute or arbitrate', () => {
  const { model: m, key: k } = fixture();
  assert.throws(() => m.accept(k, 'attacker', 150));
  m.accept(k, 'carrier', 150);
  assert.throws(() => m.release(k, 'attacker'));
  assert.throws(() => m.dispute(k, 'attacker'));
  m.dispute(k, 'carrier');
  assert.throws(() => m.resolve(k, 'shipper', 1n));
  assert.equal(m.get(k).vault, 2500000000n);
});
test('deadline boundary separates acceptance and refund', () => {
  const { model: m, key: k } = fixture();
  assert.throws(() => m.refund(k, 'shipper', 199));
  assert.throws(() => m.accept(k, 'carrier', 200));
  m.refund(k, 'shipper', 200);
  assert.equal(m.get(k).shipperPaid, 2500000000n);
});
test('accepted orders cannot be refunded even after deadline', () => {
  const { model: m, key: k } = fixture();
  m.accept(k, 'carrier', 150);
  assert.throws(() => m.refund(k, 'shipper', 1000));
});
test('dispute blocks release; allocation conserves value including donations', () => {
  const { model: m, key: k } = fixture();
  m.accept(k, 'carrier', 150); m.donate(k, 30n); m.dispute(k, 'shipper');
  assert.throws(() => m.release(k, 'shipper'));
  assert.throws(() => m.resolve(k, 'arbiter', 2500000001n));
  m.resolve(k, 'arbiter', 2000000000n);
  const o = m.get(k);
  assert.equal(o.carrierPaid + o.shipperPaid, 2500000030n);
  assert.equal(o.vault, 0n);
  assert.throws(() => m.resolve(k, 'arbiter', 0n));
});
test('settlement conservation for all sampled allocations', () => {
  for (let allocation = 0n; allocation <= 2500000000n; allocation += 100000000n) {
    const { model: m, key: k } = fixture();
    m.accept(k, 'carrier', 150); m.dispute(k, 'carrier'); m.resolve(k, 'arbiter', allocation);
    assert.equal(m.get(k).carrierPaid + m.get(k).shipperPaid, 2500000000n);
  }
});
test('invalid principal, colliding roles and duplicate order IDs rejected', () => {
  const { model: m, args } = fixture();
  assert.throws(() => m.fund(args, 100));
  assert.throws(() => m.fund({ ...args, id: 2, principal: 0n }, 100));
  assert.throws(() => m.fund({ ...args, id: 2, principal: -1n }, 100));
  assert.throws(() => m.fund({ ...args, id: 2, principal: 1n << 64n }, 100));
  assert.throws(() => m.fund({ ...args, id: 2, arbitrator: 'shipper' }, 100));
});
test('returned snapshots cannot mutate escrow roles or balances', () => {
  const { model: m, key: k } = fixture();
  const snapshot = m.get(k); snapshot.shipper = 'attacker'; snapshot.vault = 0n;
  assert.equal(m.get(k).shipper, 'shipper');
  assert.equal(m.get(k).vault, 2500000000n);
});
