use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m13 13.5 2-2.5-2-2.5" ></ path > < path d = "m21 21-4.3-4.3" ></ path > < path d = "M9 8.5 7 11l2 2.5" ></ path > < circle r = "8" cx = "11" cy = "11" ></ circle > < / > } } pub const LUCIDE_SEARCH_CODE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;