use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "8" height = "8" y = "2" rx = "2" ></ rect > < path d = "M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" ></ path > < path d = "M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" ></ path > < path d = "M10 18H5c-1.7 0-3-1.3-3-3v-1" ></ path > < polyline points = "7 21 10 18 7 15" ></ polyline > < rect height = "8" width = "8" x = "14" y = "14" rx = "2" ></ rect > < / > } } pub const LUCIDE_COMBINE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;