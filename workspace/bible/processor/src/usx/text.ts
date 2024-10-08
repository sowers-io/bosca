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

import { Attributes, HtmlContext, StringContext, UsxContext, UsxItem, UsxItemFactory } from './item'
import { Position } from './position'

export class Text implements UsxItem {
  text: string = ''

  readonly position: Position
  readonly verse: string | null

  constructor(context: UsxContext, parent: UsxItem | null) {
    this.position = context.position
    this.verse = context.addVerseItem(parent, this)
  }

  get htmlClass(): string {
    return 'verse'
  }

  get htmlAttributes(): { [p: string]: string } {
    if (!this.verse) return {}
    return { 'data-verse': this.verse }
  }

  toHtml(context: HtmlContext): string {
    if (this.text.trim().length === 0) return ''
    return context.render('span', this, this.text)
  }

  toString(context: StringContext | undefined = undefined): string {
    const ctx = context || StringContext.defaultContext
    if (!ctx.includeNewLines) return this.text.replace(/\r?\n/g, '')
    return this.text
  }
}

export class TextFactory extends UsxItemFactory<Text> {

  static readonly instance = new TextFactory()

  private constructor() {
    super('#text')
  }

  protected onInitialize() {
  }

  create(context: UsxContext, parent: UsxItem | null, _: Attributes): Text {
    return new Text(context, parent)
  }
}