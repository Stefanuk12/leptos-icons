use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" x = "2" width = "20" rx = "2" height = "6" ></ rect > < rect y = "14" x = "2" rx = "2" width = "20" height = "6" ></ rect > < / > } } pub const LUCIDE_STRETCH_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;