use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" y = "14" x = "5" rx = "2" width = "14" ></ rect > < rect x = "7" y = "4" height = "6" rx = "2" width = "10" ></ rect > < path d = "M2 14h20" ></ path > < path d = "M2 4h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;