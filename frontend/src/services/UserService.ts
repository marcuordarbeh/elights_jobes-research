import axios from 'axios';

const API_URL = '/api/';

class UserService {
    getUserBoard() {
        return axios.get(API_URL + 'dashboard', { headers: this.authHeader() });
    }

    authHeader() {
        const user = JSON.parse(localStorage.getItem('user')!);

        if (user && user.token) {
            return { Authorization: 'Bearer ' + user.token };
        } else {
            return {};
        }
    }
}

export default new UserService();