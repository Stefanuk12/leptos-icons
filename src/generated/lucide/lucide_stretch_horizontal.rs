use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" rx = "2" width = "20" height = "6" x = "2" ></ rect > < rect width = "20" x = "2" y = "14" rx = "2" height = "6" ></ rect > < / > } } pub const LUCIDE_STRETCH_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;