use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 20a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2" ></ path > < path d = "M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14" ></ path > < path d = "M10 20h4" ></ path > < circle r = "2" cy = "20" cx = "16" ></ circle > < circle cx = "8" cy = "20" r = "2" ></ circle > < / > } } pub const LUCIDE_LUGGAGE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;