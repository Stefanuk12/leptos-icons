use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 11h-4a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h4" ></ path > < path d = "M6 7v13a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7" ></ path > < rect rx = "1" width = "16" height = "5" x = "4" y = "2" ></ rect > < / > } } pub const LUCIDE_PILL_BOTTLE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;