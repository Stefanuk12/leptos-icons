use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "4" ></ circle > < path d = "M12 3v1" ></ path > < path d = "M12 20v1" ></ path > < path d = "M3 12h1" ></ path > < path d = "M20 12h1" ></ path > < path d = "m18.364 5.636-.707.707" ></ path > < path d = "m6.343 17.657-.707.707" ></ path > < path d = "m5.636 5.636.707.707" ></ path > < path d = "m17.657 17.657.707.707" ></ path > < / > } } pub const LUCIDE_SUN_MEDIUM : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;