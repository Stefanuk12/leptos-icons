use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 4H9a3 3 0 0 0-2.83 4" ></ path > < path d = "M14 12a4 4 0 0 1 0 8H6" ></ path > < line x1 = "4" y2 = "12" y1 = "12" x2 = "20" ></ line > < / > } } pub const LUCIDE_STRIKETHROUGH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round")] } ;