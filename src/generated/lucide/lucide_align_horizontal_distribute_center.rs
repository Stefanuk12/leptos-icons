use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "6" height = "14" x = "4" y = "5" ></ rect > < rect rx = "2" x = "14" height = "10" width = "6" y = "7" ></ rect > < path d = "M17 22v-5" ></ path > < path d = "M17 7V2" ></ path > < path d = "M7 22v-3" ></ path > < path d = "M7 5V2" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;