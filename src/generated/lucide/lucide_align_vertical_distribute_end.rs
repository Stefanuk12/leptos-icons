use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" x = "5" y = "14" height = "6" rx = "2" ></ rect > < rect width = "10" x = "7" height = "6" y = "4" rx = "2" ></ rect > < path d = "M2 20h20" ></ path > < path d = "M2 10h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_END : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;