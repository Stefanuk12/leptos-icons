use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "16" height = "6" x = "16" rx = "1" width = "6" ></ rect > < rect x = "2" y = "16" width = "6" height = "6" rx = "1" ></ rect > < rect y = "2" width = "6" x = "9" height = "6" rx = "1" ></ rect > < path d = "M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3" ></ path > < path d = "M12 12V8" ></ path > < / > } } pub const LUCIDE_NETWORK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;