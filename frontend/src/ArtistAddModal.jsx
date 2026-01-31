import { useState, useCallback } from 'react'
import axios from 'axios';
import { useDebouncedCallback } from 'use-debounce';

function ArtistAddModal() {

    var [loadingArtist, setLoadingArtist] = useState(true);
    var [internalArtistText, setInternalArtistText] = useState("");
    var [searchResults, setSearchResults] = useState([]);
    var [mbid, setMbid] = useState(null);
    var [disableInput, setDisableInput] = useState(false);
    var [canSubmit, setCanSubmit] = useState(false);
    var [imageUrl, setImageUrl] = useState(null);


    const nameInput = useDebouncedCallback(
      (e) => {
        if (e.target.value == "") { return; }
        setLoadingArtist(true);
        axios.get('/api/search_artist', {
                params: {
                    query: e.target.value
                }
            })
            .then(function (response) {
                setSearchResults(response.data.results);
                setLoadingArtist(false);
            });
      },
      500
    );

    const nameInputBoth = (e) => {
      setInternalArtistText(e.target.value);
      nameInput(e);
    };


    var changeMbid = (result) => {
      setMbid(result.mbid);
      setDisableInput(true);
      setInternalArtistText(result.name);
      setCanSubmit(true);
    };

    var clear = useCallback(() => {
                setLoadingArtist(true);
                setSearchResults([]);
                setMbid(null);
                setDisableInput(false);
                setCanSubmit(false);
                setInternalArtistText("");
    });

    var submit = useCallback(() => {

      axios.get('/api/add_artist', {
                params: {
                    mbid: mbid
                }
            })
            .then(function (response) {
                clear();
            });
    });

    var readableResult = (result) => {
      if (result.disambig !== null && result.disambig !== undefined) { return result.name + " (" + result.disambig + ")"; }
      return result.name;
    }

    return (
        <div className="modal-box max-w-[50%]">
          <h3 className="text-lg font-bold modal-top">Add artist</h3>
          <div className="modal-middle py-[0.5rem]">
            { /* https://pastaable.com/tools/tailwind-grid-generator very helpful */ }
            <div className='grid grid-cols-8 grid-rows-6 gap-4'>
              <div className="col-start-1 row-start-1 col-span-5 row-span-6" id="inputs">
                <div className="">
                  <div className="">
                      <div className="dropdown w-full">
                        <input className="input" placeholder="Search" disabled={disableInput} value={internalArtistText} onChange={nameInputBoth} />
                        {(internalArtistText != "") ? <ul tabindex="0" className="dropdown-content z-[2] menu p-2 shadow bg-base-100 max-h-80 flex-nowrap overflow-auto" hidden={disableInput}>
                          <li>
                            { loadingArtist ? <a><span className="loading loading-dots loading-xs"></span></a> : 
                            searchResults.map((result) => {
                              return <a onClick={() => {changeMbid(result)}}>{readableResult(result)}</a>;
                            })}
                          </li>
                        </ul> : <></>}
                      </div>
                  </div>
                </div>
              </div>
              <div className='col-start-6 row-start-1 col-span-3 row-span-3' id="image">
                {mbid == null ? <div className='skeleton h-[15rem] w-[15rem]'></div> : 
                <div className='h-[15rem] w-[15rem]'>
                  <img className="h-full w-full" src={"/api/try_image?mbid=" + mbid} />
                </div>}
              </div>
              <div className='col-start-6 row-start-4 col-span-3 row-span-3' id="stuff">
                <div className="join"><p className='join-item text-neutral-400 text-xs self-start'>MBz ID:&nbsp;</p><p className='join-item self-center text-xs'>{mbid}</p></div>
              </div>
            </div>
          </div>
          <div className="modal-action">
            <label htmlFor="artist_add" className="btn bg-accent-content" disabled={!canSubmit} onClick={submit}>Add</label>
            <label htmlFor="artist_add" className="btn" onClick={clear}>Cancel</label>
          </div>
        </div>
    );
}

export default ArtistAddModal;