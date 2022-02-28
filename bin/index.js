#!/usr/bin/env node --enable-source-maps --experimental-modules --experimental-json-modules --no-warnings --es-module-specifier-resolution=node

import { run } from '../build/index';

void run();
