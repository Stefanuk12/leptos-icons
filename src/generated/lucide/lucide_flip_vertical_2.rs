use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m17 3-5 5-5-5h10" ></ path > < path d = "m17 21-5-5-5 5h10" ></ path > < path d = "M4 12H2" ></ path > < path d = "M10 12H8" ></ path > < path d = "M16 12h-2" ></ path > < path d = "M22 12h-2" ></ path > < / > } } pub const LUCIDE_FLIP_VERTICAL_2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;