use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "8" height = "4" y = "2" ry = "1" width = "8" rx = "1" ></ rect > < path d = "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" ></ path > < path d = "M9 14h6" ></ path > < / > } } pub const LUCIDE_CLIPBOARD_MINUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;