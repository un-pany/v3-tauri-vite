import dayjs from "dayjs"

/** 格式化时间 */
export const formatDateTime = (time: string | number | Date) => {
  if (!time) return ""
  const date = new Date(time)
  return dayjs(date).format("YYYY-MM-DD HH:mm:ss")
}
