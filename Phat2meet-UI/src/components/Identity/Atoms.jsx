import { web3Enable, web3Accounts } from '@polkadot/extension-dapp'

import { atom } from 'jotai'

import {
  rpcApiInstanceAtom,
} from '../Atoms/FoundationBase'

export const extensionEnabledAtom = atom(false)

extensionEnabledAtom.onMount = (set) => {
  (async () => {
    try {
      const injected = await web3Enable('phat2meet')
      console.log('injected', injected)
      if (injected.length > 0) {
        set(true)
      }
    } catch (error) {
      console.error(error)
    }
  })()
}

export const accountsAtom = atom(async (get) => {
  const enabled = get(extensionEnabledAtom)
  if (enabled) {
    try {
      const allAccounts = await web3Accounts();

      return allAccounts
    } catch (err) {
      console.error('[accountsAtom] load keyring failed with: ', err)
    }
  }
  return []
})

export const currentAccountAtom = atom(undefined);

export const currentProfileAtom = atom(get => {
  const currentAccount = get(currentAccountAtom)

  if (!currentAccount) {
    return {
      connected: false,
    }
  }
  return {
    ...currentAccount,
    connected: true,
  }
})

export const balanceAtom = atom(async (get) => {
  const api = get(rpcApiInstanceAtom)
  const selected = get(currentAccountAtom)
  if (!api || !selected) {
    return 0
  }
  const account = await api.query.system.account(selected.address)
  const value = parseInt((BigInt(account.data.free.toString()) / BigInt(100000000)).toString(), 10) / 10000
  return value
})

// contract ID on phat-cb (contract address on polkadot.js.org/apps)
export const contractIdAtom = "0xfcfe1813af28dda3933cba418ea45acd6bd7188d5b1a0108e83ec1d14fa8f290"
