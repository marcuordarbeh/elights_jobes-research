import axios from 'axios';

const API_URL = '/api/';

class AuthService {
    login(username: string, password: string) {
        return axios
            .post(API_URL + 'login', {
                username,
                password
            })
            .then(response => {
                if (response.data.token) {
                    localStorage.setItem('user', JSON.stringify(response.data));
                }
                return response.data;
            });
    }

    logout() {
        localStorage.removeItem('user');
    }

    register(username: string, password: string, role: string) {
        return axios.post(API_URL + 'register', {
            username,
            password,
            role
        });
    }
}

export default new AuthService();