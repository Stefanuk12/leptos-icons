use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 4H9a3 3 0 0 0-2.83 4" ></ path > < path d = "M14 12a4 4 0 0 1 0 8H6" ></ path > < line x2 = "20" x1 = "4" y1 = "12" y2 = "12" ></ line > < / > } } pub const LUCIDE_STRIKETHROUGH : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;