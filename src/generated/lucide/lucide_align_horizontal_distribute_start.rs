use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "5" width = "6" height = "14" x = "4" rx = "2" ></ rect > < rect width = "6" rx = "2" y = "7" x = "14" height = "10" ></ rect > < path d = "M4 2v20" ></ path > < path d = "M14 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_DISTRIBUTE_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24")] } ;