use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" x = "5" y = "14" height = "6" rx = "2" ></ rect > < rect width = "10" y = "4" x = "7" rx = "2" height = "6" ></ rect > < path d = "M22 7h-5" ></ path > < path d = "M7 7H1" ></ path > < path d = "M22 17h-3" ></ path > < path d = "M5 17H2" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;