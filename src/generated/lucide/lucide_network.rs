use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "16" rx = "1" width = "6" y = "16" height = "6" ></ rect > < rect y = "16" x = "2" height = "6" width = "6" rx = "1" ></ rect > < rect y = "2" x = "9" height = "6" width = "6" rx = "1" ></ rect > < path d = "M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3" ></ path > < path d = "M12 12V8" ></ path > < / > } } pub const LUCIDE_NETWORK : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;