import { z } from "zod"

export const userSchema = z.object({
  id: z.string(),
  first_name: z.string(),
  last_name: z.string(),
  created_at: z.date(),
  updated_at: z.date()
})

export const sessionSchema = z.object({
  id: z.string(),
  user_id: z.string(),
  expires_at: z.date(),
  created_at: z.date(),
  updated_at: z.date()
})

export const userCredentialsSchema = z.object({
  user_id: z.string(),
  email: z.string(),
  hashed_password: z.string(),
  created_at: z.date(),
  updated_at: z.date()
})

export const signUpInputSchema = z.object({
  email: z.string(),
  password: z.string(),
  first_name: z.string(),
  last_name: z.string()
})

export const loginWithEmailInputSchema = z.object({
  email: z.string(),
  password: z.string()
})
