import { isAxiosError } from "axios";
import { ErrorMessage } from "common/bindings/ErrorMessage";
import { LoginUserDTO } from "common/bindings/LoginUserDTO";
import { UserDataDTO } from "common/bindings/UserDataDTO";
import { create } from "zustand";
import { api } from "../api/base";

interface LoginStateStore {
  loading: boolean;
  userData: UserDataDTO | null;
  errors: Error[] | null;
  logIn: (arg0: LoginUserDTO) => Promise<void>;
  logOut: () => Promise<void>;
  clear: () => void;
  fetchUserData: () => Promise<void>;
  pushError: (e: unknown) => void;
}

export const useLoginState = create<LoginStateStore>((set, get) => ({
  loading: false,
  userData: null,
  errors: null,
  logIn: async ({ username, password }) => {
    try {
      set({ loading: true });
      const response = await api.post<UserDataDTO>("/user/login", {
        username,
        password,
      } satisfies LoginUserDTO);
      set({ userData: response.data, loading: false });
    } catch (e) {
      if (
        isAxiosError<ErrorMessage<string>>(e) &&
        e.response?.data.kind === "AlreadyLoggedIn" &&
        e.response.status === 400
      ) {
        get().fetchUserData();
      } else {
        get().clear();
        get().pushError(e);
      }
    }
  },
  fetchUserData: async () => {
    try {
      set({ loading: true });
      const response = await api.get<UserDataDTO>("/user");
      set({ userData: response.data, loading: false });
    } catch (e) {
      get().clear();
      get().pushError(e);
    }
  },
  logOut: async () => {
    get().clear();
    await api.post("/user/logout").catch(get().pushError);
  },
  pushError: (e: unknown) => {
    set((state) => ({ errors: [...(state.errors ?? []), e as Error] }));
  },
  clear: () => {
    set({ loading: false, userData: null, errors: null });
  },
}));
