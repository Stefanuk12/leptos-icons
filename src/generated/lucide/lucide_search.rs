use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cy = "11" cx = "11" ></ circle > < path d = "m21 21-4.3-4.3" ></ path > < / > } } pub const LUCIDE_SEARCH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;