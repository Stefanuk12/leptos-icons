use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "16" height = "6" y = "2" rx = "2" ></ rect > < path d = "M10 16v-2a2 2 0 0 1 2-2h8a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2" ></ path > < rect y = "16" rx = "1" width = "4" x = "8" height = "6" ></ rect > < / > } } pub const LUCIDE_PAINT_ROLLER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24")] } ;