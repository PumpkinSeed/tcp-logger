package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func main() {
	// Listen for incoming connections.
	l, err := net.Listen("tcp", ":8000")
	if err != nil {
		fmt.Println("Error listening:", err.Error())
		os.Exit(1)
	}
	// Close the listener when the application closes.
	defer l.Close()
	fmt.Println("Listening on " + ":8000")
	for {
		// Listen for an incoming connection.
		conn, err := l.Accept()
		if err != nil {
			fmt.Println("Error accepting: ", err.Error())
			os.Exit(1)
		}
		// Handle connections in a new goroutine.
		go handleConnection(conn)
	}
}

func handleConnection(c net.Conn) {
	fmt.Printf("Serving %s\n", c.RemoteAddr().String())

	var buf = make([]byte, 8000)
	_, err := bufio.NewReader(c).Read(buf)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(string(buf))

	c.Close()
}
