use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" y = "4" x = "14" height = "16" width = "4" ></ rect > < rect height = "16" x = "6" width = "4" rx = "1" y = "4" ></ rect > < / > } } pub const LUCIDE_PAUSE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;