import { createSharedComposable } from '@vueuse/core'

export interface BreadcrumbLink {
  title: string | Ref<string>
  to?: string
}

export class BreadcrumbManager {
  public readonly links = ref<BreadcrumbLink[]>([])

  public set(breadcrumbs: BreadcrumbLink[]) {
    this.links.value = breadcrumbs
  }
}

let _manager: BreadcrumbManager | null = null

function _useBreadcrumbs(): BreadcrumbManager {
  if (!_manager) {
    _manager = new BreadcrumbManager()

    const router = useRouter()
    watch(router.currentRoute, () => {
      _manager!.set([])
    })
  }
  return _manager
}

export const useBreadcrumbs = createSharedComposable(_useBreadcrumbs)
