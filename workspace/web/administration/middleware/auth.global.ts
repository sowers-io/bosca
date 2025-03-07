export default defineNuxtRouteMiddleware((to, from) => {
  const cookie = useCookie('_bat')
  if (cookie.value) return
  if (
    to.path !== '/login' &&
    to.path !== '/forgotpassword' &&
    to.path !== '/signup' &&
    to.path !== '/signup/verify' &&
    to.path !== '/terms' &&
    to.path !== '/privacy'
  ) {
    return navigateTo('/login')
  }
})
