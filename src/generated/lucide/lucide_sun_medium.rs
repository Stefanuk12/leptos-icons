use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "4" ></ circle > < path d = "M12 3v1" ></ path > < path d = "M12 20v1" ></ path > < path d = "M3 12h1" ></ path > < path d = "M20 12h1" ></ path > < path d = "m18.364 5.636-.707.707" ></ path > < path d = "m6.343 17.657-.707.707" ></ path > < path d = "m5.636 5.636.707.707" ></ path > < path d = "m17.657 17.657.707.707" ></ path > < / > } } pub const LUCIDE_SUN_MEDIUM : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;