use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "6" y = "14" rx = "2" x = "5" ></ rect > < rect width = "10" y = "4" height = "6" rx = "2" x = "7" ></ rect > < path d = "M2 14h20" ></ path > < path d = "M2 4h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_START : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;