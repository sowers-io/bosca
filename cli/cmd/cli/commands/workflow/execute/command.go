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

package execute

import (
	"bosca.io/cmd/cli/commands/flags"
	"bosca.io/cmd/cli/commands/workflow/execute/collection"
	"bosca.io/cmd/cli/commands/workflow/execute/metadata"
	"github.com/spf13/cobra"
)

var Command = &cobra.Command{
	Use:   "execute",
	Short: "Execute workflows",
}

func init() {
	Command.AddCommand(metadata.Command, collection.Command)
	Command.Flags().Bool(flags.ArgsFlag, false, "The args to use to find items")
	Command.Flags().Bool(flags.WaitFlag, false, "Wait for completion")
	Command.Flags().String(flags.EndpointFlag, "localhost:5011", "The endpoint to use.")
}
