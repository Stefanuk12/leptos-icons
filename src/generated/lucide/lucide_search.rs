use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" cy = "11" r = "8" ></ circle > < path d = "m21 21-4.3-4.3" ></ path > < / > } } pub const LUCIDE_SEARCH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;