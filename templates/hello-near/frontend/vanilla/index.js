import 'regenerator-runtime/runtime';
import * as Contract from './near-interface';
import * as Wallet from './near-wallet';

document.querySelector('form').onsubmit = doUserAction;
document.querySelector('#sign-in-button').onclick = Wallet.signIn;
document.querySelector('#sign-out-button').onclick = Wallet.signOut;

// nearInitPromise gets called on page load
window.nearInitPromise = Wallet.startUp()
                         .then(isSignedIn => flow(isSignedIn))
                         .catch(console.error)

function flow(isSignedIn){
  if(isSignedIn){
    signedInFlow()
  }else{
    signedOutFlow()
  }
  fetchGreeting();
}

// Take the new greeting and send it to the contract
async function doUserAction(event) {
  event.preventDefault();
  const { greeting } = event.target.elements;

  document.querySelector('#signed-in-flow main')
          .classList.add('please-wait');

  await Contract.setGreeting(greeting.value);

  // ===== Fetch the data from the blockchain =====
  await fetchGreeting();
  document.querySelector('#signed-in-flow main')
          .classList.remove('please-wait');
}

// Get greeting from the contract on chain
async function fetchGreeting() {
  const currentGreeting = await Contract.getGreeting();

  document.querySelectorAll('[data-behavior=greeting]').forEach(el => {
    el.innerText = currentGreeting;
    el.value = currentGreeting;
  });
}

// Display the signed-out-flow container
function signedOutFlow() {
  document.querySelector('#signed-in-flow').style.display = 'none';
  document.querySelector('#signed-out-flow').style.display = 'block';
}

// Displaying the signed in flow container and fill in account-specific data
function signedInFlow() {
  document.querySelector('#signed-out-flow').style.display = 'none';
  document.querySelector('#signed-in-flow').style.display = 'block';
  document.querySelectorAll('[data-behavior=account-id]').forEach(el => {
    el.innerText = window.accountId;
  });
}