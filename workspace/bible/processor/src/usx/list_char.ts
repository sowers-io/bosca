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

import { Reference, ReferenceFactory } from './reference'
import { Milestone, MilestoneFactory } from './milestone'
import { Footnote, FootnoteFactory } from './footnote'
import { Break, BreakFactory } from './break'
import { Text, TextFactory } from './text'
import { Attributes, StyleFactoryFilter, UsxContext, UsxItem, UsxItemContainer, UsxItemFactory } from './item'
import { ListCharStyle, ListCharStyles } from './styles'
import { Char, CharFactory } from './char'

type ListCharType = Reference | Char | Milestone | Footnote | Break | Text

export class ListChar extends UsxItemContainer<ListCharType> {

  style: ListCharStyle
  // char.link?
  // char.closed?

  constructor(context: UsxContext, parent: UsxItem | null, attributes: Attributes) {
    super(context, parent, attributes)
    this.style = attributes.STYLE.toString() as ListCharStyle
  }

  get htmlClass(): string {
    return this.style
  }
}

export class ListCharFactory extends UsxItemFactory<ListChar> {

  static readonly instance = new ListCharFactory()

  constructor() {
    super('char', new StyleFactoryFilter(ListCharStyles))
  }

  protected onInitialize() {
    this.register(ReferenceFactory.instance)
    this.register(CharFactory.instance)
    this.register(MilestoneFactory.instance)
    this.register(FootnoteFactory.instance)
    this.register(BreakFactory.instance)
    this.register(TextFactory.instance)
  }

  create(context: UsxContext, parent: UsxItem | null, attributes: Attributes): ListChar {
    return new ListChar(context, parent, attributes)
  }
}
