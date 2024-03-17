import { Popover, Transition } from "@headlessui/react";
import { EnterIcon, ExitIcon, ReloadIcon } from "@radix-ui/react-icons";
import clsx from "clsx";
import { LoginUserDTO } from "common/bindings/LoginUserDTO";
import { FC, HTMLProps, forwardRef } from "react";
import { Field, Form } from "react-final-form";
import { useLoginState } from "../stores/login";
const Label = forwardRef<HTMLLabelElement, HTMLProps<HTMLLabelElement>>(
  ({ className, children, ...props }, ref) => (
    <label
      ref={ref}
      {...props}
      className={clsx("font-bold mb-2", className)}
    >
      {children}
    </label>
  ),
);

const Input = forwardRef<HTMLInputElement, HTMLProps<HTMLInputElement>>(
  ({ className, ...props }, ref) => (
    <input
      {...props}
      className={clsx("bg-zinc-100 border border-black p-2", className)}
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
      <div className="p-2 w-fit">
        <div
          className={clsx(
            [
              "p-2 h-16 aspect-square",
              "border border-black",
              "bg-zinc-300 text-black",
              "flex items-center justify-center",
            ],
            className,
          )}
        >
          <ReloadIcon className="animate-spin" />
        </div>
      </div>
    );
  }

  return userData !== null ? (
    <div
      className={clsx(
        "flex flex-row flex-nowrap items-center gap-8 p-2 w-fit max-w-96 bg-zinc-300 border-black border text-black",
        className,
      )}
    >
      <div className="h-16 aspect-square bg-red-400 border border-black" />
      <div className="font-bold min-w-[16ch]">{userData.username}</div>
      <button
        className="flex justify-center items-center h-16 aspect-square bg-zinc-200 border border-black"
        onClick={() => logOut()}
        type="button"
        aria-label="Log out"
        title="Log out"
      >
        <ExitIcon />
      </button>
    </div>
  ) : (
    <Popover className="relative m-2">
      <Popover.Button
        className={clsx(
          [
            "m-2 p-2 h-16 aspect-square",
            "border border-black",
            "bg-zinc-300 text-black",
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
        <Popover.Panel className="p-4 absolute z-10 left-0 bg-zinc-200 border border-black text-black shadow shadow-zinc-400">
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
                  className="w-full bg-zinc-400 p-2 border border-black"
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
