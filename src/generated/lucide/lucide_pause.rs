use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" x = "14" rx = "1" y = "4" width = "4" ></ rect > < rect y = "4" width = "4" x = "6" rx = "1" height = "16" ></ rect > < / > } } pub const LUCIDE_PAUSE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;