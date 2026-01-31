import { useState } from 'react'
import React from 'react';
import axios from 'axios';
import './App.css'
import ArtistAddModal from './ArtistAddModal.jsx';
import ApiAddModal from './ApiAddModal.jsx';
import Display from './Display.jsx';

// designed with much inspiration from daisy-components at https://github.com/willpinha/daisy-components

class App extends React.Component {

  constructor(props) {
    super(props);

    this.state = { artists: [], apis: [] };
    this.updateArtists = this.updateArtists.bind(this);
    this.updateApis = this.updateApis.bind(this);
  }


  updateArtists = (t) => {
    axios.get('/api/list_artists')
      .then(function (response) {
        t.setState({ artists: response.data.artists });
      });
  }
  updateApis = (t) => {
    axios.get('/api/list_apis')
      .then(function (response) {
        t.setState({ apis: response.data.apis });
      });
  }

  componentDidMount() {
    this.setState({ artists: [] });
    setInterval(() => { this.updateArtists(this); this.updateApis(this); }, 1000);
  }

  displayArtistClick(artist) {
    console.log(artist);
  }

  render() {
    return (
      <div className="p-[1rem]" id="content">
        <nav id="top_bar" className="navbar justify-between bg-base-300 h-[10vh]">
          <a className="btn btn-ghost text-lg">
            concave
          </a>

          <div className="hidden sm:flex gap-2">

            <div className="dropdown dropdown-end">
              <button className="btn btn-sm btn-primary mr-[1rem]">
                +
              </button>

              <ul tabindex="0" className="dropdown-content menu z-[1] bg-base-200 px-3 py-3 rounded-box shadow w-56 gap-2">
                <li><label htmlFor={'artist_add'}>Artist</label ></li>
                <li><label htmlFor={'api_add'}>API</label></li>
              </ul>
            </div>
          </div>
        </nav>
        <br />
        <div className='h-full w-full'>
          <Display apis={this.state.apis} artists={this.state.artists} onArtistClick={this.displayArtistClick}/>
        </div>

        {/* Normally, since buttons in modals in daisyUI close the modal, we need to use the "legacy checkbox method" to close them,
    so that we are able to free up buttons to work within the modal. */}
        <input type="checkbox" id="artist_add" className="modal-toggle" />
        <div className="modal" role="dialog">
          <ArtistAddModal />
        </div>

        <input type="checkbox" id="api_add" className="modal-toggle" />
        <div className="modal" role="dialog">
            <ApiAddModal />
        </div>

      </div>
    )
  }
}

export default App
