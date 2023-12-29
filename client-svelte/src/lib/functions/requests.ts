function getBaseServerPath(): string | null {
    return localStorage.getItem("base_server_path");
}

export async function putCreateGame(game_name: string, name: string) {
    const response: Response = await fetch(getBaseServerPath() + game_name, {
        method: "PUT",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({
            player: name
        })
    })
    return response;
}

export async function postJoinGame(game_name: string, name: string) {
    const request = await fetch(getBaseServerPath() + game_name, {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({
            player: name
        })
    })
    return request;
  }

export async function getGame(game_name: string) {
    const response: Response = await fetch(getBaseServerPath() + game_name, {
        method: "GET",
        headers: {"Content-Type": "application/json"},
    })
    return response;
}


export async function postAnswer(game_name: string, name: string, answer: string) {
    const response: Response = await fetch(getBaseServerPath() + game_name + "/answer", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({
            player: name,
            answer: answer,
        })
    })
    return response;
}


// export async function deletePlayerFromGame(game_name: string, name: string) {
//     const response: Response = await fetch(getBaseServerPath() + game_name + "/exit", {
//         method: "DELETE",
//         mode: "no-cors",
//         headers: {"Content-Type": "application/json"},
//         body: JSON.stringify({
//             player: name,
//         })
//     })
//     return response;
// }


export async function postGuess(game_name: string, name: string, guess: number) {
    const response: Response = await fetch(getBaseServerPath() + game_name + "/guess", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({
            player: name,
            guess: guess,
        }),
    })
    return response;
}

export async function getScore(game_name: string) {
    const response: Response = await fetch(getBaseServerPath() + game_name + "/score", {
        method: "GET",
        headers: {"Content-Type": "application/json"},
    })
    return response;
}

export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}