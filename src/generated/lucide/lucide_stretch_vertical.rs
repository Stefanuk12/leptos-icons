use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" rx = "2" y = "2" width = "6" x = "4" ></ rect > < rect x = "14" rx = "2" height = "20" width = "6" y = "2" ></ rect > < / > } } pub const LUCIDE_STRETCH_VERTICAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;