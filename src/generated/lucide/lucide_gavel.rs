use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m14.5 12.5-8 8a2.119 2.119 0 1 1-3-3l8-8" ></ path > < path d = "m16 16 6-6" ></ path > < path d = "m8 8 6-6" ></ path > < path d = "m9 7 8 8" ></ path > < path d = "m21 11-8-8" ></ path > < / > } } pub const LUCIDE_GAVEL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;