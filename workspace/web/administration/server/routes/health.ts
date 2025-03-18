export default defineEventHandler(() => {
  return {
    uptime: process.uptime(),
    message: `OK`,
    timestamp: Date.now(),
  }
})
