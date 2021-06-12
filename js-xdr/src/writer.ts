import { constantCase } from "change-case";

import { writeFileSync, copyFileSync, mkdirSync } from "fs";
import { dirname, join } from "path";

import { determineDependencies, determineTypeReference, XdrType } from "../types/types";

export function initializeOutputPath(outputPath: string) {
  mkdirSync(outputPath, { recursive: true });
}

export function generateXdrDefinition(
  types: Record<string, XdrType>,
  constants: Record<string, number>,
  outputPath: string
) {
  let result =
    "#[allow(unused_imports)]\nuse alloc::boxed::Box;\n#[allow(unused_imports)]\nuse crate::xdr_codec::XdrCodec;\n";
  result +=
    "#[allow(unused_imports)]\nuse crate::streams::{ReadStream, ReadStreamError, WriteStream, WriteStreamError};\n";
  result +=
    "#[allow(unused_imports)]\nuse crate::compound_types::{LimitedVarOpaque, LimitedString, LimitedVarArray, UnlimitedVarOpaque, UnlimitedString, UnlimitedVarArray};\n\n";

  result +=
    Object.entries(constants)
      .map(([constant, value]) => `#[allow(dead_code)]\npub const ${constantCase(constant)}: i32 = ${value};\n`)
      .join("") + "\n";

  let toBeDone: string[];
  if (process.env.GENERATE_TYPES) {
    toBeDone = process.env.GENERATE_TYPES.split(",").map((name) => name.trim());
  } else {
    toBeDone = Object.keys(types); // generate all types
  }

  const done: string[] = [];

  let typeName: string | undefined;
  while ((typeName = toBeDone.pop())) {
    const typeDefinition = types[typeName];

    /*result += `
      export type ${typeName} = ${typeDefinition.tsType};
      export const ${typeName}: converter.XdrConverter<${typeName}> = converter.generator(() => ${
      typeDefinition.converter
    });
    `;*/

    if (typeDefinition.type !== "enum" && typeDefinition.type !== "struct" && typeDefinition.type !== "union") {
      result += `#[allow(dead_code)]\npub type ${typeName} = ${determineTypeReference(typeDefinition)};\n\n`;
    } else {
      const derive =
        typeDefinition.type === "enum" ? "Debug, Copy, Clone, Eq, PartialEq" : "Debug, Clone, Eq, PartialEq";
      result += `#[allow(dead_code)]\n#[derive(${derive})]\n${typeDefinition.typeDefinition}\n`;
      result += `impl XdrCodec for ${typeName} {${typeDefinition.typeImplementation}\n}\n\n`;
    }

    done.push(typeName);
    Object.keys(determineDependencies(typeDefinition)).forEach((key) => {
      if (done.indexOf(key) === -1 && toBeDone.indexOf(key) === -1) {
        toBeDone.push(key);
      }
    });
  }

  const mainFileName = process.env.MAIN_FILE_NAME;
  if (!mainFileName) {
    throw new Error('Environment variable "MAIN_FILE_NAME" not specified');
  }

  writeFileSync(join(outputPath, mainFileName), result);
}

const staticFiles = [
  "converters/basicTypes.ts",
  "converters/compoundTypes.ts",
  "converters/index.ts",
  "converters/streams.ts",
  "converters/types.ts",
  "utils/int64.ts",
  "index.ts",
];

export function copyStaticFiles(outputPath: string) {
  const usedDirectories: Record<string, boolean> = {};

  staticFiles.forEach((fileName) => {
    const directory = dirname(fileName);

    if (!usedDirectories[directory]) {
      usedDirectories[directory] = true;
      mkdirSync(join(outputPath, directory), { recursive: true });
    }

    copyFileSync(join(__dirname, "../../static/", fileName), join(outputPath, fileName));
  });
}