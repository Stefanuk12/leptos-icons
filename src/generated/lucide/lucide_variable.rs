use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 21s-4-3-4-9 4-9 4-9" ></ path > < path d = "M16 3s4 3 4 9-4 9-4 9" ></ path > < line x1 = "15" x2 = "9" y1 = "9" y2 = "15" ></ line > < line x1 = "9" y1 = "9" y2 = "15" x2 = "15" ></ line > < / > } } pub const LUCIDE_VARIABLE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;