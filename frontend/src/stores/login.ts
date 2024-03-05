import { UserDataDTO } from "common/bindings/UserDataDTO";
import { create } from "zustand";
import { LoginUserDTO } from "common/bindings/LoginUserDTO";
import { api } from "../api/base";
import { isAxiosError } from "axios";
import { ErrorMessage } from "common/bindings/ErrorMessage";

interface LoginStateStore {
  loading: boolean;
  userData: UserDataDTO | null;
  logIn: (arg0: LoginUserDTO) => Promise<void>;
  logOut: () => Promise<void>;
  clear: () => void;
  fetchUserData: () => Promise<void>;
}

export const useLoginState = create<LoginStateStore>((set, get) => ({
  loading: false,
  userData: null,
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
        console.error(e);
        get().clear();
      }
    }
  },
  fetchUserData: async () => {
    try {
      set({ loading: true });
      const response = await api.get<UserDataDTO>("/user");
      set({ userData: response.data, loading: false });
    } catch (e) {
      console.log(e);
      get().clear();
    }
  },
  logOut: async () => {
    await api.post("/user/logout");
    get().clear();
  },
  clear: () => {
    set({ loading: false, userData: null });
  },
}));
