use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 20a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2" ></ path > < path d = "M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14" ></ path > < path d = "M10 20h4" ></ path > < circle cy = "20" cx = "16" r = "2" ></ circle > < circle r = "2" cy = "20" cx = "8" ></ circle > < / > } } pub const LUCIDE_LUGGAGE : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;