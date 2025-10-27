export interface Settings {
  workSecs: number;
  restSecs: number;
  autoStartApp: boolean;
}
export const DefaultSettings: Settings = {
  workSecs: 0,
  restSecs: 0,
  autoStartApp: false,
};
