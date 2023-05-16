# NATS Prerequisites
To run this experiment you need a NATS server.  The simplest way to get a NATS server up and running is to follow the instructions to [install using a package manager](https://docs.nats.io/running-a-nats-service/introduction/installation#installing-via-a-package-manager).  You can then run the server:
```sh
$ nats-server
[6492] 2023/05/16 14:57:34.118649 [INF] Starting nats-server
[6492] 2023/05/16 14:57:34.118786 [INF]   Version:  2.9.16
[6492] 2023/05/16 14:57:34.118791 [INF]   Git:      [not set]
[6492] 2023/05/16 14:57:34.118806 [INF]   Name:     NCEIMJ4MCRXLVMWNS2NRHR44NKOY3HNSSFNORIC5FSI27DZYT2BSC4TI
[6492] 2023/05/16 14:57:34.118811 [INF]   ID:       NCEIMJ4MCRXLVMWNS2NRHR44NKOY3HNSSFNORIC5FSI27DZYT2BSC4TI
[6492] 2023/05/16 14:57:34.119611 [INF] Listening for client connections on 0.0.0.0:4222
[6492] 2023/05/16 14:57:34.120106 [INF] Server is ready
```

You will probably also want the NATS CLI as this allows you to send and receive messages from your NATS server - follow the [installation and use instructions](https://docs.nats.io/using-nats/nats-tools/nats_cli).

# Copyright and License
Copyright 2023, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.