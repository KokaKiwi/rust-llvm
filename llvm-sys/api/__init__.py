from bindgen.utils import import_all
from bindgen.gen import utils as gen_utils
import_all(__name__, __file__)

from .link import write_link
from .ns import root
