use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17h-3" ></ path > < path d = "M22 7h-5" ></ path > < path d = "M5 17H2" ></ path > < path d = "M7 7H2" ></ path > < rect rx = "2" width = "14" x = "5" height = "6" y = "14" ></ rect > < rect y = "4" rx = "2" height = "6" width = "10" x = "7" ></ rect > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;