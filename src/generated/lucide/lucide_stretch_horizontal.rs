use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" height = "6" rx = "2" x = "2" width = "20" ></ rect > < rect height = "6" rx = "2" width = "20" x = "2" y = "14" ></ rect > < / > } } pub const LUCIDE_STRETCH_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;