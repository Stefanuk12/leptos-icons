use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" width = "8" height = "4" rx = "1" ry = "1" x = "8" ></ rect > < path d = "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" ></ path > < path d = "m15 11-6 6" ></ path > < path d = "m9 11 6 6" ></ path > < / > } } pub const LUCIDE_CLIPBOARD_X : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;