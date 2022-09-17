import axios from 'axios';
import { USER, TEST_USERS } from "./credits";
import { config } from '../src/config';

export async function registerAccount(user: USER) {
  try {
    const resp = await axios({
      method: 'post',
      url: `${config.CRS_BASE_URL}direct/user-reg`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
      },
      data: {
        email: user.email,
        mobile: user.mobile,
        fname: user.fname,
        lname: user.lname,
        smsMsg: true,
        emailMsg: true,
        pushMsg: true
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function userIdntity(user: USER) {
  try {
    const resp = await axios({
      method: 'post',
      url: `${config.CRS_BASE_URL}users/identity`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
      },
      data: {
        ssn: user.ssn,
        dob: user.dob,
        mobile: user.mobile,
        street1: user.street1,
        street2: user.street2,
        city: user.city,
        state: user.state,
        zip: user.zip
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function sendMobileCode(mtoken: any) {
  try {
    const resp = await axios({
      method: 'post',
      url: `${config.CRS_BASE_URL}users/send-code`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
      },
      data: {
        mtoken
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function renewMobileCode(key: any) {
  try {
    const resp = await axios({
      method: 'post',
      url: `${config.CRS_BASE_URL}users/renew-code`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
      },
      data: {
        key
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function verifyMobileCode(key: any, code: string) {
  try {
    const resp = await axios({
      method: 'post',
      url: `${config.CRS_BASE_URL}users/verify-code`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
      },
      data: {
        key,
        code
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function getIdentityQuiz() {
  try {
    const resp = await axios({
      method: 'GET',
      url: `${config.CRS_BASE_URL}users/get-quiz`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function verifyIdentityQuiz(key: any, id: number, answers: any) {
  try {
    const resp = await axios({
      method: 'post',
      url: `${config.CRS_BASE_URL}users/verify-quiz`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
      },
      data: {
        key,
        id,
        answers
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}