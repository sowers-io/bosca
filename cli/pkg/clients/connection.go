/*
 * Copyright 2024 Sowers, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package clients

import (
	"crypto/tls"
	"errors"
	"strings"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
)

func NewClientConnection(endpoint string) (*grpc.ClientConn, error) {
	if endpoint == "" {
		return nil, errors.New("endpoint is required")
	}
	var opts []grpc.DialOption
	if strings.Contains(endpoint, "localhost") {
		opts = []grpc.DialOption{
			grpc.WithTransportCredentials(insecure.NewCredentials()),
			grpc.WithDefaultCallOptions(grpc.MaxCallRecvMsgSize(104857600)),
		}
	} else {
		opts = []grpc.DialOption{
			grpc.WithTransportCredentials(credentials.NewTLS(&tls.Config{})),
			grpc.WithDefaultCallOptions(grpc.MaxCallRecvMsgSize(104857600)),
		}
	}
	conn, err := grpc.Dial(endpoint, opts...)
	if err != nil {
		return nil, err
	}
	return conn, nil
}
