import fs from 'fs'
import path from 'path'

/**
 * Defines the configuration required for running integration tests.
 * This includes contract IDs, RPC URL, and the admin's secret key.
 */
export interface TestConfig {
  adminSecretKey: string
  rpcUrl: string
  authorityContractId: string
}

/**
 * Check if a Stellar account exists on the network
 */
export async function accountExists(publicKey: string): Promise<boolean> {
  try {
    const response = await fetch(`https://horizon-testnet.stellar.org/accounts/${publicKey}`)
    return response.ok
  } catch (error) {
    return false
  }
}

/**
 * Fund a Stellar account using Friendbot (testnet only)
 * Only funds if the account doesn't exist yet
 */
export async function fundAccountIfNeeded(publicKey: string): Promise<void> {
  const exists = await accountExists(publicKey)

  if (exists) {
    console.log(`Account ${publicKey} already exists, skipping funding`)
    return
  }

  try {
    console.log(`Funding new account: ${publicKey}`)
    const response = await fetch(`https://friendbot.stellar.org?addr=${encodeURIComponent(publicKey)}`)
    if (!response.ok) {
      console.warn(`Friendbot funding failed for ${publicKey}: ${response.statusText}`)
    } else {
      console.log(`Successfully funded account: ${publicKey}`)
    }
  } catch (error) {
    console.warn(`Error funding account ${publicKey}:`, error)
  }
}

/**
 * Load test configuration from deployments.json and environment
 */
export function loadTestConfig(): TestConfig {
  const deploymentsPath = path.join(__dirname, '..', 'deployments.json')

  try {
    // Load deployment data
    const deployments = JSON.parse(fs.readFileSync(deploymentsPath, 'utf8'))
    const testnetDeployments = deployments.testnet

    if (!testnetDeployments) {
      throw new Error('No testnet deployments found in deployments.json')
    }

    const authorityContractId = testnetDeployments.authority?.id

    if (!authorityContractId) {
      throw new Error('Authority contract ID not found in deployments.json')
    }

    /** MUST COME from .env file */
    const adminSecretKey = process.env.ADMIN_SECRET_KEY as string;
    const rpcUrl = 'https://soroban-testnet.stellar.org'

    return {
      adminSecretKey,
      rpcUrl,
      authorityContractId
    }
  } catch (error) {
    throw new Error(`Failed to load test configuration: ${error}`)
  }
}
