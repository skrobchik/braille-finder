Author: Robert Serrano Kobylyansky

Date: 2022-11-03

This small program was writen to design an actuator mechanism for a
8-dot braille cell. There are two actuators, one for each column
of dots. Each column has 4 dots, that is the `pattern_len`. The actuator
works by moving a rail that has is engraved in such a way so that
it contains all the possible combinations of dots. The rail is moved
by a stepper motor to the desired position, resulting in the corresponding
dots being raised.
```
    pattern: 0b0101      0      1       0      1
                               NN             NN    <- pins (dots)
                               NN             NN
                        NN     NN      NN     NN
       /----\  /----\   NN   /----\    NN   /----\  /----\  
______/      \/      \__NN__/      \_ _NN__/      \/      \_______  <- rail [0b011 0101 10]
```
The goal is to find the shortest rail which can be used to represent all
the dot patterns. `rayon` crate is used to parallelize the search, and speed things up.
