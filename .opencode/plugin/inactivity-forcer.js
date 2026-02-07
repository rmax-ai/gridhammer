// .opencode/plugin/inactivity-forcer.js

const RECENT_ABORT_WINDOW_MS = 60_000;

export const InactivityForcerPlugin = async ({ project, client, $ }) => {
  const recentAbortBySessionId = new Map();
  return {
    event: async ({ event }) => {
      switch (event.type) {
        case 'session.idle': {
          const sessionID = event.data?.sessionID || event.properties?.sessionID;
          if (!sessionID) {
            console.warn('No sessionID in idle event');
            return;
          }

          const lastAbortAt = recentAbortBySessionId.get(sessionID);
          if (
            typeof lastAbortAt === 'number' &&
            Date.now() - lastAbortAt < RECENT_ABORT_WINDOW_MS
          ) {
            return;
          }

           console.log(`Session ${sessionID} is idle, forcing exit...`);

           try {
             const session = await client.session.get({
               path: {
                 id: sessionID,
               },
             });
             if (session.interactive) {
               console.log('Session is interactive, not aborting.');
               return;
             }
             // Abort the session to stop inactivity
             recentAbortBySessionId.set(sessionID, Date.now());
             await client.session.abort({
               path: {
                 id: sessionID,
               },
             });
             console.log('Session aborted successfully.');
           } catch (error) {
             console.error('Failed to abort session:', error);
             recentAbortBySessionId.delete(sessionID);
           }
          break;
        }
      }
    }
  };
};
