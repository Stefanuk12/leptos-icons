use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" x = "2" y = "4" rx = "2" width = "20" ></ rect > < rect height = "6" width = "20" rx = "2" y = "14" x = "2" ></ rect > < / > } } pub const LUCIDE_STRETCH_HORIZONTAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;