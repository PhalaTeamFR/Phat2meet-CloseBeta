import { useEffect } from 'react'
import { atom, useAtom, useSetAtom } from 'jotai'


import { ApiPromise, WsProvider } from '@polkadot/api'

export const rpcEndpointAtom = atom('')

export const rpcEndpointErrorAtom = atom('')

export const rpcApiInstanceAtom = atom(ApiPromise, null)

export const createApiInstance = (endpointUrl) => {
  const wsProvider = new WsProvider(endpointUrl);
  const api = new ApiPromise({
    provider: wsProvider
  });

  return [wsProvider, api]
}

export const rpcApiStatusAtomProvider = atom('disconnected')

export const useConnectApi = () => {
  const [endpointUrl, setEndpointUrl] = useAtom(rpcEndpointAtom)
  const setStatus = useSetAtom(rpcApiStatusAtomProvider)
  const setApiInstance = useSetAtom(rpcApiInstanceAtom)
  const setError = useSetAtom(rpcEndpointErrorAtom)

  useEffect(() => {
    setError('')
    if (!endpointUrl) {
      setStatus('disconnected Fondation module')
      setApiInstance(null)
    } else {
      setStatus('connecting Fondation module')

      const fn = async () => {
        const [ws, api] = createApiInstance(endpointUrl)

        api.on('connected', async () => {
          await api.isReady
          setStatus('connected')
        })

        api.on('disconnected', () => {
          setStatus((prev) => prev === 'error' ? prev : 'disconnected')
          setEndpointUrl('')
        })

        api.on('ready', () => console.log(new Date(), 'API ready'))

        const onError = (err) => {
          console.error(new Date(), 'api error', err)
          setStatus('error')
          setError(`RPC Error`)
          setApiInstance(null)
          setEndpointUrl('')
          api.off('error', onError)
          try {
            api.disconnect()
            ws.disconnect()
          } catch (err1) {
            console.error('hey err1', err1)
          }
        }
        api.on('error', onError)

        setTimeout(() => {
          setStatus(prev => {
            if (prev !== 'connected') {
              setApiInstance(null)
              setEndpointUrl('')
              setError('RPC Endpoint is unreachable')
              return 'error'
            }
            return prev
          })
        }, 10000)

        await api.isReady
        setApiInstance(api)
      }

      try {
        fn()
      } catch (err) {
        console.error('error', err)
      }
    }
  }, [endpointUrl, setEndpointUrl, setStatus, setApiInstance, setError])
}