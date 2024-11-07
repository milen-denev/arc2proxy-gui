const { invoke } = window.__TAURI__.core;

//GENERAL 

async function load_configuration() {
  await get_configuration();
  const listening_address = document.querySelector('#listening_address');
  const listening_port_http = document.querySelector('#listening_port_http');
  const listening_port_https = document.querySelector('#listening_port_https');
  const logging_level = document.querySelector('#logging_level');
  const add_caching = document.querySelector('#add_caching');
  const add_rate_limiting = document.querySelector('#add_rate_limiting');
  const add_logging = document.querySelector('#add_logging');
  const disable_default_body_limit = document.querySelector('#disable_default_body_limit');
  const add_sql_injection_protection = document.querySelector('#add_sql_injection_protection');
  const recv_buffer_size = document.querySelector('#recv_buffer_size');
  const send_buffer_size = document.querySelector('#send_buffer_size');
  const ip_ttl = document.querySelector('#ip_ttl');
  const tcp_keep_alive_seconds = document.querySelector('#tcp_keep_alive_seconds');
  const max_backlog = document.querySelector('#max_backlog');

  listening_address.textContent = config.listening_address;
  listening_port_http.textContent = config.listening_port_http;
  listening_port_https.textContent = config.listening_port_https;
  logging_level.textContent = config.logging_level;
  add_caching.textContent = config.add_caching;
  add_rate_limiting.textContent = config.add_rate_limiting;
  add_logging.textContent = config.add_logging;
  disable_default_body_limit.textContent = config.disable_default_body_limit;
  add_sql_injection_protection.textContent = config.add_sql_injection_protection;
  recv_buffer_size.textContent = config.recv_buffer_size;
  send_buffer_size.textContent = config.send_buffer_size;
  ip_ttl.textContent = config.ip_ttl;
  tcp_keep_alive_seconds.textContent = config.tcp_keep_alive_seconds;
  max_backlog.textContent = config.max_backlog;
}

//PAGE

let config;

async function get_configuration() {
  config = await invoke("get_configuration");
}

window.addEventListener("DOMContentLoaded", async () => {
  await load_configuration();
});

//MODAL

// Get elements
const modal = document.getElementById('settingsModal');
const closeButton = document.querySelector('.close-button');

const modalTitle = document.getElementById('modalTitle');
const modalInput = document.getElementById('modalInput');
const modalForm = document.getElementById('modalForm');

const successMessage = document.getElementById('success');
const dangerMessage = document.getElementById('danger');

// Close modal when 'x' is clicked
closeButton.addEventListener('click', () => {
  modal.style.display = 'none';
});

// Close modal when clicking outside the modal content
window.addEventListener('click', (event) => {
  if (event.target === modal) {
    modal.style.display = 'none';
  }
});

// Close modal with Escape key
document.addEventListener('keydown', (event) => {
  if (event.key === 'Escape') {
    modal.style.display = 'none';
  }
});

function openModal(event) {
    successMessage.style.display = 'none';
    dangerMessage.style.display = 'none';
    const td = event.currentTarget;
    const titleText = td.querySelector('b').innerText;
    const inputName = td.querySelector('span').id;
    const inputValue = td.querySelector('span').innerText;

    // Set modal title
    modalTitle.innerText = titleText;
  
    // Set input name and clear any previous value
    modalInput.name = inputName;
    modalInput.value = inputValue;
  
    // Display the modal
    modal.style.display = 'block';
}

const tds = document.querySelectorAll('#startup_settings td');

tds.forEach(td => {
    td.addEventListener('click', openModal);
});

modalForm.addEventListener('submit', async (e) => {
    e.preventDefault();

    var settingName = modalInput.name;
    var settingValue = modalInput.value;
    var success = await invoke('save_value', { settingName, settingValue });

    if (success) {
        successMessage.style.display = 'block';
    } else {
        dangerMessage.style.display = 'block';
    }

    await load_configuration();
});