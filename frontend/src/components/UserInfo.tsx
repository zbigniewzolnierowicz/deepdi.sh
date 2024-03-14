import { Popover, Transition } from "@headlessui/react";
import { EnterIcon, ExitIcon, ReloadIcon } from "@radix-ui/react-icons";
import clsx from "clsx";
import { LoginUserDTO } from "common/bindings/LoginUserDTO";
import { FC, HTMLProps, forwardRef } from "react";
import { Field, Form } from "react-final-form";
import { useLoginState } from "../stores/login"; const Label = forwardRef<HTMLLabelElement, HTMLProps<HTMLLabelElement>>( ({ className, children, ...props }, ref) => (
    <label
      ref={ref}
      {...props}
      className={clsx("text-white font-bold mb-2", className)}
    >
      {children}
    </label>
  ),
);

const Input = forwardRef<HTMLInputElement, HTMLProps<HTMLInputElement>>(
  ({ className, ...props }, ref) => (
    <input
      {...props}
      className={clsx("bg-slate-800 rounded p-2", className)}
      ref={ref}
    />
  ),
);

export const UserInfo: FC<{ className?: string }> = ({ className }) => {
  const { userData, loading, logIn, logOut } = useLoginState();

  const onSubmit = (r: LoginUserDTO) => {
    logIn(r);
  };

  if (loading) {
    return (
      <div
        className={clsx(
          [
            "m-2 p-2 h-16 aspect-square",
            "rounded-xl",
            "bg-slate-700 text-white",
            "flex items-center justify-center",
          ],
          className,
        )}
      >
        <ReloadIcon className="animate-spin" />
      </div>
    );
  }

  return userData !== null ? (
    <div
      className={clsx(
        "flex flex-row flex-nowrap items-center gap-8 p-2 rounded-2xl w-fit max-w-96 bg-slate-700",
        className,
      )}
    >
      <div className="h-16 aspect-square bg-red-400 rounded-xl" />
      <div className="text-white font-bold min-w-[16ch]">
        {userData.username}
      </div>
      <button
        className="flex justify-center items-center h-16 aspect-square bg-slate-800 rounded-xl"
        onClick={() => logOut()}
        type="button"
        aria-label="Log out"
        title="Log out"
      >
        <ExitIcon className="text-white" />
      </button>
    </div>
  ) : (
    <Popover className="relative m-2">
      <Popover.Button
        className={clsx(
          [
            "m-2 p-2 h-16 aspect-square",
            "rounded-xl",
            "bg-slate-700 text-white",
            "flex items-center justify-center",
          ],
          className,
        )}
      >
        {({ open }) => (
          <EnterIcon
            className={clsx("transition-transform ease-in-out", {
              "rotate-90": open,
            })}
          />
        )}
      </Popover.Button>
      <Transition
        enter="transition duration-100 ease-out"
        enterFrom="transform scale-95 opacity-0"
        enterTo="transform scale-100 opacity-100"
        leave="transition duration-75 ease-out"
        leaveFrom="transform scale-100 opacity-100"
        leaveTo="transform scale-95 opacity-0"
      >
        <Popover.Panel className="p-4 absolute z-10 right-0 bg-slate-700 rounded-xl text-white shadow shadow-slate-800">
          <Form onSubmit={onSubmit}>
            {(props) => (
              <form
                onSubmit={props.handleSubmit}
                className="flex flex-col flex-wrap gap-4"
              >
                <div className="flex flex-col">
                  <Label htmlFor="login-username">Username</Label>
                  <Field name="username" id="login-username" component="input">
                    {({ input }) => <Input {...input} />}
                  </Field>
                </div>
                <div className="flex flex-col">
                  <Label htmlFor="password">Password</Label>
                  <Field
                    name="password"
                    id="login-password"
                    component="input"
                    type="password"
                  >
                    {({ input }) => <Input {...input} />}
                  </Field>
                </div>
                <button
                  className="w-full bg-slate-800 p-2 rounded-lg"
                  type="submit"
                >
                  Log in
                </button>
              </form>
            )}
          </Form>
        </Popover.Panel>
      </Transition>
    </Popover>
  );
};
