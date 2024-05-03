'use client'

import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import * as z from 'zod'

import { Input } from '@/components/Input'
import { LoginSchema } from '@/schemas'

export default function Login() {
  const { register, handleSubmit } = useForm<z.infer<typeof LoginSchema>>({
    resolver: zodResolver(LoginSchema),
    defaultValues: {
      email: '',
      password: '',
    },
  })

  return (
    <>
      <div className="flex  flex-1 flex-col justify-center">
        <div className="">
          <h2 className="text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
            Sign in to your account
          </h2>
        </div>

        <div className="mt-10 px-5 sm:mx-auto sm:w-full sm:max-w-sm sm:px-0">
          <form
            className="space-y-3"
            onSubmit={handleSubmit((val) => {
              console.log(val)
            })}
          >
            <div>
              <Input
                register={register('email')}
                id="email"
                label="Email address"
                type="email"
              />
              <Input
                register={register('password')}
                id="password"
                label="Password"
                type="password"
              />

              <button
                type="submit"
                className="mt-2 flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
              >
                Sign in
              </button>
            </div>
          </form>
        </div>
      </div>
    </>
  )
}
