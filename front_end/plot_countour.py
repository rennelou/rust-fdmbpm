import h5py
import numpy as np
import matplotlib.pyplot as plt

origin = 'lower'

filename = "slab3d.h5"
lines = 50
with h5py.File(filename, "r") as f:

    for key in f.keys():
        print("Key %s" % key)

    group = f['dir']
    print("Group: %s" % group)
    
    [zdelta, ydelta, xdelta] = group['deltas'][()]
    data = group['intensity'][()]

    xdepht = data[0][0].size
    ydepht = data[0].size / xdepht
    zdepht = data.size / (ydepht * xdepht)

    y = np.arange(0., ydepht * ydelta, ydelta)
    x = np.arange(0., xdepht * xdelta, xdelta)
    X, Y = np.meshgrid(x, y)

    zstep = zdepht / 4
    fig1, axs = plt.subplots(2, 2, constrained_layout=True)
    for i, ax in enumerate(axs.ravel()):
        index = int(i * zstep)
        Z = data[index]
        cs = ax.contourf(X, Y, Z, 10, cmap=plt.cm.bone, origin=origin)
        cs1 =  ax.contour(cs, levels=cs.levels[::2], colors='r', origin=origin)
        #cbar = fig1.colorbar(cs)  #barra lateral de intensidade
        #cbar.ax.set_ylabel('verbosity coefficient')
        # Add the contour line levels to the colorbar
        #cbar.add_lines(cs1)

    plt.title('Intensidade')
    plt.show()