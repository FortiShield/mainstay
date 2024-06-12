import { Idl, TypesCoder } from "@nx-pkg/mainstay";

export class SplAssociatedTokenAccountTypesCoder implements TypesCoder {
  constructor(_idl: Idl) {}

  encode<T = any>(_name: string, _type: T): Buffer {
    throw new Error(
      "SplAssociatedTokenAccount does not have user-defined types"
    );
  }
  decode<T = any>(_name: string, _typeData: Buffer): T {
    throw new Error(
      "SplAssociatedTokenAccount does not have user-defined types"
    );
  }
}