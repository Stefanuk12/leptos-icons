use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 10a7.31 7.31 0 0 0 10 10Z" ></ path > < path d = "m9 15 3-3" ></ path > < path d = "M17 13a6 6 0 0 0-6-6" ></ path > < path d = "M21 13A10 10 0 0 0 11 3" ></ path > < / > } } pub const LUCIDE_SATELLITE_DISH : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;