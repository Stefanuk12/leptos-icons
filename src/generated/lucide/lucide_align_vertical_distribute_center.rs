use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" x = "5" rx = "2" y = "14" width = "14" ></ rect > < rect y = "4" x = "7" width = "10" rx = "2" height = "6" ></ rect > < path d = "M22 7h-5" ></ path > < path d = "M7 7H1" ></ path > < path d = "M22 17h-3" ></ path > < path d = "M5 17H2" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;