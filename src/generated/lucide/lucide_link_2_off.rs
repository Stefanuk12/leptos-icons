use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 17H7A5 5 0 0 1 7 7" ></ path > < path d = "M15 7h2a5 5 0 0 1 4 8" ></ path > < line y1 = "12" x1 = "8" y2 = "12" x2 = "12" ></ line > < line x2 = "22" x1 = "2" y2 = "22" y1 = "2" ></ line > < / > } } pub const LUCIDE_LINK_2_OFF : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;