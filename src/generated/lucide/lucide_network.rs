use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "16" width = "6" height = "6" y = "16" rx = "1" ></ rect > < rect rx = "1" y = "16" x = "2" height = "6" width = "6" ></ rect > < rect x = "9" y = "2" height = "6" rx = "1" width = "6" ></ rect > < path d = "M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3" ></ path > < path d = "M12 12V8" ></ path > < / > } } pub const LUCIDE_NETWORK : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;