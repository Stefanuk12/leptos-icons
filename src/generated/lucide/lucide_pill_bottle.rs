use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 11h-4a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h4" ></ path > < path d = "M6 7v13a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7" ></ path > < rect rx = "1" height = "5" width = "16" y = "2" x = "4" ></ rect > < / > } } pub const LUCIDE_PILL_BOTTLE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;