use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 17H7A5 5 0 0 1 7 7" ></ path > < path d = "M15 7h2a5 5 0 0 1 4 8" ></ path > < line x1 = "8" x2 = "12" y1 = "12" y2 = "12" ></ line > < line y2 = "22" x2 = "22" x1 = "2" y1 = "2" ></ line > < / > } } pub const LUCIDE_LINK_2_OFF : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24")] } ;