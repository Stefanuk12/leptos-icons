use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "5" rx = "2" width = "14" height = "6" y = "14" ></ rect > < rect width = "10" y = "4" rx = "2" x = "7" height = "6" ></ rect > < path d = "M22 7h-5" ></ path > < path d = "M7 7H1" ></ path > < path d = "M22 17h-3" ></ path > < path d = "M5 17H2" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;