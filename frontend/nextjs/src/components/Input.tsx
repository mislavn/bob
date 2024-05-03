import type { ChangeEventHandler, FC, InputHTMLAttributes } from 'react'
import type { FieldError, UseFormRegisterReturn } from 'react-hook-form'

type InputProps = {
  label: string
  id: string
  register?: UseFormRegisterReturn
  error?: FieldError
  onChange?: ChangeEventHandler<HTMLInputElement>
} & InputHTMLAttributes<HTMLInputElement>

export const Input: FC<InputProps> = ({
  label,
  id,
  register,
  error,
  onChange,
  ...inputProps
}) => {
  return (
    <>
      <label
        htmlFor={id}
        className="block text-sm font-medium leading-6 text-gray-900"
      >
        {label}
      </label>
      <div className="mb-2">
        <input
          id={id}
          className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
          onChange={(e) => {
            register?.onChange(e)
            onChange?.(e)
          }}
          {...register}
          {...inputProps}
        />
        {error?.message ? <span>{error.message}</span> : null}
      </div>
    </>
  )
}
