use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" rx = "2" width = "8" height = "8" y = "2" ></ rect > < path d = "M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" ></ path > < path d = "M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" ></ path > < path d = "M10 18H5c-1.7 0-3-1.3-3-3v-1" ></ path > < polyline points = "7 21 10 18 7 15" ></ polyline > < rect y = "14" x = "14" width = "8" rx = "2" height = "8" ></ rect > < / > } } pub const LUCIDE_COMBINE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;