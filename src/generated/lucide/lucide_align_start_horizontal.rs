use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "6" width = "6" height = "16" rx = "2" x = "4" ></ rect > < rect height = "9" width = "6" rx = "2" y = "6" x = "14" ></ rect > < path d = "M22 2H2" ></ path > < / > } } pub const LUCIDE_ALIGN_START_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;