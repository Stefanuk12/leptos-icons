use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "8" rx = "1" width = "8" y = "2" height = "4" ></ rect > < path d = "M10.4 12.6a2 2 0 0 1 3 3L8 21l-4 1 1-4Z" ></ path > < path d = "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-5.5" ></ path > < path d = "M4 13.5V6a2 2 0 0 1 2-2h2" ></ path > < / > } } pub const LUCIDE_CLIPBOARD_PEN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;