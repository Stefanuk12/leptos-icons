use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 18H5a3 3 0 0 1-3-3v-1" ></ path > < path d = "M14 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" ></ path > < path d = "M20 2a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2" ></ path > < path d = "m7 21 3-3-3-3" ></ path > < rect width = "8" x = "14" y = "14" height = "8" rx = "2" ></ rect > < rect y = "2" height = "8" width = "8" rx = "2" x = "2" ></ rect > < / > } } pub const LUCIDE_COMBINE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;