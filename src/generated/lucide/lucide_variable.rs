use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 21s-4-3-4-9 4-9 4-9" ></ path > < path d = "M16 3s4 3 4 9-4 9-4 9" ></ path > < line x1 = "15" y2 = "15" x2 = "9" y1 = "9" ></ line > < line x2 = "15" x1 = "9" y2 = "15" y1 = "9" ></ line > < / > } } pub const LUCIDE_VARIABLE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;