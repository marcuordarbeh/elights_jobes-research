import React, { useEffect, useState } from 'react';
import UserService from '../services/UserService';

const DashboardPage = () => {
    const [content, setContent] = useState('');

    useEffect(() => {
        UserService.getUserBoard().then(
            (response) => {
                setContent(response.data);
            },
            (error) => {
                const _content =
                    (error.response && error.response.data) ||
                    error.message ||
                    error.toString();

                setContent(_content);
            }
        );
    }, []);

    return (
        <div className="dashboard-page">
            <header className="jumbotron">
                <h3>{content}</h3>
            </header>
        </div>
    );
};

export default DashboardPage;