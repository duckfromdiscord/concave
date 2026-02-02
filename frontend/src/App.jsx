import { useState } from 'react'
import React from 'react';
import axios from 'axios';
import './App.css'
import ArtistAddModal from './ArtistAddModal.jsx';
import ApiAddModal from './ApiAddModal.jsx';
import Display from './Display.jsx';
import ArtistModifyModal from './ArtistModifyModal.jsx';

// designed with much inspiration from daisy-components at https://github.com/willpinha/daisy-components

class App extends React.Component {

  constructor(props) {
    super(props);

    this.state = { artists: [], apis: [], sources: [], source_kvs: [], selectedArtist: null };
    this.updateArtists = this.updateArtists.bind(this);
    this.updateApis = this.updateApis.bind(this);
    this.updateSources = this.updateSources.bind(this);
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
  updateSources = (t) => {
    axios.get('/api/get_sources')
      .then((response) => {
        t.setState({sources: response.data.sources});
        var source_kvs = [];
        for (var source of response.data.sources) {
          var name = source.name;
          for (var kv of source.artist_kv) {
            // internal name, source it's for, readable name
            source_kvs.push([kv[0], name, kv[1]]);
          }
        }
        this.setState({source_kvs: source_kvs});
      });
  }

  componentDidMount() {
    this.setState({ artists: [] });
    setInterval(() => { this.updateArtists(this); this.updateApis(this); this.updateSources(this); }, 1000);
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
          <Display apis={this.state.apis} artists={this.state.artists} onArtistClick={(artist) => {
    document.getElementById("artist_modify").checked = true;
    this.setState({selectedArtist: artist});
    }}/>
        </div>

        {/* Normally, since buttons in modals in daisyUI close the modal, we need to use the "legacy checkbox method" to close them,
    so that we are able to free up buttons to work within the modal. */}
        <input type="checkbox" id="artist_add" className="modal-toggle" />
        <div className="modal" role="dialog">
          <ArtistAddModal sources={this.state.sources} apis={this.state.apis} />
        </div>

        <input type="checkbox" id="api_add" className="modal-toggle" />
        <div className="modal" role="dialog">
            <ApiAddModal sources={this.state.sources} />
        </div>

        <input type="checkbox" id="artist_modify" className="modal-toggle" />
        <div className="modal" role="dialog">
            <ArtistModifyModal kvs={this.state.source_kvs} selectedArtist={this.state.selectedArtist} apis={this.state.apis} sources={this.state.sources} />
        </div>


      </div>
    )
  }
}

export default App
