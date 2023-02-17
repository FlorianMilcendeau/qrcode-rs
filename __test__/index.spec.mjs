import test from 'ava'

import {createQrCode} from '../index.js'

test("should create a qr code", (t) => {
    const qrcode = createQrCode({owner: "0x2OWNER", entity: "0x3ENTITY"});
    t.truthy(typeof qrcode === "string");
});
