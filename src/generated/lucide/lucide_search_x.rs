use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m13.5 8.5-5 5" ></ path > < path d = "m8.5 8.5 5 5" ></ path > < circle cx = "11" r = "8" cy = "11" ></ circle > < path d = "m21 21-4.3-4.3" ></ path > < / > } } pub const LUCIDE_SEARCH_X : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;