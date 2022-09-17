
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
    email: "johndoe@mail.com",
    mobile: "19371234567",
    photo: "",
    ssn: "",
    dob: "",
    street1: "",
    street2: "",
    city: "",
    state: "",
    zip: ""
  },
  {
    id: "2",
    fname: "Bobby",
    lname: "Smith",
    email: "bobbysmith@mail.com",
    mobile: "19371234567",
    photo: "",
    ssn: "",
    dob: "",
    street1: "",
    street2: "",
    city: "",
    state: "",
    zip: ""
  },
  {
    id: "3",
    fname: "Kyle",
    lname: "Abrams",
    email: "kyleabrams@mail.com",
    mobile: "19371234567",
    photo: "",
    ssn: "",
    dob: "",
    street1: "",
    street2: "",
    city: "",
    state: "",
    zip: ""
  },
  {
    id: "4",
    fname: "David",
    lname: "Simpson",
    email: "davidsympson@mail.com",
    mobile: "19371234567",
    photo: "",
    ssn: "",
    dob: "",
    street1: "",
    street2: "",
    city: "",
    state: "",
    zip: ""
  },
]
