import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

function RecieveScreen() {
  const [ipAddresses, setIpAddresses] = useState([]); // State for IP addresses
  const [modalIsOpen, setModalIsOpen] = useState(false); // State for modal visibility

  const handleOpenModal = async () => { // Function to handle modal opening
    setModalIsOpen(true);
    try {
      const message = await invoke('find_ip_address');
      setIpAddresses(message);
    } catch (error) {
      console.error("Error:", error);
      // Handle errors appropriately (e.g., display error message to user)
    }
  };

  const handleCloseModal = () => {
    setModalIsOpen(false);
  };


  const connect = async (ip) => {
    console.log(`Here connect ${ip.ip_address}`);
    //
    const message = await invoke('connect_command', {ipConnect: ip.ip_address });
    console.log("${message}")

  }
  const handleIpClick = (ip) => {
    console.log(ip.ip_address);
  };
  useEffect(() => {
    if (modalIsOpen) { // Fetch data only when modal is open
      const fetchData = async () => {
        try {
          const message = await invoke('find_ip_address');
          setIpAddresses(message);
        } catch (error) {
          console.error("Error:", error);
          // Handle errors appropriately
        }
      };

      fetchData();
    }
  }, [modalIsOpen]); // Dependency array for `modalIsOpen`

  return (
    <div className="drawer-content flex flex-col items-center justify-center pt-20 mt-40">
      <div className="drawer-content flex flex-col items-center justify-center">
        <button className="btn" onClick={handleOpenModal}>
          Recieve Button
        </button>
        <dialog id="my_modal_5" className="modal modal-bottom sm:modal-middle" open={modalIsOpen}>
          <div className="modal-box">
            <h3 className="font-bold text-lg">Hello!</h3>
            {ipAddresses ? (
              <p className="py-4">Ip addresses: {ipAddresses.length === 0 ? (
                <p>No data found</p>
              ) : (
                <ul>
                  {ipAddresses.map((item) => (
                    <li key={item.id} onClick={() => connect(item)}> {item.hostName} {item.ip_address}</li>
                  ))}
                </ul>
              )}</p>
            ) : (
              <p className="py-4">Fetching IP addresses...</p>
            )}
            <div className="modal-action">
              <form method="dialog">
                <button className="btn" onClick={handleCloseModal}>
                  Close
                </button>
              </form>
            </div>
          </div>
        </dialog>
      </div>
    </div>
  );
}

export default RecieveScreen;
