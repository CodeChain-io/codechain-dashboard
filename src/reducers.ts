export interface AppReducer {
  dummyData: boolean;
}

const initialState: AppReducer = {
  dummyData: true
};

type Action = DummyAction;

interface DummyAction {
  type: "DummyAction";
  data: string;
}

export const appReducer = (state = initialState, action: Action) => {
  return state;
};
