use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m18 8 4 4-4 4" ></ path > < path d = "m6 8-4 4 4 4" ></ path > < path d = "M8 12h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M16 12h.01" ></ path > < / > } } pub const LUCIDE_CHEVRONS_LEFT_RIGHT_ELLIPSIS : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;