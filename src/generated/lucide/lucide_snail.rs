use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 13a6 6 0 1 0 12 0 4 4 0 1 0-8 0 2 2 0 0 0 4 0" ></ path > < circle r = "8" cy = "13" cx = "10" ></ circle > < path d = "M2 21h12c4.4 0 8-3.6 8-8V7a2 2 0 1 0-4 0v6" ></ path > < path d = "M18 3 19.1 5.2" ></ path > < path d = "M22 3 20.9 5.2" ></ path > < / > } } pub const LUCIDE_SNAIL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;