import { useState, useCallback } from 'react'
import axios from 'axios';
import { useDebouncedCallback } from 'use-debounce';
import React from 'react';

class ArtistModifyModal extends React.Component {

  constructor(props) {
    super(props);

    this.state = { kvs: [] };

  }

  componentDidUpdate(prevProps) {
    if (this.props.selectedArtist === null) {
      return;
    }
    if (((prevProps.selectedArtist ? prevProps.selectedArtist : { "id": null }).id != this.props.selectedArtist.id)) {
      axios.get('/api/artist_kvs', {
        params: {
          artist_id: this.props.selectedArtist.id
        }
      })
        .then((response) => {
          let kvs = [];
          for (var known_kv of this.props.kvs) {
            // internal name, source it's for, readable name, value
            let found = false;
            for (var kv of response.data.kvs) {
              if (known_kv[0] == kv[0]) {
                kvs.push([known_kv[0], known_kv[1], known_kv[2], kv[1]])
                found = true;
                continue;
              }
            }
            if (!found) { kvs.push(known_kv); }
          }
          this.setState({ kvs: kvs })
        });
    }
  }

  render() {
    return (
      <div className="modal-box max-w-[50%]">
        <h3 className="text-lg font-bold modal-top">Modify artist</h3>
        <div className="modal-middle py-[0.5rem]">
          {this.state.kvs.map((v, i) => {
            return <div><br /><label className="input py-2">
              {v[2]}
              <input type="text" className="grow" onInput={(e) => {
                let kvs = [];
                for (var kv of this.state.kvs) {
                  if (kv[0] !== v[0]) {
                    continue;
                  }
                  kvs.push([kv[0], kv[1], kv[2], e.target.value]);
                }
                this.setState({ kvs: kvs });
              }} value={v[3] ? v[3] : ""} />
            </label></div>;
          })}
          <div className="modal-action">
            <label htmlFor="artist_modify" className="btn bg-accent-content"
              onClick={() => {
                // TODO: clear everything after so it loads changes when opening, closing, opening the same artist 
                for (let kv of this.state.kvs) {
                  if (kv.length === 4) {
                    for (var api of this.props.apis) {
                      if (api.name === kv[1]) {
                        axios.post('/api/set_kv', {
                          artist_id: this.props.selectedArtist.id,
                          api_id: api.id,
                          key: kv[0],
                          value: kv[3],
                        });
                      }
                    }
                  }
                }
              }}
            >Save</label>
            <label htmlFor="artist_modify" className="btn">Cancel</label>
          </div>
        </div>
      </div>
    );
  }
}

export default ArtistModifyModal;