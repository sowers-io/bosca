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

import { FootnoteStyle } from './styles'
import { Text, TextFactory } from './text'
import {
  Attributes,
  HtmlContext,
  StringContext,
  StyleFactoryFilter,
  UsxContext,
  UsxItem,
  UsxItemContainer,
  UsxItemFactory,
} from './item'
import { FootnoteStyles } from './styles'
import { FootnoteChar, FootnoteCharFactory } from './footnote_char'

type FootnoteItem = FootnoteChar | Text

export class Footnote extends UsxItemContainer<FootnoteItem> {
  style: FootnoteStyle
  caller: string
  category?: string

  constructor(context: UsxContext, parent: UsxItem | null, attributes: Attributes) {
    super(context, parent, attributes)
    this.style = attributes.STYLE.toString() as FootnoteStyle
    this.caller = attributes.CALLER.toString()
    this.category = attributes.CATEGORY?.toString()
  }

  get htmlClass(): string {
    return this.style
  }

  toHtml(context: HtmlContext): string {
    if (!context.includeFootNotes) return ''
    return super.toHtml(context)
  }

  toString(context: StringContext | undefined = undefined): string {
    const ctx = context || StringContext.defaultContext
    if (!ctx.includeFootNotes) return ''
    return super.toString(context)
  }
}

export class FootnoteFactory extends UsxItemFactory<Footnote> {

  static readonly instance = new FootnoteFactory()

  private constructor() {
    super('note', new StyleFactoryFilter(FootnoteStyles))
  }

  protected onInitialize() {
    this.register(FootnoteCharFactory.instance)
    this.register(TextFactory.instance)
  }

  create(context: UsxContext, parent: UsxItem | null, attributes: Attributes): Footnote {
    return new Footnote(context, parent, attributes)
  }
}

