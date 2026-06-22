import {
    Logger, logger,
    LoggingDebugSession,
    InitializedEvent, TerminatedEvent, StoppedEvent, OutputEvent,
    Response,
    Scope, Handles, StackFrame, Source,
    Thread,
} from '@vscode/debugadapter';
import { DebugProtocol } from '@vscode/debugprotocol';
import * as fs from 'fs';
import * as path from 'path';
import { spawn, ChildProcess } from 'child_process';

type DebugEvent = {
    event: string;
    reason?: string;
    file?: string;
    line?: number;
    depth?: number;
    exitCode?: number;
    text?: string;
    message?: string;
    stackFrames?: { name: string; file: string; line: number }[];
    variables?: { name: string; value: string; mutable: boolean }[];
};

interface ILaunchRequestArguments extends DebugProtocol.LaunchRequestArguments {
    program: string;
    stopOnEntry?: boolean;
    blPath?: string;
}

export class BackLangDebugSession extends LoggingDebugSession {
    private programFile: string = '';
    private blProcess: ChildProcess | null = null;
    private breakpoints: Map<string, number[]> = new Map();
    private variableHandles = new Handles<string>();
    private pending: Array<{ resolve: (val: string) => void; reject: (err: Error) => void }> = [];
    private pausedDepth: number = 0;
    private pausedLine: number = 0;
    private pausedFile: string = '';
    private debugStarted: boolean = false;

    public constructor() {
        super('backlang-debug');
        this.setDebuggerColumnsStartAt1(true);
        this.setDebuggerLinesStartAt1(true);
    }

    protected initializeRequest(response: DebugProtocol.InitializeResponse): void {
        response.body = response.body || {};
        response.body.supportsConfigurationDoneRequest = true;
        response.body.supportsEvaluateForHovers = false;
        response.body.supportsRestartFrame = false;
        response.body.supportsExceptionFilterOptions = false;
        response.body.supportTerminateDebuggee = true;
        response.body.supportsFunctionBreakpoints = false;
        response.body.supportsDelayedStackTraceLoading = false;
        response.body.supportsLogPoints = false;
        this.sendResponse(response);
        this.sendEvent(new InitializedEvent());
    }

    protected async launchRequest(response: DebugProtocol.LaunchResponse, args: ILaunchRequestArguments): Promise<void> {
        logger.setup(Logger.LogLevel.Verbose, false);
        this.programFile = args.program;

        if (!fs.existsSync(this.programFile)) {
            this.sendErrorResponse(response, 1001, `Cannot find program "${this.programFile}"`);
            return;
        }

        const blPath = args.blPath || this.findBlBinary();
        if (!blPath) {
            this.sendErrorResponse(response, 1002, 'BackLang binary not found');
            return;
        }

        this.sendResponse(response);

        // Spawn with --debug flag
        this.blProcess = spawn(blPath, ['--debug', this.programFile], {
            cwd: path.dirname(this.programFile),
            env: { ...process.env },
            stdio: ['pipe', 'pipe', 'pipe'],
        });

        // Handle stderr — debug protocol events (one JSON per line)
        let buffer = '';
        this.blProcess.stderr?.on('data', (data: Buffer) => {
            buffer += data.toString();
            const lines = buffer.split('\n');
            buffer = lines.pop() || '';
            for (const line of lines) {
                const trimmed = line.trim();
                if (!trimmed) continue;
                try {
                    const evt: DebugEvent = JSON.parse(trimmed);
                    this.handleDebugEvent(evt);
                } catch {
                    this.sendEvent(new OutputEvent(`[debug] ${trimmed}\n`, 'stderr'));
                }
            }
        });

        // Handle stdout — program output
        this.blProcess.stdout?.on('data', (data: Buffer) => {
            this.sendEvent(new OutputEvent(data.toString(), 'stdout'));
        });

        this.blProcess.on('close', (code) => {
            this.sendEvent(new TerminatedEvent());
        });

        // Send initial breakpoints
        for (const [file, lines] of this.breakpoints) {
            this.sendToBl(`setBreakpoints ${file} ${lines.join(' ')}`);
        }

        // Stop on entry if requested
        if (args.stopOnEntry) {
            this.sendToBl(`setBreakpoints ${this.programFile} 0`);
        }

        this.debugStarted = true;
        this.sendToBl('start');
    }

    private handleDebugEvent(evt: DebugEvent) {
        switch (evt.event) {
            case 'paused':
                this.pausedFile = evt.file || '';
                this.pausedLine = evt.line || 0;
                this.pausedDepth = evt.depth || 0;
                this.sendEvent(new StoppedEvent(evt.reason || 'breakpoint', 1));
                break;
            case 'continued':
                // No event needed — execution resumed
                break;
            case 'terminated':
                this.sendEvent(new TerminatedEvent());
                break;
            case 'output':
                this.sendEvent(new OutputEvent(evt.text || '', 'console'));
                break;
            case 'stackTrace':
                // Handle inline stack trace if needed
                break;
            case 'variables':
                // Handle inline variables if needed
                break;
            case 'error':
                this.sendEvent(new OutputEvent(`[debug error] ${evt.message}\n`, 'stderr'));
                break;
            default:
                this.sendEvent(new OutputEvent(`[debug] ${JSON.stringify(evt)}\n`, 'console'));
        }
    }

    private sendToBl(cmd: string) {
        if (this.blProcess?.stdin?.writable) {
            this.blProcess.stdin.write(cmd + '\n');
        }
    }

    // ── DAP Handlers ────────────────────────────────

    protected setBreakPointsRequest(response: DebugProtocol.SetBreakpointsResponse, args: DebugProtocol.SetBreakpointsArguments): void {
        const clientLines = args.lines || [];
        const bps: DebugProtocol.Breakpoint[] = [];
        const filename = args.source?.path || '';

        if (filename) {
            this.breakpoints.set(filename, [...clientLines]);
            for (const l of clientLines) {
                bps.push({ verified: true, line: l } as DebugProtocol.Breakpoint);
            }
            // Forward to process if already started
            if (this.debugStarted) {
                this.sendToBl(`setBreakpoints ${filename} ${clientLines.join(' ')}`);
            }
        }

        response.body = { breakpoints: bps };
        this.sendResponse(response);
    }

    protected configurationDoneRequest(response: DebugProtocol.ConfigurationDoneResponse): void {
        this.sendResponse(response);
    }

    protected threadsRequest(response: DebugProtocol.ThreadsResponse): void {
        response.body = { threads: [new Thread(1, 'Main Thread')] };
        this.sendResponse(response);
    }

    protected stackTraceRequest(response: DebugProtocol.StackTraceResponse): void {
        const frame = new StackFrame(
            0,
            `Line ${this.pausedLine}`,
            new Source(path.basename(this.pausedFile), this.pausedFile),
            Math.max(1, this.pausedLine)
        );
        response.body = { stackFrames: [frame], totalFrames: 1 };
        this.sendResponse(response);
    }

    protected scopesRequest(response: DebugProtocol.ScopesResponse): void {
        response.body = {
            scopes: [
                new Scope('Locals', this.variableHandles.create('locals'), false),
            ]
        };
        this.sendResponse(response);
    }

    protected variablesRequest(response: DebugProtocol.VariablesResponse): void {
        response.body = { variables: [] };
        this.sendResponse(response);
    }

    protected continueRequest(response: DebugProtocol.ContinueResponse): void {
        this.sendToBl('continue');
        this.sendResponse(response);
    }

    protected nextRequest(response: DebugProtocol.NextResponse): void {
        this.sendToBl('next');
        this.sendResponse(response);
    }

    protected stepInRequest(response: DebugProtocol.StepInResponse): void {
        this.sendToBl('step');
        this.sendResponse(response);
    }

    protected stepOutRequest(response: DebugProtocol.StepOutResponse): void {
        this.sendToBl('finish');
        this.sendResponse(response);
    }

    protected pauseRequest(response: DebugProtocol.PauseResponse): void {
        // We can't actually pause mid-execution since Rust blocks on stdin
        // Pulse flag
        this.sendResponse(response);
    }

    protected terminateRequest(response: DebugProtocol.TerminateResponse): void {
        this.sendToBl('terminate');
        if (this.blProcess) this.blProcess.kill();
        this.sendResponse(response);
    }

    protected disconnectRequest(response: DebugProtocol.DisconnectResponse): void {
        if (this.blProcess) this.blProcess.kill();
        this.sendResponse(response);
    }

    protected evaluateRequest(response: DebugProtocol.EvaluateResponse): void {
        response.body = { result: 'N/A', variablesReference: 0, type: 'string' };
        this.sendResponse(response);
    }

    private findBlBinary(): string | null {
        for (const p of [
            '/home/surgeo/Projects/BackLang/target/release/bl',
            '/home/surgeo/Projects/BackLang/target/debug/bl',
        ]) if (fs.existsSync(p)) return p;
        return null;
    }
}

import { DebugSession } from '@vscode/debugadapter';
DebugSession.run(BackLangDebugSession);
