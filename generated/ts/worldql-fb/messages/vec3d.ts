// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

export class Vec3d {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):Vec3d {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

x():number {
  return this.bb!.readFloat64(this.bb_pos);
}

y():number {
  return this.bb!.readFloat64(this.bb_pos + 8);
}

z():number {
  return this.bb!.readFloat64(this.bb_pos + 16);
}

static sizeOf():number {
  return 24;
}

static createVec3d(builder:flatbuffers.Builder, x: number, y: number, z: number):flatbuffers.Offset {
  builder.prep(8, 24);
  builder.writeFloat64(z);
  builder.writeFloat64(y);
  builder.writeFloat64(x);
  return builder.offset();
}

}