use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" x2 = "15" x1 = "3" y1 = "22" ></ line > < line y2 = "9" x1 = "4" x2 = "14" y1 = "9" ></ line > < path d = "M14 22V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v18" ></ path > < path d = "M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 2 2a2 2 0 0 0 2-2V9.83a2 2 0 0 0-.59-1.42L18 5" ></ path > < / > } } pub const LUCIDE_FUEL : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;