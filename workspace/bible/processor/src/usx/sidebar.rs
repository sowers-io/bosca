// export class Sidebar extends UsxItemContainer<SidebarType> {
//
//     style: SidebarStyle
//     category?: String
//
//     constructor(context: UsxContext, parent: UsxItem | null, attributes: Attributes) {
//         super(context, parent, attributes)
//         this.style = attributes.STYLE.toString() as SidebarStyle
//         this.category = attributes.CATEGORY?.toString()
//     }
//
//     get htmlClass(): string {
//         return this.style
//     }
// }