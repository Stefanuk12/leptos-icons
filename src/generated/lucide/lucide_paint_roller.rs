use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" rx = "2" height = "6" width = "16" y = "2" ></ rect > < path d = "M10 16v-2a2 2 0 0 1 2-2h8a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2" ></ path > < rect x = "8" height = "6" rx = "1" y = "16" width = "4" ></ rect > < / > } } pub const LUCIDE_PAINT_ROLLER : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;