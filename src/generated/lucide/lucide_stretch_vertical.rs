use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "20" width = "6" x = "4" rx = "2" y = "2" ></ rect > < rect rx = "2" y = "2" width = "6" x = "14" height = "20" ></ rect > < / > } } pub const LUCIDE_STRETCH_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24")] } ;