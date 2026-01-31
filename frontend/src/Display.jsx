import { useState, useCallback } from 'react'
import axios from 'axios';
import React from 'react';

class Display extends React.Component {

    constructor(props) {
        super(props);
    }

    render() {

        var removeArtist = (id) => {
            axios.get('/api/delete_artist', {
                params: {
                    id: id
                }
            })
                .then(function (response) {
                });
        };

        var removeApi = (id) => {
            axios.get('/api/delete_api', {
                params: {
                    id: id
                }
            })
                .then(function (response) {
                });
        };

        return (
            <div class="grid grid-cols-4 grid-rows-3 gap-4 h-[80vh]">
                <div class="col-start-1 row-start-1 col-span-2 row-span-2">
                    <div className="h-full w-full bg-base-200 rounded-box p-[var(--radius-box)]">
                        <ul className="list bg-base-100 rounded-box shadow-md h-full">
                            <div className='max-h-full overflow-scroll'>
                                <li className="p-4 pb-2 text-xs opacity-60 tracking-wide">Artists</li>
                                {this.props.artists.map((artist) => {
                                    return <li className="list-row hover:bg-base-200 m-[0.5rem]">
                                        <div></div>
                                        <div>
                                            <a className="text-sm font-semibold" href="#" onClick={() => this.props.onArtistClick(artist)}>{artist.name}</a>
                                            <div className="text-xs font-semibold opacity-60 uppercase">{artist.mbid}</div>
                                        </div>
                                        <button className="btn btn-square btn-ghost btn-error" onClick={() => { removeArtist(artist.id) }}>
                                            -
                                        </button>
                                    </li>;
                                })}
                            </div>
                        </ul>
                    </div>
                </div>
                <div class="col-start-1 row-start-3 col-span-2 row-span-1">
                    <div className="h-full w-full bg-base-200 rounded-box p-[var(--radius-box)]">
                        <ul className="list bg-base-100 rounded-box shadow-md h-full">
                            <div className='max-h-full overflow-scroll'>
                                <li className="p-4 pb-2 text-xs opacity-60 tracking-wide">APIs</li>
                                {this.props.apis.map((api) => {
                                    return <li className="list-row hover:bg-base-200 m-[0.5rem]">
                                        <div></div>
                                        <div>
                                            <a className="text-sm font-semibold">{api.readable_name}</a>
                                            <div className="text-xs font-semibold opacity-60 uppercase">{api.id}</div>
                                        </div>
                                        <button className="btn btn-square btn-ghost btn-error" onClick={() => { removeApi(api.id) }}>
                                            -
                                        </button>
                                    </li>;
                                })}
                            </div>
                        </ul>
                    </div></div>
                <div class="col-start-3 row-start-1 col-span-2 row-span-3">
                    <div className="h-full w-full bg-base-200 rounded-box p-[var(--radius-box)]">
                        
                    </div></div>
            </div>
        );
    }

}

export default Display;