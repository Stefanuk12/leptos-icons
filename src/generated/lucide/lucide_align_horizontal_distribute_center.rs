use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" x = "4" height = "14" rx = "2" y = "5" ></ rect > < rect rx = "2" y = "7" height = "10" x = "14" width = "6" ></ rect > < path d = "M17 22v-5" ></ path > < path d = "M17 7V2" ></ path > < path d = "M7 22v-3" ></ path > < path d = "M7 5V2" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;