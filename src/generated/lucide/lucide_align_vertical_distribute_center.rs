use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17h-3" ></ path > < path d = "M22 7h-5" ></ path > < path d = "M5 17H2" ></ path > < path d = "M7 7H2" ></ path > < rect height = "6" x = "5" rx = "2" y = "14" width = "14" ></ rect > < rect y = "4" x = "7" height = "6" rx = "2" width = "10" ></ rect > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;