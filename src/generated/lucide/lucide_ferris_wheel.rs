use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "2" cy = "12" cx = "12" ></ circle > < path d = "M12 2v4" ></ path > < path d = "m6.8 15-3.5 2" ></ path > < path d = "m20.7 7-3.5 2" ></ path > < path d = "M6.8 9 3.3 7" ></ path > < path d = "m20.7 17-3.5-2" ></ path > < path d = "m9 22 3-8 3 8" ></ path > < path d = "M8 22h8" ></ path > < path d = "M18 18.7a9 9 0 1 0-12 0" ></ path > < / > } } pub const LUCIDE_FERRIS_WHEEL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;