import { useState, useCallback } from 'react'
import axios from 'axios';
import { useDebouncedCallback } from 'use-debounce';
import React from 'react';

class ApiAddModal extends React.Component {

  constructor(props) {
    super(props);

    this.state = { api: "", api_readable: "", required: [], apis: [], responses: {} };

    axios.get('/api/get_sources', {
                params: {
                }
            })
            .then((response) => {
                this.setState({apis: response.data.sources});
            });
  }

  merge(ob, k, v) {
    let newob = structuredClone(ob);
    newob[k] = v;
    return newob;
  }

  render() {
  return (
    <div className="modal-box max-w-[50%]">
      <h3 className="text-lg font-bold modal-top">Add API</h3>
      <div className="modal-middle py-[0.5rem]">

        <div className="dropdown w-full">
          <input className="input" placeholder="Search" value={this.state.api_readable} onInput={(e) => this.setState({api: e.target.value})} />
          <ul tabindex="0" className="dropdown-content z-[2] menu p-2 shadow bg-base-100 max-h-80 flex-nowrap overflow-auto">
            <li>
                   {this.state.apis.map((v, i) => {
                     return <a onClick={() => { this.setState({api: v.name, api_readable: v.readable, required: v.params, responses: {}}) }}>{v.readable}</a>
                   })}
                
            </li>
          </ul>
        </div>
        
        {this.state.required.map((v, i) => {
                     return <div><br /><label className="input py-2">
                            {v}
                            <input type="text" className="grow" onInput={(e) => this.setState({ responses: this.merge(this.state.responses, v, e.target.value) })} value={this.state.responses[v] ? this.state.responses[v] : ""} />
                            <span className="badge badge-neutral badge-xs">Required</span>
                          </label></div>;
          })}

        <div className="modal-action">
          <label htmlFor="api_add" className="btn bg-accent-content"
          onClick={() => {
            console.log(this.state);
            axios.post('/api/add_api', {
                  name: this.state.api,
                  params: this.state.responses
            })
            .then((response) => {
            });
          }}
          >Add</label>
          <label htmlFor="api_add" className="btn">Cancel</label>
        </div>
      </div>
    </div>
  );
  }
}

export default ApiAddModal;