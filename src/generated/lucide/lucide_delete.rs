use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 5H9l-7 7 7 7h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2Z" ></ path > < line x1 = "18" x2 = "12" y1 = "9" y2 = "15" ></ line > < line x2 = "18" x1 = "12" y1 = "9" y2 = "15" ></ line > < / > } } pub const LUCIDE_DELETE : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;