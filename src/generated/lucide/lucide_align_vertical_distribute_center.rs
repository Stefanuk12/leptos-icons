use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "5" width = "14" height = "6" y = "14" rx = "2" ></ rect > < rect width = "10" rx = "2" y = "4" x = "7" height = "6" ></ rect > < path d = "M22 7h-5" ></ path > < path d = "M7 7H1" ></ path > < path d = "M22 17h-3" ></ path > < path d = "M5 17H2" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24")] } ;