import axios from 'axios';
import { USER, TEST_USERS } from "./credits";
import { config } from '../src/config';

export async function getProfile(id: number) {
  try {
    const resp = await axios({
      method: 'GET',
      url: `${config.FACEBOOK_BASE_URL}${id}`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.FACEBOOK_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function postData(body: any) {
  try {
    const resp = await axios({
      method: 'POST',
      url: `${config.FACEBOOK_BASE_URL}me/feed`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.FACEBOOK_API_KEY}`
      },
      data: body
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function uploadData(source: any, caption: any) {
  try {
    const resp = await axios({
      method: 'POST',
      url: `${config.FACEBOOK_BASE_URL}me/photos`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.FACEBOOK_API_KEY}`
      },
      data: {
        source,
        caption
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function deleteData(id: number) {
  try {
    const resp = await axios({
      method: 'DELETE',
      url: `${config.FACEBOOK_BASE_URL}${id}`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.FACEBOOK_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}
