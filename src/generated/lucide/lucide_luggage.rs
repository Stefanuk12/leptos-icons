use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 20h0a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h0" ></ path > < path d = "M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14" ></ path > < path d = "M10 20h4" ></ path > < circle cx = "16" r = "2" cy = "20" ></ circle > < circle cx = "8" cy = "20" r = "2" ></ circle > < / > } } pub const LUCIDE_LUGGAGE : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;