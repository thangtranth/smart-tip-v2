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
  console.log("Tipped 1 NEAR");
  raiseNotification();
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
  raiseNotification();
};

const totalActivityPointEl = document.querySelector(
  "#get-total-activity-point"
);
totalActivityPointEl.onclick = async () => {
  const response = await contract.get_total_activity_point();
  console.log(response);
  totalActivityPointEl.textContent = "Get activity point: " + response;
};

const totalAmountAllocate = document.querySelector(
  "#get-total-amount-to-allocate"
);
totalAmountAllocate.onclick = async () => {
  const response = await contract.get_total_amount_to_allocate();
  console.log(response);
  totalAmountAllocate.textContent = "Get amount to allocate: " + response;
};

document.querySelector("#send-fund-contributors").onclick = async () => {
  const response = await contract.pay_all_contributors({});
  console.log("fund-sent");
  raiseNotification();
};

document.querySelector("#get-contributors-and-points").onclick = async () => {
  const response = await contract.get_contributors_and_point({});
  console.log(response);
  renderContributorsPoints(response);
};

const ulEl = document.getElementById("ul-el");
function renderContributorsPoints(response) {
  let listItems = "";
  for (let i = 0; i < response.length; i++) {
    console.log(i);
    listItems += `
    <li> ${response[i][0]}: ${response[i][1]} point </li>
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
    await contract.complete_activity({ task_id: parseInt(checkBox1.value) });
    console.log("Complete task 1");
    checkBox1.setAttribute("checked", "checked");
    raiseNotification();
  }
};

const checkBox2 = document.querySelector("#task2");
checkBox2.onclick = async () => {
  if (checkBox2.checked == true) {
    console.log("Click task 2");
    console.log(checkBox2.value);
    await contract.complete_activity({ task_id: parseInt(checkBox2.value) });
    console.log("Complete task 2");
    checkBox2.setAttribute("checked", "checked");
    raiseNotification();
  }
};

const checkBox3 = document.querySelector("#task3");
checkBox3.onclick = async () => {
  if (checkBox3.checked == true) {
    console.log("Click task 3");
    console.log(checkBox3.value);
    await contract.complete_activity({ task_id: parseInt(checkBox3.value) });
    console.log("Complete task 3");
    checkBox3.setAttribute("checked", "checked");
    raiseNotification();
  }
};

// const checkBox3 = document.querySelector("#task3");
// checkBox3.onclick = async () => {
//   raiseNotification();
// };

function raiseNotification() {
  document.querySelector("[data-behavior=notification]").style.display =
    "block";
  console.log("Notification");
  setTimeout(() => {
    document.querySelector("[data-behavior=notification]").style.display =
      "none";
  }, 11000);
}

window.nearInitPromise = initContract()
  .then(() => {
    if (window.walletConnection.isSignedIn()) signedInFlow();
    else signedOutFlow();
  })
  .catch(console.error);
