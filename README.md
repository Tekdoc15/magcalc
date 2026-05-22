Saturation Current Calcuator

Why is this a thing?\
I needed to check the saturation current values of a bunch of different magnetic cores for a project. After doing a bunch by hand I decided to just build this to speed things up.

How to use it:\
The script takes either command line arguments or will prompt you for information.
If you are giving arguments they currently must be given in the following order:
B_sat l_e N mu_r

Note:\
B_sat must be in Tesla\
l_e must be in meters\
N is the number of turns of the conductor\
mu_r is dimensionless


Todo:
Add a way to have it also calculate the saturation current when at lower permeability than optimal
rework the arguments so that they use flags instead of position
