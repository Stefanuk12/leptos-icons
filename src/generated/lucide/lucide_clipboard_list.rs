use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "8" height = "4" width = "8" y = "2" rx = "1" ry = "1" ></ rect > < path d = "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" ></ path > < path d = "M12 11h4" ></ path > < path d = "M12 16h4" ></ path > < path d = "M8 11h.01" ></ path > < path d = "M8 16h.01" ></ path > < / > } } pub const LUCIDE_CLIPBOARD_LIST : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;