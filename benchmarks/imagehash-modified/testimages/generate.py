import numpy
from PIL import Image

for x in range(64):
    imarray = numpy.random.rand(250, 250, 3) * 255
    im = Image.fromarray(imarray.astype('uint8')).convert('RGBA')
    rgb_im = im.convert('RGB')
    rgb_im.save('{idx}.bmp'.format(idx=x))
