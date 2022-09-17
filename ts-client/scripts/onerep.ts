import axios from 'axios';
import { USER, TEST_USERS } from "./credits";
import { config } from '../src/config';

export async function getProfiles() {
  try {
    const resp = await axios({
      method: 'GET',
      url: `${config.ONEREP_BASE_URL}profiles`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.ONEREP_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function getProfile(id: number) {
  try {
    const resp = await axios({
      method: 'GET',
      url: `${config.ONEREP_BASE_URL}profiles/${id}`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.ONEREP_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function updateProfile(id: number) {
  try {
    const resp = await axios({
      method: 'PUT',
      url: `${config.ONEREP_BASE_URL}profiles/${id}`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.ONEREP_API_KEY}`
      },
      data: {
        first_name: "",
        last_name: "",
        middle_name: "",
        birth_date: "",
        first_names: [],
        last_names: [],
        middle_names: [],
        emails: [],
        phone_numbers: [],
        addressess: []
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function deleteProfile(id: number) {
  try {
    const resp = await axios({
      method: 'DELETE',
      url: `${config.ONEREP_BASE_URL}profiles/${id}`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.ONEREP_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function activateProfile(id: number) {
  try {
    const resp = await axios({
      method: 'PUT',
      url: `${config.ONEREP_BASE_URL}profiles/${id}/activate`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.ONEREP_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function deactivateProfile(id: number) {
  try {
    const resp = await axios({
      method: 'PUT',
      url: `${config.ONEREP_BASE_URL}profiles/${id}/deactivate`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.ONEREP_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function optoutProfile(id: number) {
  try {
    const resp = await axios({
      method: 'POST',
      url: `${config.ONEREP_BASE_URL}profiles/${id}/optout`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.ONEREP_API_KEY}`
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

export async function createProfile(user: USER) {
  try {
    const resp = await axios({
      method: 'GET',
      url: `${config.CRS_BASE_URL}users/identity`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
      },
      data: {
        first_name: "",
        last_name: "",
        middle_name: "",
        birth_date: "",
        first_names: [],
        last_names: [],
        middle_names: [],
        emails: [],
        phone_numbers: [],
        addressess: []
      }
    });
  } catch (error) {
    console.log('[Error] = ', error);
  }
}