use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 18H5a3 3 0 0 1-3-3v-1" ></ path > < path d = "M14 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" ></ path > < path d = "M20 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" ></ path > < path d = "m7 21 3-3-3-3" ></ path > < rect width = "8" rx = "2" x = "14" height = "8" y = "14" ></ rect > < rect rx = "2" x = "2" y = "2" width = "8" height = "8" ></ rect > < / > } } pub const LUCIDE_COMBINE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;