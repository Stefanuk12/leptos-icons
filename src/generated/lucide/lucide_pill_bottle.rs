use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 11h-4a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h4" ></ path > < path d = "M6 7v13a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7" ></ path > < rect width = "16" x = "4" rx = "1" y = "2" height = "5" ></ rect > < / > } } pub const LUCIDE_PILL_BOTTLE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;