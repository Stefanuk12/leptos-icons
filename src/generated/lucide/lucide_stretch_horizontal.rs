use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" width = "20" rx = "2" x = "2" height = "6" ></ rect > < rect width = "20" rx = "2" x = "2" y = "14" height = "6" ></ rect > < / > } } pub const LUCIDE_STRETCH_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;