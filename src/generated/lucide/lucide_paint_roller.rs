use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "16" y = "2" rx = "2" height = "6" ></ rect > < path d = "M10 16v-2a2 2 0 0 1 2-2h8a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2" ></ path > < rect width = "4" rx = "1" y = "16" height = "6" x = "8" ></ rect > < / > } } pub const LUCIDE_PAINT_ROLLER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;