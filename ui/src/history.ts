type Subscriber = (path: string) => void;

const subscribers: Subscriber[] = [];

/**
 * Push page onto the history stack
 */
export function push(path: string, title = ""): void {
  window.history.pushState({}, title, path);
  subscribers.forEach((subscriber) => subscriber(path));
}

export function subscribe(subscriber: Subscriber): void {
  subscribers.push(subscriber);
}

export function unsubscribe(subscriber: Subscriber): void {
  const index = subscribers.indexOf(subscriber);
  if (index) subscribers.splice(index, 1);
}
