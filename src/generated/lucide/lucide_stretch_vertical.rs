use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" width = "6" y = "2" rx = "2" x = "4" ></ rect > < rect width = "6" height = "20" rx = "2" x = "14" y = "2" ></ rect > < / > } } pub const LUCIDE_STRETCH_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;