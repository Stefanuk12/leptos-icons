use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 18H5a3 3 0 0 1-3-3v-1" ></ path > < path d = "M14 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" ></ path > < path d = "M20 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" ></ path > < path d = "m7 21 3-3-3-3" ></ path > < rect y = "14" x = "14" rx = "2" height = "8" width = "8" ></ rect > < rect rx = "2" width = "8" x = "2" y = "2" height = "8" ></ rect > < / > } } pub const LUCIDE_COMBINE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;