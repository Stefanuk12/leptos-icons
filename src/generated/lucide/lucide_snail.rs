use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 13a6 6 0 1 0 12 0 4 4 0 1 0-8 0 2 2 0 0 0 4 0" ></ path > < circle cx = "10" r = "8" cy = "13" ></ circle > < path d = "M2 21h12c4.4 0 8-3.6 8-8V7a2 2 0 1 0-4 0v6" ></ path > < path d = "M18 3 19.1 5.2" ></ path > < path d = "M22 3 20.9 5.2" ></ path > < / > } } pub const LUCIDE_SNAIL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;