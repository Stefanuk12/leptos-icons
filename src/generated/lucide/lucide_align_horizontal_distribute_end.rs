use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "5" x = "4" width = "6" height = "14" rx = "2" ></ rect > < rect y = "7" width = "6" height = "10" rx = "2" x = "14" ></ rect > < path d = "M10 2v20" ></ path > < path d = "M20 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_DISTRIBUTE_END : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;