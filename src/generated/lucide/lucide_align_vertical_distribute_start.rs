use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "6" width = "14" y = "14" x = "5" ></ rect > < rect height = "6" rx = "2" y = "4" width = "10" x = "7" ></ rect > < path d = "M2 14h20" ></ path > < path d = "M2 4h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_START : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;