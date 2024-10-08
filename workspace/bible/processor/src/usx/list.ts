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

import { ListStyle, ListStyles } from './styles'
import { Attributes, StyleFactoryFilter, UsxContext, UsxItem, UsxItemContainer, UsxItemFactory } from './item'
import { Reference, ReferenceFactory } from './reference'
import { Footnote, FootnoteFactory } from './footnote'
import { Char, CharFactory } from './char'
import { Milestone, MilestoneFactory } from './milestone'
import { Figure, FigureFactory } from './figure'
import { Verse } from './verse'
import { Break, BreakFactory } from './break'
import { Text, TextFactory } from './text'
import { CrossReference, CrossReferenceFactory } from './cross_reference'
import { ListChar, ListCharFactory } from './list_char'
import { VerseStartFactory } from './verse_start'
import { VerseEndFactory } from './verse_end'

type ListType = Reference | Footnote | CrossReference | Char | ListChar | Milestone | Figure | Verse | Break | Text

export class List extends UsxItemContainer<ListType> {

  style: ListStyle
  vid: string | null

  constructor(context: UsxContext, parent: UsxItem | null, attributes: Attributes) {
    super(context, parent, attributes)
    this.style = attributes.STYLE.toString() as ListStyle
    this.vid = attributes.VID?.toString() || null
  }

  get htmlClass(): string {
    return this.style
  }
}

export class ListFactory extends UsxItemFactory<List> {

  static readonly instance = new ListFactory()

  private constructor() {
    super('para', new StyleFactoryFilter(ListStyles))
  }

  protected onInitialize() {
    this.register(ReferenceFactory.instance)
    this.register(FootnoteFactory.instance)
    this.register(CrossReferenceFactory.instance)
    this.register(CharFactory.instance)
    this.register(ListCharFactory.instance)
    this.register(MilestoneFactory.instance)
    this.register(FigureFactory.instance)
    this.register(VerseStartFactory.instance)
    this.register(VerseEndFactory.instance)
    this.register(BreakFactory.instance)
    this.register(TextFactory.instance)
  }

  create(context: UsxContext, parent: UsxItem | null, attributes: Attributes): List {
    return new List(context, parent, attributes)
  }
}