import { atom } from "jotai";

export enum TestPanelViewType {
  TABULAR = 'tabular',
  CARD_EXPANDED = 'card_expanded',
  CARD_SIMPLE = 'card_simple',
  CLIENT_GRAPH = 'client_graph'
}

export type ResponseViewType = 'parsed' | 'pretty' | 'raw';

export interface TabularViewConfig {
  showInputs: boolean;
  showModel: boolean;
  responseViewType: ResponseViewType;
}

export const testPanelViewTypeAtom = atom<TestPanelViewType>(TestPanelViewType.CARD_EXPANDED);
export const tabularViewConfigAtom = atom<TabularViewConfig>({
  showInputs: true,
  showModel: false,
  responseViewType: 'parsed',
}); 