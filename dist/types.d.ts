interface SorobanSimulationResult {
    auth: any[];
    retval: string;
}
export interface TransactionSimulationPayload {
    method: string;
    tx: string;
    simulationResult: SorobanSimulationResult;
    simulationTransactionData: string;
}
export {};
//# sourceMappingURL=types.d.ts.map