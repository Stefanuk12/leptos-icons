use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17h-3" ></ path > < path d = "M22 7h-5" ></ path > < path d = "M5 17H2" ></ path > < path d = "M7 7H2" ></ path > < rect height = "6" width = "14" x = "5" rx = "2" y = "14" ></ rect > < rect height = "6" rx = "2" x = "7" y = "4" width = "10" ></ rect > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;