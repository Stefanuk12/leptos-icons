use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "8" cy = "8" r = "6" ></ circle > < path d = "M18.09 10.37A6 6 0 1 1 10.34 18" ></ path > < path d = "M7 6h1v4" ></ path > < path d = "m16.71 13.88.7.71-2.82 2.82" ></ path > < / > } } pub const LUCIDE_COINS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;