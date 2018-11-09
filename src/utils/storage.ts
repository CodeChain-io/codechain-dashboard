export const getObjectFromStorage = <T>(key: string): T | null | undefined => {
  if (typeof Storage !== "undefined") {
    const item = sessionStorage.getItem(key);
    if (item) {
      try {
        return JSON.parse(item);
      } catch (e) {
        // nothing
      }
    }
  }
  return undefined;
};

export const saveObjectToStorage = (key: string, data: object) => {
  if (typeof Storage !== "undefined") {
    sessionStorage.setItem(key, JSON.stringify(data));
  }
};
