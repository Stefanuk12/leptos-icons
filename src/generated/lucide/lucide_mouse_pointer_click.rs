use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 4.1 12 6" ></ path > < path d = "m5.1 8-2.9-.8" ></ path > < path d = "m6 12-1.9 2" ></ path > < path d = "M7.2 2.2 8 5.1" ></ path > < path d = "M9.037 9.69a.498.498 0 0 1 .653-.653l11 4.5a.5.5 0 0 1-.074.949l-4.349 1.041a1 1 0 0 0-.74.739l-1.04 4.35a.5.5 0 0 1-.95.074z" ></ path > < / > } } pub const LUCIDE_MOUSE_POINTER_CLICK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24")] } ;