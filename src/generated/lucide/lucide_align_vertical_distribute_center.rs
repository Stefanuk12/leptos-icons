use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17h-3" ></ path > < path d = "M22 7h-5" ></ path > < path d = "M5 17H2" ></ path > < path d = "M7 7H2" ></ path > < rect x = "5" y = "14" height = "6" width = "14" rx = "2" ></ rect > < rect height = "6" rx = "2" width = "10" x = "7" y = "4" ></ rect > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;