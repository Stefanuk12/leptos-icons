use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" height = "14" y = "5" rx = "2" width = "6" ></ rect > < rect height = "10" width = "6" y = "7" rx = "2" x = "14" ></ rect > < path d = "M10 2v20" ></ path > < path d = "M20 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_DISTRIBUTE_END : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;