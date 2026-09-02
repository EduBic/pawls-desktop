import React from 'react';
// import { Logos } from '@allenai/varnish';


import styled from 'styled-components';

// const { AI2Logo } = Logos;

export const Header = () => {
    return (
        <>
            <h1>Logo placeholder</h1>
            {/* <AI2Logo color="white" size="micro" /> */}
            {/* <Logo src={pawlsLogo} /> */}
        </>
    );
};

const Logo = styled.img`
    margin: 20px 4px 10px 0px;
    padding: 4px;
    max-width: 100%;
`;
