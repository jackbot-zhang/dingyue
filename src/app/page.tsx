'use client';

import {invoke} from '@tauri-apps/api/tauri'
import React, {useEffect, useState} from 'react';

export default function Home() {
    const [message, setMessage] = useState<string>("");
    useEffect(() => {
        getProxies();
    }, []);

    function getProxies() {
        invoke('get_proxies')
            .then((proxy) => {
                setMessage(proxy as string);
            })
            .catch((error) => setMessage(error));
    }

    return (
        <main>
            <pre>{message}</pre>
        </main>
    )
}

