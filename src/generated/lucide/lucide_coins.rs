use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "8" r = "6" cx = "8" ></ circle > < path d = "M18.09 10.37A6 6 0 1 1 10.34 18" ></ path > < path d = "M7 6h1v4" ></ path > < path d = "m16.71 13.88.7.71-2.82 2.82" ></ path > < / > } } pub const LUCIDE_COINS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;