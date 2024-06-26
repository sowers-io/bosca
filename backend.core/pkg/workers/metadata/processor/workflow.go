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

package processor

import (
	protobuf "bosca.io/api/protobuf/bosca"
	"bosca.io/api/protobuf/bosca/content"
	"bosca.io/pkg/workers/common"
	"context"
)

func CompleteTransition(ctx context.Context, metadata *content.Metadata) error {
	contentService := common.GetContentService(ctx)
	_, err := contentService.CompleteTransitionWorkflow(common.GetServiceAuthorizedContext(ctx), &content.CompleteTransitionWorkflowRequest{
		MetadataId: metadata.Id,
		Status:     "workflow transition",
		Success:    true,
	})
	return err
}

type Transition struct {
	Metadata *content.Metadata
	StateId  string
}

func TransitionTo(ctx context.Context, transition *Transition) error {
	contentService := common.GetContentService(ctx)
	_, err := contentService.BeginTransitionWorkflow(common.GetServiceAuthorizedContext(ctx), &content.TransitionWorkflowRequest{
		MetadataId: transition.Metadata.Id,
		StateId:    transition.StateId,
		Status:     "workflow transition",
	})
	return err
}

func GetTraitWorkflows(ctx context.Context, metadata *content.Metadata) ([]*content.TraitWorkflow, error) {
	if metadata.TraitIds == nil || len(metadata.TraitIds) == 0 {
		return nil, nil
	}

	svc := common.GetContentService(ctx)

	workflows, err := svc.GetWorkflows(common.GetServiceAuthorizedContext(ctx), &protobuf.Empty{})
	if err != nil {
		return nil, err
	}

	traits, err := svc.GetTraits(common.GetServiceAuthorizedContext(ctx), &protobuf.Empty{})
	if err != nil {
		return nil, err
	}

	traitsById := make(map[string]*content.Trait)
	for _, trait := range traits.Traits {
		traitsById[trait.Id] = trait
	}

	workflowsById := make(map[string]*content.Workflow)
	for _, workflow := range workflows.Workflows {
		workflowsById[workflow.Id] = workflow
	}

	traitWorkflows := make([]*content.TraitWorkflow, 0, len(metadata.TraitIds))
	for _, id := range metadata.TraitIds {
		if trait, ok := traitsById[id]; ok {
			for _, workflowId := range trait.TraitWorkflowIds {
				if workflow, ok := workflowsById[workflowId]; ok {
					traitWorkflows = append(traitWorkflows, &content.TraitWorkflow{
						TraitId:    trait.Id,
						WorkflowId: workflow.Id,
						Queue:      workflow.Queue,
						Metadata:   metadata,
					})
				}
			}
		}
	}

	return traitWorkflows, nil
}
