use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" x = "14" rx = "1" y = "4" width = "4" ></ rect > < rect x = "6" y = "4" rx = "1" height = "16" width = "4" ></ rect > < / > } } pub const LUCIDE_PAUSE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none")] } ;