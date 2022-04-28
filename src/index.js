import "regenerator-runtime/runtime";

import { initContract, login, logout, toYocto } from "./utils";

import getConfig from "./config";
import { async } from "regenerator-runtime/runtime";
const { networkId } = getConfig(process.env.NODE_ENV || "development");
const gas = 100_000_000_000_000;

let currentGreeting;

function signedOutFlow() {
  document.querySelector("#signed-out-flow").style.display = "block";
}

function signedInFlow() {
  document.querySelector("#signed-in-flow").style.display = "block";
}
document.querySelector("#sign-in-button").onclick = login;
document.querySelector("#sign-out-button").onclick = logout;
document.querySelector("#tip-button").onclick = async () => {
  console.log("click tip button");
  try {
    await window.contract.tip({}, gas, toYocto("1"));
  } catch (e) {
    alert(
      "Something went wrong! " +
        "Maybe you need to sign out and back in? " +
        "Check your browser console for more info."
    );
    throw e;
  }
};

document.querySelector("#create-smarttip").onclick = async () => {
  try {
    await window.contract.new({ member_list: [window.accountId] });
  } catch (e) {
    alert(
      "Something went wrong! " +
        "Maybe you need to sign out and back in? " +
        "Check your browser console for more info."
    );
    throw e;
  }
  console.log("Initiate contract");
};

document.querySelector("#get-total-activity-point").onclick = async () => {
  const response = await contract.get_total_activity_point();
  console.log(response);
};

document.querySelector("#get-total-amount-to-allocate").onclick = async () => {
  const response = await contract.get_total_amount_to_allocate();
  console.log(response);
};

document.querySelector("#send-fund-contributors").onclick = async () => {
  const response = await contract.pay_all_contributors({});
  console.log("fund-sent");
};

let contributorsPoints = [];
document.querySelector("#get-contributors-and-points").onclick = async () => {
  const response = await contract.get_contributors_and_point({});
  contributorsPoints.push(response);
  // console.log(contributorsPoints);
  renderContributorsPoints(contributorsPoints);
};

const ulEl = document.getElementById("ul-el");
function renderContributorsPoints(response) {
  let listItems = "";
  for (let i = 0; i < response[0].length; i++) {
    console.log(i);
    listItems += `
    <li> ${response[0][i][0]}: ${response[0][i][1]} point </li>
    `;
  }
  console.log(listItems);
  ulEl.innerHTML = listItems;
}

// Complete task
const checkBox1 = document.querySelector("#task1");
checkBox1.onclick = async () => {
  if (checkBox1.checked == true) {
    console.log("Click task 1");
    console.log(checkBox1.value);
    await contract.complete_activitiy({ task_id: parseInt(checkBox1.value) });
  }
};

const checkBox2 = document.querySelector("#task2");
checkBox2.onclick = async () => {
  if (checkBox2.checked == true) {
    console.log("Click task 2");
    console.log(checkBox2.value);
  }
};

const checkBox3 = document.querySelector("#task3");
checkBox3.onclick = async () => {
  if (checkBox3.checked == true) {
    console.log("Click task 3");
    console.log(checkBox3.value);
  }
};
// document.querySelector("#task1").onclick = async () => {
//   console.log("Click task 1");
// };
// document.querySelector("#task1").onclick = async () => {
//   console.log("Click task 1");
// };
// async function tip() {
//   await window.contract.tip({}, 0, 1_000_000_000_000_000_000_000_000);
// }

// async function get_total_amount_to_allocate() {
//   const response = await window.contract.get_total_amount_to_allocate();
//   console.log(response);
// }

// `nearInitPromise` gets called on page load
window.nearInitPromise = initContract()
  .then(() => {
    if (window.walletConnection.isSignedIn()) signedInFlow();
    else signedOutFlow();
  })
  .catch(console.error);
