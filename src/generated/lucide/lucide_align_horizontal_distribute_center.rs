use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" y = "5" width = "6" rx = "2" x = "4" ></ rect > < rect x = "14" height = "10" width = "6" y = "7" rx = "2" ></ rect > < path d = "M17 22v-5" ></ path > < path d = "M17 7V2" ></ path > < path d = "M7 22v-3" ></ path > < path d = "M7 5V2" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;