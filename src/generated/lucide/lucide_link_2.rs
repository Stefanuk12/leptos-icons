use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 17H7A5 5 0 0 1 7 7h2" ></ path > < path d = "M15 7h2a5 5 0 1 1 0 10h-2" ></ path > < line x1 = "8" x2 = "16" y2 = "12" y1 = "12" ></ line > < / > } } pub const LUCIDE_LINK_2 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;