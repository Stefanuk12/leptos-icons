use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "16" width = "6" y = "16" height = "6" rx = "1" ></ rect > < rect width = "6" y = "16" rx = "1" height = "6" x = "2" ></ rect > < rect x = "9" width = "6" y = "2" rx = "1" height = "6" ></ rect > < path d = "M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3" ></ path > < path d = "M12 12V8" ></ path > < / > } } pub const LUCIDE_NETWORK : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;