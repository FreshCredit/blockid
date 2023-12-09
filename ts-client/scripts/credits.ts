
export interface USER {
  id: string,
  fname: string,
  lname: string,
  email: string,
  mobile: string,
  photo: string,
  ssn: string, // Social Serial Number
  dob: string, // Date of Birth
  street1: string,
  street2: string,
  city: string,
  state: string,
  zip: string
}

export const TEST_USERS: USER[] = [
  {
    id: "1",
    fname: "John",
    lname: "Doe",
    email: "johndoe4@mail.com",
    mobile: "5555555528",
    photo: "",
    ssn: "",
    dob: "",
    street1: "",
    street2: "",
    city: "",
    state: "",
    zip: ""
  }
]
