use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9a3 3 0 1 0 0 6" ></ path > < path d = "M2 12h1" ></ path > < path d = "M14 21V3" ></ path > < path d = "M10 4V3" ></ path > < path d = "M10 21v-1" ></ path > < path d = "m3.64 18.36.7-.7" ></ path > < path d = "m4.34 6.34-.7-.7" ></ path > < path d = "M14 12h8" ></ path > < path d = "m17 4-3 3" ></ path > < path d = "m14 17 3 3" ></ path > < path d = "m21 15-3-3 3-3" ></ path > < / > } } pub const LUCIDE_SUN_SNOW : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;