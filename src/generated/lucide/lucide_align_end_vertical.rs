use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" y = "4" height = "6" rx = "2" width = "16" ></ rect > < rect rx = "2" width = "9" x = "9" y = "14" height = "6" ></ rect > < path d = "M22 22V2" ></ path > < / > } } pub const LUCIDE_ALIGN_END_VERTICAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2")] } ;