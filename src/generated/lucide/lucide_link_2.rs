use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 17H7A5 5 0 0 1 7 7h2" ></ path > < path d = "M15 7h2a5 5 0 1 1 0 10h-2" ></ path > < line x2 = "16" y1 = "12" y2 = "12" x1 = "8" ></ line > < / > } } pub const LUCIDE_LINK_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;