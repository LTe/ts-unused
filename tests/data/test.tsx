export type User = {
  username: string,
  age: number;
}

const john: User = {
  username: 'john',
  age: 21
}

export const ReactComponent = (props: User) => {
  return <p>{props._username}</p>
}
