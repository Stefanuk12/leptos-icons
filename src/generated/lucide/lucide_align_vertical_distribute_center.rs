use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17h-3" ></ path > < path d = "M22 7h-5" ></ path > < path d = "M5 17H2" ></ path > < path d = "M7 7H2" ></ path > < rect height = "6" y = "14" width = "14" rx = "2" x = "5" ></ rect > < rect width = "10" rx = "2" height = "6" x = "7" y = "4" ></ rect > < / > } } pub const LUCIDE_ALIGN_VERTICAL_DISTRIBUTE_CENTER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24")] } ;