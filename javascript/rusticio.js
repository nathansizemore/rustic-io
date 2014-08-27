// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.

function rustic_io(ip, port) {
    this.eventList = [];
    this.socket = null;
    this.initWebsocket(ip, port);
}

rustic_io.prototype.initWebsocket = function(ip, port) {
    this.socket = new WebSocket('ws://' + ip + ':' + port);
    this.socket.onopen = function(event) {
        console.log('Connection established');
    }

    var instance = this;
    this.socket.onmessage = function(event) {
        console.log('Message received');
        var msg = JSON.parse(event.data);
        msg.data = JSON.parse(msg.data);
        for (var i = 0; i < instance.eventList.length; i++) {
            if (instance.eventList[i].event === msg.event) {
                instance.eventList[i].callback(instance.socket, msg.data);
            }
        }
    }
};

rustic_io.prototype.on = function(event, callback) {
    this.eventList.push({
        event: event,
        callback: callback
    });
};

rustic_io.prototype.send = function(event, data) {
    var jsonMessage = {
        event: event,
        data: data
    };
    this.socket.send(JSON.stringify(jsonMessage));
};


