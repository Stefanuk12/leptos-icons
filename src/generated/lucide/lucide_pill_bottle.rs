use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 11h-4a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h4" ></ path > < path d = "M6 7v13a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7" ></ path > < rect x = "4" width = "16" y = "2" height = "5" rx = "1" ></ rect > < / > } } pub const LUCIDE_PILL_BOTTLE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;