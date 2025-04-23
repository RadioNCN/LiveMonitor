use eframe::egui;
use eframe::egui::Window;

pub(crate) fn new(ctx:&egui::Context){
    Window::new("Guide").show(ctx, |ui| {
        ui.label("Python");
        ui.code("
import socket
import time
import numpy as np

TCP_IP = '127.0.0.1'
TCP_PORT = 7800

sender = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sender.connect((TCP_IP, TCP_PORT))
MESSAGE = 'Sinus\\n'
sender.send(bytes(MESSAGE, encoding='utf8'))
time.sleep(1)

t=0
while True
    MESSAGE = '{x}\\n{y}\\n'.format(x=t, y=np.sin(t))
    sender.send(bytes(MESSAGE, encoding='utf8'))
    time.sleep(.005)
    t+=0.005"
        );
    });
}