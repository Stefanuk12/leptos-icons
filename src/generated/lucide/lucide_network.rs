use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "16" rx = "1" x = "16" width = "6" height = "6" ></ rect > < rect x = "2" y = "16" width = "6" rx = "1" height = "6" ></ rect > < rect y = "2" rx = "1" x = "9" width = "6" height = "6" ></ rect > < path d = "M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3" ></ path > < path d = "M12 12V8" ></ path > < / > } } pub const LUCIDE_NETWORK : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;