use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 5H9l-7 7 7 7h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2Z" ></ path > < line x2 = "12" x1 = "18" y1 = "9" y2 = "15" ></ line > < line y1 = "9" x2 = "18" x1 = "12" y2 = "15" ></ line > < / > } } pub const LUCIDE_DELETE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;