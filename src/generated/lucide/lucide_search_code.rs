use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 9-2 2 2 2" ></ path > < path d = "m13 13 2-2-2-2" ></ path > < circle cx = "11" r = "8" cy = "11" ></ circle > < path d = "m21 21-4.3-4.3" ></ path > < / > } } pub const LUCIDE_SEARCH_CODE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;