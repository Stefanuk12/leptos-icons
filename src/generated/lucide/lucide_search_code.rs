use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m13 13.5 2-2.5-2-2.5" ></ path > < path d = "m21 21-4.3-4.3" ></ path > < path d = "M9 8.5 7 11l2 2.5" ></ path > < circle cx = "11" r = "8" cy = "11" ></ circle > < / > } } pub const LUCIDE_SEARCH_CODE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;